use crate::analyze::CheckedFile;
use crate::frontend::ast::{Choice, Item};
use crate::smith::codegen::lang::Lang;

use super::codec::{emit_layout_decode_body, emit_layout_encode_body};
use super::emit::{choice_arm_emits, fn_name, to_snake_case};
use super::layout::StructureLayout;
use super::patterns::{
    GreedyRootLayout, LengthPrefixedLayout, MessageWrapperLayout, ParametricBlobLayout,
    ParametricOpaqueLayout, ParametricTail, ParametricWireTailLayout, PduMessageLayout,
    PrefixField, greedy_root_layout, length_prefixed_layout, message_wrapper_layout,
    parametric_blob_layout, parametric_opaque_layout, parametric_wire_tail_layout,
    pdu_message_layout,
};

pub fn emit_composite_codecs(output: &mut String, file: &CheckedFile, prefix: &str) {
    let mut message_wrappers = Vec::new();
    let mut greedy_roots = Vec::new();
    let mut pdu_messages = Vec::new();

    for item in &file.file.items {
        let Item::Structure(structure) = &item.node else {
            continue;
        };

        if let Some(layout) = parametric_blob_layout(structure, &file.symbols, file) {
            emit_parametric_blob_codec(output, prefix, structure, &layout);
        } else if let Some(layout) = parametric_wire_tail_layout(structure, &file.symbols, file) {
            emit_parametric_wire_tail_codec(output, prefix, structure, &layout);
        } else if let Some(layout) = parametric_opaque_layout(structure, &file.symbols) {
            emit_parametric_opaque_codec(output, prefix, structure, &layout);
        } else if let Some(layout) = length_prefixed_layout(structure, &file.symbols) {
            emit_length_prefixed_codec(output, prefix, structure, &layout);
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
        emit_stub_payload_codecs(output, prefix, choice, file, &implemented);
        emit_choice_codec(output, prefix, choice, file);
    }

    for layout in &message_wrappers {
        emit_message_wrapper_codec(output, prefix, file, layout);
    }

    for layout in &pdu_messages {
        emit_pdu_message_codec(output, prefix, layout);
    }

    for layout in &greedy_roots {
        emit_greedy_root_codec(output, prefix, layout, &pdu_messages);
    }
}

fn emit_parametric_blob_codec(
    output: &mut String,
    prefix: &str,
    structure: &crate::frontend::ast::Structure,
    layout: &ParametricBlobLayout,
) {
    let name = &structure.name;
    let encode = fn_name(prefix, name, "encode");
    let decode = fn_name(prefix, name, "decode");
    let lang = Lang::c(prefix);
    let enc_sig = lang.encode_fn(&encode, name, &[], false);
    let dec_sig = lang.decode_fn(&decode, name, &[], true);
    let array_field = &layout.array_field;

    let mut encode_prefix = String::new();
    let mut decode_prefix = String::new();
    let mut offset = 0u32;
    for field in &layout.prefix_fields {
        if let Some(nested_type) = &field.nested_type {
            let nested_encode = fn_name(prefix, nested_type, "encode");
            let nested_decode = fn_name(prefix, nested_type, "decode");
            encode_prefix.push_str(&format!(
                "    {{\n        size_t nested_len = 0;\n        if ({nested_encode}(&in->{name}, buf + {offset}, cap - {offset}, &nested_len) != 0) return -1;\n        if (nested_len != {wire_bytes}u) return -1;\n    }}\n",
                name = field.name,
                offset = offset,
                wire_bytes = field.wire_bytes,
            ));
            decode_prefix.push_str(&format!(
                "    if ({nested_decode}(buf + {offset}, len - {offset}, &out->{name}) != 0) return -1;\n",
                name = field.name,
                offset = offset,
            ));
        } else {
            emit_prefix_write(&mut encode_prefix, field, offset);
            emit_prefix_read(&mut decode_prefix, field, offset);
        }
        offset += field.wire_bytes;
    }

    output.push_str(&format!(
        r#"{enc_sig} {{
    const uint16_t payload_size = (uint16_t)({prefix_bytes}u + (uint16_t)in->{array_field}.len);
    if (cap < (size_t)payload_size) return -1;
{encode_prefix}    if (in->{array_field}.len > 0) {{
        if (in->{array_field}.data == NULL) return -1;
        memcpy(buf + {prefix_bytes}, in->{array_field}.data, in->{array_field}.len);
    }}
    *out_len = payload_size;
    return 0;
}}

{dec_sig} {{
    if (payload_size < {prefix_bytes} || len < (size_t)payload_size) return -1;
{decode_prefix}    const size_t sample_len = payload_size - {prefix_bytes};
    out->{array_field}.len = sample_len;
    if (sample_len > 0) {{
        if (out->{array_field}.data == NULL) return -1;
        memcpy(out->{array_field}.data, buf + {prefix_bytes}, sample_len);
    }}
    return 0;
}}

"#,
        prefix_bytes = layout.prefix_bytes,
    ));
}

