use crate::analyze::CheckedFile;
use crate::frontend::ast::{Choice, Item, Structure};
use crate::smith::c::emit::choice_arm_emits;
use crate::smith::c::patterns::{
    GreedyRootLayout, LengthPrefixedLayout, MessageWrapperLayout, ParametricBlobLayout,
    ParametricOpaqueLayout, ParametricTail, ParametricWireTailLayout, PduMessageLayout,
    PrefixField, greedy_root_layout, length_prefixed_layout, message_wrapper_layout,
    parametric_blob_layout, parametric_opaque_layout, parametric_wire_tail_layout,
    pdu_message_layout,
};
use crate::smith::codegen::lang::Lang;
use crate::smith::codegen::wire::{emit_layout_decode_body, emit_layout_encode_body};

use super::emit::{fn_name, to_snake_case};

pub fn emit_composite_codecs(output: &mut String, file: &CheckedFile, namespace: &str) {
    let lang = Lang::cpp(namespace);
    let mut message_wrappers = Vec::new();
    let mut greedy_roots = Vec::new();
    let mut pdu_messages = Vec::new();

    for item in &file.file.items {
        let Item::Structure(structure) = &item.node else {
            continue;
        };
        if let Some(layout) = parametric_blob_layout(structure, &file.symbols, file) {
            emit_parametric_blob(output, &lang, structure, &layout);
        } else if let Some(layout) = parametric_wire_tail_layout(structure, &file.symbols, file) {
            emit_parametric_wire_tail(output, &lang, structure, &layout);
        } else if let Some(layout) = parametric_opaque_layout(structure, &file.symbols) {
            emit_parametric_opaque(output, &lang, structure, &layout);
        } else if let Some(layout) = length_prefixed_layout(structure, &file.symbols) {
            emit_length_prefixed(output, &lang, structure, &layout);
        } else if let Some(layout) = message_wrapper_layout(structure, file) {
            message_wrappers.push(layout);
        } else if let Some(layout) = greedy_root_layout(structure) {
            greedy_roots.push(layout);
        } else if let Some(layout) = pdu_message_layout(structure, file) {
            pdu_messages.push(layout);
        }
    }

    for item in &file.file.items {
        let Item::Choice(choice) = &item.node else {
            continue;
        };
        let implemented = implemented_payload_types(file);
        emit_stub_payloads(output, &lang, choice, file, &implemented);
        emit_choice(output, &lang, choice, file);
    }

    for layout in &message_wrappers {
        emit_message_wrapper(output, &lang, layout);
    }
    for layout in &pdu_messages {
        emit_pdu_message(output, &lang, layout);
    }
    for layout in &greedy_roots {
        emit_greedy_root(output, &lang, layout, &pdu_messages);
    }
}

fn emit_parametric_blob(
    output: &mut String,
    lang: &Lang,
    structure: &Structure,
    layout: &ParametricBlobLayout,
) {
    let name = &structure.name;
    let encode = fn_name(name, "encode");
    let decode = fn_name(name, "decode");
    let enc_sig = lang.encode_fn(&encode, name, &[], false);
    let dec_sig = lang.decode_fn(&decode, name, &[], true);
    let array = &layout.array_field;
    let prefix_bytes = layout.prefix_bytes;

    let mut enc_prefix = String::new();
    let mut dec_prefix = String::new();
    let mut offset = 0u32;
    for field in &layout.prefix_fields {
        if let Some(nested) = &field.nested_type {
            let ne = fn_name(nested, "encode");
            let nd = fn_name(nested, "decode");
            enc_prefix.push_str(&format!(
                "    if (auto nested_len = {ne}(in.{fname}, buf.subspan({offset})); !nested_len || *nested_len != {wb}) {fail}\n",
                fname = field.name,
                wb = field.wire_bytes,
                fail = lang.ret_fail()
            ));
            dec_prefix.push_str(&format!(
                "    if (auto nested = {nd}(buf.subspan({offset}), out.{fname}); !nested) {fail}\n",
                fname = field.name,
                fail = lang.ret_fail()
            ));
        } else {
            emit_prefix_write(&mut enc_prefix, lang, field, offset, "in");
            emit_prefix_read(&mut dec_prefix, lang, field, offset, "out");
        }
        offset += field.wire_bytes;
    }

    output.push_str(&format!(
        r#"{enc_sig} {{
    const std::uint16_t payload_size = static_cast<std::uint16_t>({prefix_bytes}u + in.{array}.size());
    if (buf.size() < payload_size) {fail}
{enc_prefix}    if (!in.{array}.empty()) {{
        std::memcpy(buf.data() + {prefix_bytes}, in.{array}.data(), in.{array}.size());
    }}
    return payload_size;
}}

{dec_sig} {{
    if (payload_size < {prefix_bytes} || buf.size() < payload_size) {fail}
{dec_prefix}    const std::size_t sample_len = payload_size - {prefix_bytes};
    out.{array}.resize(sample_len);
    if (sample_len > 0) {{
        std::memcpy(out.{array}.data(), buf.data() + {prefix_bytes}, sample_len);
    }}
    return {{}};
}}

"#,
        fail = lang.ret_fail()
    ));
}