fn emit_parametric_wire_tail_codec(
    output: &mut String,
    prefix: &str,
    structure: &crate::frontend::ast::Structure,
    layout: &ParametricWireTailLayout,
) {
    let name = &structure.name;
    let encode = fn_name(prefix, name, "encode");
    let decode = fn_name(prefix, name, "decode");
    let lang = Lang::c(prefix);
    let enc_sig = lang.encode_fn(&encode, name, &[], false);
    let dec_sig = lang.decode_fn(&decode, name, &[], true);
    let prefix_bytes = layout.prefix_bytes;

    let mut encode_prefix = String::new();
    let mut decode_prefix = String::new();
    emit_layout_encode_statements(&mut encode_prefix, &layout.prefix_layout, "in");
    emit_layout_decode_statements(&mut decode_prefix, &layout.prefix_layout, "out");

    match &layout.tail {
        ParametricTail::Bytes { field, subtract: _ } => {
            output.push_str(&format!(
                r#"{enc_sig} {{
    const uint16_t payload_size = (uint16_t)({prefix_bytes}u + (uint16_t)in->{field}.len);
    if (cap < (size_t)payload_size) return -1;
    for (size_t i = 0; i < {prefix_bytes}; ++i) buf[i] = 0;
{encode_prefix}    if (in->{field}.len > 0) {{
        if (in->{field}.data == NULL) return -1;
        memcpy(buf + {prefix_bytes}, in->{field}.data, in->{field}.len);
    }}
    *out_len = payload_size;
    return 0;
}}

{dec_sig} {{
    if (payload_size < {prefix_bytes} || len < (size_t)payload_size) return -1;
{decode_prefix}    const size_t sample_len = payload_size - {prefix_bytes};
    out->{field}.len = sample_len;
    if (sample_len > 0) {{
        if (out->{field}.data == NULL) return -1;
        memcpy(out->{field}.data, buf + {prefix_bytes}, sample_len);
    }}
    return 0;
}}

"#,
                field = field,
            ));
        }
        ParametricTail::Structs {
            field,
            element_type,
            element_bytes,
            subtract: _,
        } => {
            let elem_encode = fn_name(prefix, element_type, "encode");
            let elem_decode = fn_name(prefix, element_type, "decode");
            output.push_str(&format!(
                r#"{enc_sig} {{
    const uint16_t payload_size = (uint16_t)({prefix_bytes}u + (uint16_t)(in->{field}.len * {element_bytes}));
    if (cap < (size_t)payload_size) return -1;
    for (size_t i = 0; i < {prefix_bytes}; ++i) buf[i] = 0;
{encode_prefix}    for (size_t i = 0; i < in->{field}.len; ++i) {{
        size_t elem_len = 0;
        if ({elem_encode}(&in->{field}.data[i], buf + {prefix_bytes} + i * {element_bytes}, cap - ({prefix_bytes} + i * {element_bytes}), &elem_len) != 0)
            return -1;
        if (elem_len != {element_bytes}) return -1;
    }}
    *out_len = payload_size;
    return 0;
}}

{dec_sig} {{
    if (payload_size < {prefix_bytes} || len < (size_t)payload_size) return -1;
{decode_prefix}    const size_t tail_bytes = payload_size - {prefix_bytes};
    if (tail_bytes % {element_bytes} != 0) return -1;
    const size_t count = tail_bytes / {element_bytes};
    out->{field}.len = count;
    for (size_t i = 0; i < count; ++i) {{
        if ({elem_decode}(buf + {prefix_bytes} + i * {element_bytes}, {element_bytes}, &out->{field}.data[i]) != 0)
            return -1;
    }}
    return 0;
}}

"#,
            ));
        }
    }
}