fn emit_parametric_wire_tail(
    output: &mut String,
    lang: &Lang,
    structure: &Structure,
    layout: &ParametricWireTailLayout,
) {
    let name = &structure.name;
    let encode = fn_name(name, "encode");
    let decode = fn_name(name, "decode");
    let enc_sig = lang.encode_fn(&encode, name, &[], false);
    let dec_sig = lang.decode_fn(&decode, name, &[], true);
    let prefix_bytes = layout.prefix_bytes;

    let mut enc_prefix = String::new();
    let mut dec_prefix = String::new();
    emit_layout_encode_body(&mut enc_prefix, lang, &layout.prefix_layout.fields, "in");
    emit_layout_decode_body(&mut dec_prefix, lang, &layout.prefix_layout.fields, "out");

    match &layout.tail {
        ParametricTail::Bytes { field, .. } => {
            output.push_str(&format!(
                r#"{enc_sig} {{
    const std::uint16_t payload_size = static_cast<std::uint16_t>({prefix_bytes}u + in.{field}.size());
    if (buf.size() < payload_size) {fail}
    std::fill(buf.begin(), buf.begin() + {prefix_bytes}, std::uint8_t{{0}});
{enc_prefix}    if (!in.{field}.empty()) {{
        std::memcpy(buf.data() + {prefix_bytes}, in.{field}.data(), in.{field}.size());
    }}
    return payload_size;
}}

{dec_sig} {{
    if (payload_size < {prefix_bytes} || buf.size() < payload_size) {fail}
{dec_prefix}    const std::size_t sample_len = payload_size - {prefix_bytes};
    out.{field}.resize(sample_len);
    if (sample_len > 0) {{
        std::memcpy(out.{field}.data(), buf.data() + {prefix_bytes}, sample_len);
    }}
    return {{}};
}}

"#,
                fail = lang.ret_fail()
            ));
        }
        ParametricTail::Structs {
            field,
            element_type,
            element_bytes,
            ..
        } => {
            let ee = fn_name(element_type, "encode");
            let ed = fn_name(element_type, "decode");
            output.push_str(&format!(
                r#"{enc_sig} {{
    const std::uint16_t payload_size = static_cast<std::uint16_t>({prefix_bytes}u + in.{field}.size() * {element_bytes});
    if (buf.size() < payload_size) {fail}
    std::fill(buf.begin(), buf.begin() + {prefix_bytes}, std::uint8_t{{0}});
{enc_prefix}    for (std::size_t i = 0; i < in.{field}.size(); ++i) {{
        if (auto elem_len = {ee}(in.{field}[i], buf.subspan({prefix_bytes} + i * {element_bytes})); !elem_len || *elem_len != {element_bytes}) {fail}
    }}
    return payload_size;
}}

{dec_sig} {{
    if (payload_size < {prefix_bytes} || buf.size() < payload_size) {fail}
{dec_prefix}    const std::size_t tail_bytes = payload_size - {prefix_bytes};
    if (tail_bytes % {element_bytes} != 0) {fail}
    const std::size_t count = tail_bytes / {element_bytes};
    out.{field}.resize(count);
    for (std::size_t i = 0; i < count; ++i) {{
        if (auto elem = {ed}(buf.subspan({prefix_bytes} + i * {element_bytes}, {element_bytes}), out.{field}[i])); !elem) {fail}
    }}
    return {{}};
}}