fn emit_layout_encode_statements(
    output: &mut String,
    layout: &StructureLayout,
    input_accessor: &str,
) {
    emit_layout_encode_body(output, layout, input_accessor);
}

fn emit_layout_decode_statements(
    output: &mut String,
    layout: &StructureLayout,
    output_accessor: &str,
) {
    emit_layout_decode_body(output, layout, output_accessor);
}

fn emit_parametric_opaque_codec(
    output: &mut String,
    prefix: &str,
    structure: &crate::frontend::ast::Structure,
    layout: &ParametricOpaqueLayout,
) {
    let name = &structure.name;
    let array_field = &layout.array_field;
    let encode = fn_name(prefix, name, "encode");
    let decode = fn_name(prefix, name, "decode");
    let lang = Lang::c(prefix);
    let enc_sig = lang.encode_fn(&encode, name, &[], false);
    let dec_sig = lang.decode_fn(&decode, name, &[], true);

    output.push_str(&format!(
        r#"{enc_sig} {{
    if (cap < in->{array_field}.len) return -1;
    if (in->{array_field}.len > 0) {{
        if (in->{array_field}.data == NULL) return -1;
        memcpy(buf, in->{array_field}.data, in->{array_field}.len);
    }}
    *out_len = in->{array_field}.len;
    return 0;
}}

{dec_sig} {{
    if (len < (size_t)payload_size) return -1;
    out->{array_field}.len = payload_size;
    if (payload_size > 0) {{
        if (out->{array_field}.data == NULL) return -1;
        memcpy(out->{array_field}.data, buf, payload_size);
    }}
    return 0;
}}

"#,
    ));
}

fn emit_length_prefixed_codec(
    output: &mut String,
    prefix: &str,
    structure: &crate::frontend::ast::Structure,
    layout: &LengthPrefixedLayout,
) {
    let name = &structure.name;
    let len_field = &layout.length_field;
    let array_field = &layout.array_field;
    let encode = fn_name(prefix, name, "encode");
    let decode = fn_name(prefix, name, "decode");
    let lang = Lang::c(prefix);
    let enc_sig = lang.encode_fn(&encode, name, &[], false);
    let dec_sig = lang.decode_fn_consumed(&decode, name);

    output.push_str(&format!(
        r#"{enc_sig} {{
    if (cap < 2u + in->{array_field}.len) return -1;
    buf[0] = (uint8_t)((in->{len_field} >> 8) & 0xFFu);
    buf[1] = (uint8_t)(in->{len_field} & 0xFFu);
    if (in->{array_field}.len > 0) {{
        if (in->{array_field}.data == NULL) return -1;
        memcpy(buf + 2, in->{array_field}.data, in->{array_field}.len);
    }}
    *out_len = 2u + in->{array_field}.len;
    return 0;
}}

{dec_sig} {{
    if (len < 2) return -1;
    out->{len_field} = (uint16_t)(((uint16_t)buf[0] << 8) | buf[1]);
    if (len < (size_t)2u + out->{len_field}) return -1;
    out->{array_field}.len = out->{len_field};
    if (out->{len_field} > 0) {{
        if (out->{array_field}.data == NULL) return -1;
        memcpy(out->{array_field}.data, buf + 2, out->{len_field});
    }}
    *consumed = 2u + out->{len_field};
    return 0;
}}

"#,
    ));
}