"#,
                fail = lang.ret_fail()
            ));
        }
    }
}

fn emit_parametric_opaque(
    output: &mut String,
    lang: &Lang,
    structure: &Structure,
    layout: &ParametricOpaqueLayout,
) {
    let name = &structure.name;
    let encode = fn_name(name, "encode");
    let decode = fn_name(name, "decode");
    let enc_sig = lang.encode_fn(&encode, name, &[], false);
    let dec_sig = lang.decode_fn(&decode, name, &[], true);
    let array = &layout.array_field;
    output.push_str(&format!(
        r#"{enc_sig} {{
    if (buf.size() < in.{array}.size()) {fail}
    if (!in.{array}.empty()) {{
        std::memcpy(buf.data(), in.{array}.data(), in.{array}.size());
    }}
    return in.{array}.size();
}}

{dec_sig} {{
    if (buf.size() < payload_size) {fail}
    out.{array}.resize(payload_size);
    if (payload_size > 0) {{
        std::memcpy(out.{array}.data(), buf.data(), payload_size);
    }}
    return {{}};
}}

"#,
        fail = lang.ret_fail()
    ));
}

fn emit_length_prefixed(
    output: &mut String,
    lang: &Lang,
    structure: &Structure,
    layout: &LengthPrefixedLayout,
) {
    let name = &structure.name;
    let encode = fn_name(name, "encode");
    let decode = fn_name(name, "decode");
    let enc_sig = lang.encode_fn(&encode, name, &[], false);
    let dec_sig = lang.decode_fn_consumed(&decode, name);
    let len_field = &layout.length_field;
    let array = &layout.array_field;
    output.push_str(&format!(
        r#"{enc_sig} {{
    if (buf.size() < 2u + in.{array}.size()) {fail}
    buf[0] = static_cast<std::uint8_t>((in.{len_field} >> 8) & 0xFFu);
    buf[1] = static_cast<std::uint8_t>(in.{len_field} & 0xFFu);
    if (!in.{array}.empty()) {{
        std::memcpy(buf.data() + 2, in.{array}.data(), in.{array}.size());
    }}
    return 2u + in.{array}.size();
}}

{dec_sig} {{
    if (buf.size() < 2) {fail}
    out.{len_field} = static_cast<std::uint16_t>((static_cast<std::uint16_t>(buf[0]) << 8) | buf[1]);
    if (buf.size() < 2u + out.{len_field}) {fail}
    out.{array}.resize(out.{len_field});
    if (out.{len_field} > 0) {{
        std::memcpy(out.{array}.data(), buf.data() + 2, out.{len_field});
    }}
    return 2u + out.{len_field};
}}

"#,
        fail = lang.ret_fail()
    ));
}