fn emit_prefix_write(output: &mut String, field: &PrefixField, byte_offset: u32) {
    match field.wire_bytes {
        1 => {
            output.push_str(&format!(
                "    buf[{byte_offset}] = (uint8_t)(in->{name} & 0xFFu);\n",
                name = field.name,
            ));
        }
        2 => {
            output.push_str(&format!(
                "    buf[{byte_offset}] = (uint8_t)((in->{name} >> 8) & 0xFFu);\n    buf[{next}] = (uint8_t)(in->{name} & 0xFFu);\n",
                name = field.name,
                next = byte_offset + 1,
            ));
        }
        4 => {
            output.push_str(&format!(
                "    buf[{byte_offset}] = (uint8_t)((in->{name} >> 24) & 0xFFu);\n    buf[{b1}] = (uint8_t)((in->{name} >> 16) & 0xFFu);\n    buf[{b2}] = (uint8_t)((in->{name} >> 8) & 0xFFu);\n    buf[{b3}] = (uint8_t)(in->{name} & 0xFFu);\n",
                name = field.name,
                b1 = byte_offset + 1,
                b2 = byte_offset + 2,
                b3 = byte_offset + 3,
            ));
        }
        8 => {
            output.push_str(&format!(
                "    buf[{byte_offset}] = (uint8_t)((in->{name} >> 56) & 0xFFu);\n    buf[{b1}] = (uint8_t)((in->{name} >> 48) & 0xFFu);\n    buf[{b2}] = (uint8_t)((in->{name} >> 40) & 0xFFu);\n    buf[{b3}] = (uint8_t)((in->{name} >> 32) & 0xFFu);\n    buf[{b4}] = (uint8_t)((in->{name} >> 24) & 0xFFu);\n    buf[{b5}] = (uint8_t)((in->{name} >> 16) & 0xFFu);\n    buf[{b6}] = (uint8_t)((in->{name} >> 8) & 0xFFu);\n    buf[{b7}] = (uint8_t)(in->{name} & 0xFFu);\n",
                name = field.name,
                b1 = byte_offset + 1,
                b2 = byte_offset + 2,
                b3 = byte_offset + 3,
                b4 = byte_offset + 4,
                b5 = byte_offset + 5,
                b6 = byte_offset + 6,
                b7 = byte_offset + 7,
            ));
        }
        wire_bytes => {
            output.push_str(&format!(
                "    /* unsupported prefix width {wire_bytes} bytes for field {} */\n",
                field.name
            ));
        }
    }
}

fn emit_prefix_read(output: &mut String, field: &PrefixField, byte_offset: u32) {
    match field.wire_bytes {
        1 => {
            output.push_str(&format!(
                "    out->{name} = ({ty})(buf[{byte_offset}] & 0xFFu);\n",
                name = field.name,
                ty = host_type_for_bytes(field.wire_bytes),
            ));
        }
        2 => {
            output.push_str(&format!(
                "    out->{name} = ({ty})(((uint16_t)buf[{byte_offset}] << 8) | buf[{next}]);\n",
                name = field.name,
                ty = host_type_for_bytes(field.wire_bytes),
                next = byte_offset + 1,
            ));
        }
        4 => {
            output.push_str(&format!(
                "    out->{name} = ({ty})(((uint32_t)buf[{byte_offset}] << 24) | ((uint32_t)buf[{b1}] << 16) | ((uint32_t)buf[{b2}] << 8) | buf[{b3}]);\n",
                name = field.name,
                ty = host_type_for_bytes(field.wire_bytes),
                b1 = byte_offset + 1,
                b2 = byte_offset + 2,
                b3 = byte_offset + 3,
            ));
        }
        8 => {
            output.push_str(&format!(
                "    out->{name} = ({ty})(((uint64_t)buf[{byte_offset}] << 56) | ((uint64_t)buf[{b1}] << 48) | ((uint64_t)buf[{b2}] << 40) | ((uint64_t)buf[{b3}] << 32) | ((uint64_t)buf[{b4}] << 24) | ((uint64_t)buf[{b5}] << 16) | ((uint64_t)buf[{b6}] << 8) | buf[{b7}]);\n",
                name = field.name,
                ty = host_type_for_bytes(field.wire_bytes),
                b1 = byte_offset + 1,
                b2 = byte_offset + 2,
                b3 = byte_offset + 3,
                b4 = byte_offset + 4,
                b5 = byte_offset + 5,
                b6 = byte_offset + 6,
                b7 = byte_offset + 7,
            ));
        }
        wire_bytes => {
            output.push_str(&format!(
                "    /* unsupported prefix width {wire_bytes} bytes for field {} */\n",
                field.name
            ));
        }
    }
}