fn emit_choice(output: &mut String, lang: &Lang, choice: &Choice, file: &CheckedFile) {
    let encode = fn_name(&choice.name, "encode");
    let decode = fn_name(&choice.name, "decode");
    let disc_ty = choice
        .params
        .iter()
        .find(|p| p.name == choice.discriminant)
        .map(|p| p.ty.clone())
        .unwrap_or_else(|| choice.discriminant.clone());
    let arms = choice_arm_emits(choice, file);
    let enc_sig = lang.choice_encode_sig(&encode, &choice.name, &disc_ty);
    let dec_sig = lang.choice_decode_sig(&decode, &choice.name, &disc_ty);

    output.push_str(&format!("{enc_sig} {{\n    switch (tag) {{\n",));
    for arm in &arms {
        if arm.enum_case.starts_with("/*") {
            continue;
        }
        let arm_encode = fn_name(&arm.payload_type, "encode");
        output.push_str(&format!(
            "    case {case_}:\n        return {arm_encode}({body}, buf);\n",
            case_ = lang.enum_case(&arm.enum_case),
            body = lang.choice_body_encode(&arm.payload_type, &arm.union_member),
        ));
    }
    output.push_str(&format!(
        "    default: {fail}\n    }}\n}}\n\n",
        fail = lang.ret_fail()
    ));

    output.push_str(&format!(
        "{dec_sig} {{\n    out.tag = tag;\n    switch (tag) {{\n",
    ));
    for arm in &arms {
        if arm.enum_case.starts_with("/*") {
            continue;
        }
        let arm_decode = fn_name(&arm.payload_type, "decode");
        output.push_str(&format!(
            "    case {case_}:\n        return {arm_decode}(buf, payload_size, {body});\n",
            case_ = lang.enum_case(&arm.enum_case),
            body = lang.choice_body_decode(&arm.payload_type, &arm.union_member),
        ));
    }
    output.push_str(&format!(
        "    default: {fail}\n    }}\n}}\n\n",
        fail = lang.ret_fail()
    ));
}

fn emit_message_wrapper(output: &mut String, lang: &Lang, layout: &MessageWrapperLayout) {
    let encode = fn_name(&layout.name, "encode");
    let decode = fn_name(&layout.name, "decode");
    let he = fn_name(&layout.header_type, "encode");
    let hd = fn_name(&layout.header_type, "decode");
    let be = fn_name(&layout.choice_name, "encode");
    let bd = fn_name(&layout.choice_name, "decode");
    let hw = layout.header_wire_bytes;
    let disc = format!("in.{}.{}", layout.header_field, layout.discriminant_field);
    let disc_out = format!("out.{}.{}", layout.header_field, layout.discriminant_field);
    let enc_sig = lang.encode_fn(&encode, &layout.name, &[], false);
    let dec_sig = lang.decode_fn(&decode, &layout.name, &[], false);

    output.push_str(&format!(
        r#"{enc_sig} {{
    if (buf.size() < {hw}) {fail}
    if (auto body_len = {be}(in.{body}, {disc}, buf.subspan({hw})); !body_len) {fail}
    {header} hdr = in.{header};
    if (auto hdr_len = {he}(hdr, static_cast<std::uint16_t>(*body_len), buf.first({hw})); !hdr_len) {fail}
    return {hw} + *body_len;
}}

{dec_sig} {{
    std::uint16_t body_size = 0;
    if (auto hdr = {hd}(buf, body_size, out.{header})); !hdr) {fail}
    if (buf.size() < {hw} + body_size) {fail}
    return {bd}(buf.subspan({hw}, body_size), {disc_out}, body_size, out.{body});
}}

"#,
        body = layout.body_field,
        header = layout.header_type,
        fail = lang.ret_fail()
    ));
}

fn emit_pdu_message(output: &mut String, lang: &Lang, layout: &PduMessageLayout) {
    let encode = fn_name(&layout.name, "encode");
    let decode = fn_name(&layout.name, "decode");
    let me = fn_name(&layout.message_type, "encode");
    let md = fn_name(&layout.message_type, "decode");
    let hd = fn_name(&layout.header_type, "decode");
    let snake = to_snake_case(&layout.name);
    let concat_in = format!("in.{}", layout.concat_field);
    let concat_out = format!("out.{}", layout.concat_field);
    let enc_sig = lang.encode_fn(&encode, &layout.name, &[], false);
    let dec_sig = lang.decode_fn_consumed(&decode, &layout.name);

    output.push_str(&format!(
        r#"[[nodiscard]] inline std::size_t {snake}_padding_len(std::size_t msg_len, bool concat) {{
    if (!concat) return 0;
    return ({align}u - (msg_len % {align}u)) % {align}u;
}}

{enc_sig} {{
    if (auto msg_len = {me}(in.{message}, buf); !msg_len) {fail}
    const std::size_t padding = {snake}_padding_len(*msg_len, {concat_in});
    if (buf.size() < *msg_len + padding) {fail}
    if (padding > 0) std::fill(buf.begin() + *msg_len, buf.begin() + *msg_len + padding, std::uint8_t{{0}});
    return *msg_len + padding;
}}

{dec_sig} {{
    std::uint16_t body_size = 0;
    if (auto hdr = {hd}(buf, out.{header_path}, body_size)); !hdr) {fail}
    if (auto msg = {md}(buf, out.{message})); !msg) {fail}
    const std::size_t msg_len = {hw}u + static_cast<std::size_t>(body_size);
    const std::size_t padding = {snake}_padding_len(msg_len, {concat_out});
    if (buf.size() < msg_len + padding) {fail}
    return msg_len + padding;
}}

"#,
        message = layout.message_field,
        header_path = layout.header_field_path,
        hw = layout.header_wire_bytes,
        align = layout.align,
        fail = lang.ret_fail()
    ));
}

fn emit_greedy_root(
    output: &mut String,
    lang: &Lang,
    layout: &GreedyRootLayout,
    pdu_messages: &[PduMessageLayout],
) {
    let encode = fn_name(&layout.name, "encode");
    let decode = fn_name(&layout.name, "decode");
    let array = &layout.array_field;
    let element = &layout.element_type;

    if let Some(pdu) = pdu_messages.iter().find(|p| p.name == *element) {
        let pe = fn_name(&pdu.name, "encode");
        let pd = fn_name(&pdu.name, "decode");
        let concat = &pdu.concat_field;
        let enc_sig = lang.encode_fn(&encode, &layout.name, &[], false);
        let dec_sig = lang.decode_fn(&decode, &layout.name, &[], false);
        output.push_str(&format!(
            r#"{enc_sig} {{
    std::size_t offset = 0;
    for (std::size_t i = 0; i < in.{array}.size(); ++i) {{
        auto entry = in.{array}[i];
        entry.{concat} = (i + 1 < in.{array}.size());
        if (auto n = {pe}(entry, buf.subspan(offset)); !n) {fail}
        offset += *n;
    }}
    return offset;
}}

{dec_sig} {{
    std::size_t offset = 0;
    out.{array}.clear();
    while (offset < buf.size()) {{
        {pdu_name} entry{{}};
        if (auto consumed = {pd}(buf.subspan(offset), entry); !consumed) {fail}
        out.{array}.push_back(std::move(entry));
        offset += *consumed;
    }}
    return {{}};
}}

"#,
            pdu_name = pdu.name,
            fail = lang.ret_fail()
        ));
        return;
    }

    let ee = fn_name(element, "encode");
    let ed = fn_name(element, "decode");
    let enc_sig = lang.encode_fn(&encode, &layout.name, &[], false);
    let dec_sig = lang.decode_fn(&decode, &layout.name, &[], false);
    output.push_str(&format!(
        r#"{enc_sig} {{
    std::size_t offset = 0;
    for (const auto& entry : in.{array}) {{
        if (auto n = {ee}(entry, buf.subspan(offset)); !n) {fail}
        offset += *n;
    }}
    return offset;
}}

{dec_sig} {{
    std::size_t offset = 0;
    out.{array}.clear();
    while (offset < buf.size()) {{
        {element} entry{{}};
        if (auto consumed = {ed}(buf.subspan(offset), entry); !consumed) {fail}
        out.{array}.push_back(std::move(entry));
        offset += *consumed;
    }}
    return {{}};
}}

"#,
        fail = lang.ret_fail()
    ));
}

fn emit_stub_payloads(
    output: &mut String,
    lang: &Lang,
    choice: &Choice,
    file: &CheckedFile,
    implemented: &std::collections::HashSet<String>,
) {
    let mut emitted = std::collections::HashSet::new();
    for arm in choice_arm_emits(choice, file) {
        if implemented.contains(&arm.payload_type) || !emitted.insert(arm.payload_type.clone()) {
            continue;
        }
        let encode = fn_name(&arm.payload_type, "encode");
        let decode = fn_name(&arm.payload_type, "decode");
        let enc_sig = lang.encode_fn(&encode, &arm.payload_type, &[], false);
        let dec_sig = lang.decode_fn(&decode, &arm.payload_type, &[], true);
        output.push_str(&format!(
            r#"{enc_sig} {{ return std::unexpected(-1); }}
{dec_sig} {{ return std::unexpected(-1); }}

"#,
        ));
    }
}