fn host_type_for_bytes(bytes: u32) -> &'static str {
    match bytes {
        1 => "uint8_t",
        2 => "uint16_t",
        4 => "uint32_t",
        8 => "uint64_t",
        _ => "uint32_t",
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

fn emit_stub_payload_codecs(
    output: &mut String,
    prefix: &str,
    choice: &Choice,
    file: &CheckedFile,
    implemented: &std::collections::HashSet<String>,
) {
    let arms = choice_arm_emits(choice, file);
    let mut emitted = std::collections::HashSet::new();
    for arm in arms {
        if implemented.contains(&arm.payload_type) || !emitted.insert(arm.payload_type.clone()) {
            continue;
        }
        let encode = fn_name(prefix, &arm.payload_type, "encode");
        let decode = fn_name(prefix, &arm.payload_type, "decode");
        let ty = &arm.payload_type;
        let lang = Lang::c(prefix);
        let enc_sig = lang.encode_fn(&encode, ty, &[], false);
        let dec_sig = lang.decode_fn(&decode, ty, &[], true);
        output.push_str(&format!(
            r#"{enc_sig} {{
    (void)in; (void)buf; (void)cap; (void)out_len;
    return -1;
}}
{dec_sig} {{
    (void)buf; (void)len; (void)payload_size; (void)out;
    return -1;
}}

"#,
        ));
    }
}

fn discriminant_c_type(choice: &Choice) -> String {
    choice
        .params
        .iter()
        .find(|param| param.name == choice.discriminant)
        .map(|param| param.ty.clone())
        .unwrap_or_else(|| "int".into())
}

fn emit_choice_codec(output: &mut String, prefix: &str, choice: &Choice, file: &CheckedFile) {
    let choice_name = &choice.name;
    let encode = fn_name(prefix, choice_name, "encode");
    let decode = fn_name(prefix, choice_name, "decode");
    let disc_ty = discriminant_c_type(choice);
    let arms = choice_arm_emits(choice, file);
    let lang = Lang::c(prefix);
    let enc_sig = lang.choice_encode_sig(&encode, choice_name, &disc_ty);
    let dec_sig = lang.choice_decode_sig(&decode, choice_name, &disc_ty);

    output.push_str(&format!("{enc_sig} {{\n"));
    output.push_str("    switch (tag) {\n");
    for arm in &arms {
        if arm.enum_case.starts_with("/*") {
            continue;
        }
        let arm_encode = fn_name(prefix, &arm.payload_type, "encode");
        output.push_str(&format!(
            "    case {enum_case}:\n        return {arm_encode}(&in->body.{member}, buf, cap, out_len);\n",
            enum_case = arm.enum_case,
            member = arm.union_member,
        ));
    }
    output.push_str("    default:\n        return -1;\n");
    output.push_str("    }\n}\n\n");

    output.push_str(&format!("{dec_sig} {{\n"));
    output.push_str("    out->tag = tag;\n");
    output.push_str("    switch (tag) {\n");
    for arm in &arms {
        if arm.enum_case.starts_with("/*") {
            continue;
        }
        let arm_decode = fn_name(prefix, &arm.payload_type, "decode");
        output.push_str(&format!(
            "    case {enum_case}:\n        return {arm_decode}(buf, len, payload_size, &out->body.{member});\n",
            enum_case = arm.enum_case,
            member = arm.union_member,
        ));
    }
    output.push_str("    default:\n        return -1;\n");
    output.push_str("    }\n}\n\n");
}