fn implemented_payload_types(file: &CheckedFile) -> std::collections::HashSet<String> {
    let mut types = std::collections::HashSet::new();
    for item in &file.file.items {
        let Item::Structure(structure) = &item.node else {
            continue;
        };
        if parametric_blob_layout(structure, &file.symbols, file).is_some()
            || parametric_wire_tail_layout(structure, &file.symbols, file).is_some()
            || parametric_opaque_layout(structure, &file.symbols).is_some()
        {
            types.insert(structure.name.clone());
        }
    }
    types
}

fn emit_prefix_write(
    output: &mut String,
    lang: &Lang,
    field: &PrefixField,
    offset: u32,
    base: &str,
) {
    let acc = lang.member(base, &field.name);
    match field.wire_bytes {
        1 => output.push_str(&format!(
            "    {} = static_cast<std::uint8_t>({acc} & 0xFFu);\n",
            lang.buf_byte(&offset.to_string())
        )),
        2 => {
            output.push_str(&format!(
                "    {} = static_cast<std::uint8_t>(({acc} >> 8) & 0xFFu);\n",
                lang.buf_byte(&offset.to_string())
            ));
            output.push_str(&format!(
                "    {} = static_cast<std::uint8_t>({acc} & 0xFFu);\n",
                lang.buf_byte(&(offset + 1).to_string())
            ));
        }
        4 => {
            for (i, shift) in [(0u32, 24), (1, 16), (2, 8), (3, 0)] {
                output.push_str(&format!(
                    "    {} = static_cast<std::uint8_t>(({acc} >> {shift}) & 0xFFu);\n",
                    lang.buf_byte(&(offset + i).to_string())
                ));
            }
        }
        8 => {
            for i in 0..8u32 {
                let shift = (7 - i) * 8;
                output.push_str(&format!(
                    "    {} = static_cast<std::uint8_t>(({acc} >> {shift}) & 0xFFu);\n",
                    lang.buf_byte(&(offset + i).to_string())
                ));
            }
        }
        _ => {}
    }
}

fn emit_prefix_read(
    output: &mut String,
    lang: &Lang,
    field: &PrefixField,
    offset: u32,
    base: &str,
) {
    let target = lang.member(base, &field.name);
    let ty = lang.host_type(match field.wire_bytes {
        1 => "uint8_t",
        2 => "uint16_t",
        4 => "uint32_t",
        8 => "uint64_t",
        _ => "uint32_t",
    });
    match field.wire_bytes {
        1 => output.push_str(&format!(
            "    {target} = ({ty})({} & 0xFFu);\n",
            lang.buf_byte(&offset.to_string())
        )),
        2 => output.push_str(&format!(
            "    {target} = ({ty})((static_cast<std::uint16_t>({}) << 8) | {});\n",
            lang.buf_byte(&offset.to_string()),
            lang.buf_byte(&(offset + 1).to_string())
        )),
        4 => output.push_str(&format!(
            "    {target} = ({ty})((static_cast<std::uint32_t>({}) << 24) | (static_cast<std::uint32_t>({}) << 16) | (static_cast<std::uint32_t>({}) << 8) | {});\n",
            lang.buf_byte(&offset.to_string()),
            lang.buf_byte(&(offset + 1).to_string()),
            lang.buf_byte(&(offset + 2).to_string()),
            lang.buf_byte(&(offset + 3).to_string())
        )),
        8 => {
            output.push_str(&format!("    {{\n        {ty} __v = 0;\n"));
            for i in 0..8u32 {
                output.push_str(&format!(
                    "        __v = ({ty})((__v << 8) | {});\n",
                    lang.buf_byte(&(offset + i).to_string())
                ));
            }
            output.push_str(&format!("        {target} = __v;\n    }}\n"));
        }
        _ => {}
    }
}