fn emit_message_wrapper_codec(
    output: &mut String,
    prefix: &str,
    _file: &CheckedFile,
    layout: &MessageWrapperLayout,
) {
    let name = &layout.name;
    let encode = fn_name(prefix, name, "encode");
    let decode = fn_name(prefix, name, "decode");
    let header_type = &layout.header_type;
    let header_encode = fn_name(prefix, header_type, "encode");
    let header_decode = fn_name(prefix, header_type, "decode");
    let body_encode = fn_name(prefix, &layout.choice_name, "encode");
    let body_decode = fn_name(prefix, &layout.choice_name, "decode");
    let header_wire = layout.header_wire_bytes;
    let disc_access = format!(
        "in->{header}.{disc}",
        header = layout.header_field,
        disc = layout.discriminant_field
    );
    let disc_decode = format!(
        "out->{header}.{disc}",
        header = layout.header_field,
        disc = layout.discriminant_field
    );
    let lang = Lang::c(prefix);
    let enc_sig = lang.encode_fn(&encode, name, &[], false);
    let dec_sig = lang.decode_fn(&decode, name, &[], false);

    output.push_str(&format!(
        r#"{enc_sig} {{
    size_t body_len = 0;
    if (cap < {header_wire}) return -1;
    if ({body_encode}(&in->{body}, {disc_access}, buf + {header_wire}, cap - {header_wire}, &body_len) != 0)
        return -1;
  {{
        {header_type} hdr = in->{header};
        size_t hdr_len = 0;
        if ({header_encode}(&hdr, (uint16_t)body_len, buf, {header_wire}, &hdr_len) != 0)
            return -1;
    }}
    *out_len = {header_wire} + body_len;
    return 0;
}}

{dec_sig} {{
    uint16_t body_size = 0;
    if ({header_decode}(buf, len, &body_size, &out->{header}) != 0)
        return -1;
    if (len < (size_t){header_wire} + body_size)
        return -1;
    return {body_decode}(buf + {header_wire}, body_size, {disc_decode}, body_size, &out->{body});
}}

"#,
        body = layout.body_field,
        header = layout.header_field,
    ));
}

fn emit_pdu_message_codec(output: &mut String, prefix: &str, layout: &PduMessageLayout) {
    let name = &layout.name;
    let encode = fn_name(prefix, name, "encode");
    let decode = fn_name(prefix, name, "decode");
    let message_encode = fn_name(prefix, &layout.message_type, "encode");
    let message_decode = fn_name(prefix, &layout.message_type, "decode");
    let header_decode = fn_name(prefix, &layout.header_type, "decode");
    let concat_access = format!("in->{}", layout.concat_field);
    let concat_out = format!("out->{}", layout.concat_field);
    let header_path = &layout.header_field_path;
    let lang = Lang::c(prefix);
    let enc_sig = lang.encode_fn(&encode, name, &[], false);
    let dec_sig = lang.decode_fn_consumed(&decode, name);

    output.push_str(&format!(
        r#"static size_t {snake}_padding_len(size_t msg_len, int concat) {{
    if (!concat) return 0;
    return (size_t)(({align}u - (msg_len % {align}u)) % {align}u);
}}

{enc_sig} {{
    size_t msg_len = 0;
    if ({message_encode}(&in->{message}, buf, cap, &msg_len) != 0)
        return -1;
    const size_t padding = {snake}_padding_len(msg_len, (int){concat_access});
    if (cap < msg_len + padding) return -1;
    if (padding > 0)
        memset(buf + msg_len, 0, padding);
    *out_len = msg_len + padding;
    return 0;
}}

{dec_sig} {{
    uint16_t body_size = 0;
    if ({header_decode}(buf, len, &body_size, &out->{header_path}) != 0)
        return -1;
    if ({message_decode}(buf, len, &out->{message}) != 0)
        return -1;
    const size_t msg_len = {header_wire}u + (size_t)body_size;
    const size_t padding = {snake}_padding_len(msg_len, (int){concat_out});
    if (len < msg_len + padding) return -1;
    *consumed = msg_len + padding;
    return 0;
}}

"#,
        snake = to_snake_case(name),
        message = layout.message_field,
        align = layout.align,
        header_wire = layout.header_wire_bytes,
    ));
}

fn emit_greedy_root_codec(
    output: &mut String,
    prefix: &str,
    layout: &GreedyRootLayout,
    pdu_messages: &[PduMessageLayout],
) {
    let name = &layout.name;
    let encode = fn_name(prefix, name, "encode");
    let decode = fn_name(prefix, name, "decode");
    let lang = Lang::c(prefix);
    let enc_sig = lang.encode_fn(&encode, name, &[], false);
    let dec_sig = lang.decode_fn(&decode, name, &[], false);
    let array_field = &layout.array_field;
    let element = &layout.element_type;
    let element_encode = fn_name(prefix, element, "encode");
    let element_snake = to_snake_case(element);

    let pdu_message = pdu_messages.iter().find(|entry| entry.name == *element);

    if let Some(pdu_message) = pdu_message {
        let pdu_encode = fn_name(prefix, &pdu_message.name, "encode");
        let pdu_decode = fn_name(prefix, &pdu_message.name, "decode");
        let concat_set = format!("entry.{}", pdu_message.concat_field);

        output.push_str(&format!(
            r#"{enc_sig} {{
    size_t offset = 0;
    for (size_t i = 0; i < in->{array_field}.len; ++i) {{
        {pdu_name} entry = in->{array_field}.data[i];
        if (i + 1 < in->{array_field}.len) {{
            {concat_set} = 1;
        }} else {{
            {concat_set} = 0;
        }}
        size_t n = 0;
        if ({pdu_encode}(&entry, buf + offset, cap - offset, &n) != 0)
            return -1;
        offset += n;
    }}
    *out_len = offset;
    return 0;
}}

{dec_sig} {{
    size_t offset = 0;
    size_t count = 0;
    while (offset < len) {{
        if (count >= out->{array_field}.len) return -1;
        size_t consumed = 0;
        if ({pdu_decode}(buf + offset, len - offset, &out->{array_field}.data[count], &consumed) != 0)
            return -1;
        offset += consumed;
        count++;
    }}
    out->{array_field}.len = count;
    return 0;
}}

"#,
            pdu_name = pdu_message.name,
        ));
        return;
    }

    if element_decode_has_consumed(prefix, element) {
        let element_decode = fn_name(prefix, element, "decode");
        output.push_str(&format!(
            r#"{enc_sig} {{
    size_t offset = 0;
    for (size_t i = 0; i < in->{array_field}.len; ++i) {{
        size_t n = 0;
        if ({element_encode}(&in->{array_field}.data[i], buf + offset, cap - offset, &n) != 0)
            return -1;
        offset += n;
    }}
    *out_len = offset;
    return 0;
}}

{dec_sig} {{
    size_t offset = 0;
    size_t count = 0;
    while (offset < len) {{
        if (count >= out->{array_field}.len) return -1;
        size_t consumed = 0;
        if ({element_decode}(buf + offset, len - offset, &out->{array_field}.data[count], &consumed) != 0)
            return -1;
        offset += consumed;
        count++;
    }}
    out->{array_field}.len = count;
    return 0;
}}

"#,
        ));
        return;
    }

    let _ = element_snake;
    output.push_str(&format!(
        r#"{enc_sig} {{
    (void)in; (void)buf; (void)cap; (void)out_len;
    return -1;
}}
{dec_sig} {{
    (void)buf; (void)len; (void)out;
    return -1;
}}

"#,
    ));
}

fn element_decode_has_consumed(_prefix: &str, _element: &str) -> bool {
    true
}
