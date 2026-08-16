use super::lang::{Lang, LangKind};
use crate::smith::c::layout::WireField;

pub fn emit_layout_encode_body(
    output: &mut String,
    lang: &Lang,
    fields: &[WireField],
    input_accessor: &str,
) {
    for field in fields {
        if field.is_computed {
            emit_encode_field(output, lang, field, &field.name);
            continue;
        }
        let accessor = if field.is_unused {
            "0".into()
        } else {
            lang.member(input_accessor, &field.name)
        };
        emit_encode_field(output, lang, field, &accessor);
    }
}

pub fn emit_layout_decode_body(
    output: &mut String,
    lang: &Lang,
    fields: &[WireField],
    output_accessor: &str,
) {
    for field in fields {
        if field.is_computed {
            continue;
        }
        emit_decode_field(output, lang, field, output_accessor);
    }
}

pub fn emit_encode_field(output: &mut String, lang: &Lang, field: &WireField, accessor: &str) {
    let value = wire_value(lang, accessor);
    if field_is_spanning(field) {
        emit_encode_spanning_field(output, lang, field, &value);
        return;
    }

    if field.bit_offset.is_multiple_of(8)
        && field.wire_bits.is_multiple_of(8)
        && field.wire_bits > 8
    {
        emit_encode_aligned_bytes(output, lang, field, &value);
        return;
    }

    let mask = ((1u64 << field.wire_bits) - 1) as u32;
    let cast = lang.uint_type(8);

    if field.wire_bits == 8 && field.bit_offset.is_multiple_of(8) {
        let byte = field.bit_offset / 8;
        output.push_str(&format!(
            "    {} = ({cast})({value} & 0xFFu);\n",
            lang.buf_byte(&byte.to_string())
        ));
        return;
    }

    if field.wire_bits == 16 && field.bit_offset.is_multiple_of(8) {
        let byte = field.bit_offset / 8;
        let next = byte + 1;
        output.push_str(&format!(
            "    {} = ({cast})(({value} >> 8) & 0xFFu);\n",
            lang.buf_byte(&byte.to_string())
        ));
        output.push_str(&format!(
            "    {} = ({cast})({value} & 0xFFu);\n",
            lang.buf_byte(&next.to_string())
        ));
        return;
    }

    let shift = 8 - (field.bit_offset % 8) - field.wire_bits;
    let byte = field.bit_offset / 8;
    output.push_str(&format!(
        "    {} |= ({cast})(({value} & 0x{mask:X}u) << {shift});\n",
        lang.buf_byte(&byte.to_string())
    ));
}

pub fn emit_decode_field(output: &mut String, lang: &Lang, field: &WireField, output_prefix: &str) {
    if field_is_spanning(field) {
        emit_decode_spanning_field(output, lang, field, output_prefix);
        return;
    }

    if field.bit_offset.is_multiple_of(8)
        && field.wire_bits.is_multiple_of(8)
        && field.wire_bits > 16
    {
        emit_decode_aligned_bytes(output, lang, field, output_prefix);
        return;
    }

    let mask = (1u64 << field.wire_bits) - 1;
    let host = lang.host_type(&field.host_type);
    let target = lang.member(output_prefix, &field.name);

    if field.wire_bits == 8 && field.bit_offset.is_multiple_of(8) {
        let byte = field.bit_offset / 8;
        let rhs = lang.typed_from_wire(
            &host,
            &format!("{} & 0xFFu", lang.buf_byte(&byte.to_string())),
        );
        output.push_str(&format!("    {target} = {rhs};\n"));
        return;
    }

    if field.wire_bits == 16 && field.bit_offset.is_multiple_of(8) {
        let byte = field.bit_offset / 8;
        let next = byte + 1;
        let u16 = lang.uint_type(16);
        let raw = format!(
            "(({u16}){} << 8) | {}",
            lang.buf_byte(&byte.to_string()),
            lang.buf_byte(&next.to_string())
        );
        let rhs = lang.typed_from_wire(&host, &raw);
        output.push_str(&format!("    {target} = {rhs};\n"));
        return;
    }

    let shift = 8 - (field.bit_offset % 8) - field.wire_bits;
    let byte = field.bit_offset / 8;
    let raw = format!(
        "(({}) >> {shift}) & 0x{mask:X}u",
        lang.buf_byte(&byte.to_string())
    );
    let rhs = lang.typed_from_wire(&host, &raw);
    output.push_str(&format!("    {target} = {rhs};\n"));
}

fn wire_value(lang: &Lang, accessor: &str) -> String {
    if accessor == "0" {
        "0".to_string()
    } else {
        match lang.kind {
            LangKind::C => accessor.to_string(),
            LangKind::Cpp => format!("static_cast<std::uint64_t>({accessor})"),
        }
    }
}

fn field_is_spanning(field: &WireField) -> bool {
    if field.bit_offset.is_multiple_of(8) && field.wire_bits.is_multiple_of(8) {
        return false;
    }
    (field.bit_offset % 8) + field.wire_bits > 8
}

fn emit_encode_spanning_field(output: &mut String, lang: &Lang, field: &WireField, accessor: &str) {
    let start = field.bit_offset;
    let bits = field.wire_bits;
    let u64 = lang.uint_type(64);
    let u32 = lang.uint_type(32);
    let u8 = lang.uint_type(8);
    output.push_str(&format!(
        "    {{\n        {u64} __bits = ({u64})({accessor}) & ((1ull << {bits}) - 1);\n        for ({u32} __i = 0; __i < {bits}; ++__i) {{\n            {u32} __bit = {start}u + __i;\n            {u32} __byte = __bit / 8u;\n            {u32} __pos = 7u - (__bit % 8u);\n            if ((__bits >> ({bits}u - 1u - __i)) & 1ull)\n                {} |= ({u8})(1u << __pos);\n        }}\n    }}\n",
        lang.buf_byte("__byte")
    ));
}

fn emit_decode_spanning_field(
    output: &mut String,
    lang: &Lang,
    field: &WireField,
    output_prefix: &str,
) {
    let start = field.bit_offset;
    let bits = field.wire_bits;
    let mask = (1u64 << bits) - 1;
    let temp = format!("__mek_{}", field.name);
    let host = lang.host_type(&field.host_type);
    let target = lang.member(output_prefix, &field.name);
    let u64 = lang.uint_type(64);
    let u32 = lang.uint_type(32);
    output.push_str(&format!(
        "    {{\n        {u64} {temp} = 0;\n        for ({u32} __i = 0; __i < {bits}; ++__i) {{\n            {u32} __bit = {start}u + __i;\n            {u32} __byte = __bit / 8u;\n            {u32} __pos = 7u - (__bit % 8u);\n            if ({} & (1u << __pos))\n                {temp} |= 1ull << ({bits}u - 1u - __i);\n        }}\n        {target} = ({host})({temp} & 0x{mask:X}ull);\n    }}\n",
        lang.buf_byte("__byte")
    ));
}

fn emit_encode_aligned_bytes(output: &mut String, lang: &Lang, field: &WireField, accessor: &str) {
    let byte = field.bit_offset / 8;
    let bytes = field.wire_bits / 8;
    let cast = lang.uint_type(8);
    for index in 0..bytes {
        let shift = (bytes - 1 - index) * 8;
        output.push_str(&format!(
            "    {} = ({cast})(({accessor} >> {shift}) & 0xFFu);\n",
            lang.buf_byte(&(byte + index).to_string())
        ));
    }
}

fn emit_decode_aligned_bytes(
    output: &mut String,
    lang: &Lang,
    field: &WireField,
    output_prefix: &str,
) {
    let byte = field.bit_offset / 8;
    let bytes = field.wire_bits / 8;
    let temp = format!("__mek_{}", field.name);
    let host = lang.host_type(&field.host_type);
    let target = lang.member(output_prefix, &field.name);
    output.push_str(&format!("    {host} {temp} = 0;\n"));
    for index in 0..bytes {
        output.push_str(&format!(
            "    {temp} = ({host})(({temp} << 8) | {});\n",
            lang.buf_byte(&(byte + index).to_string())
        ));
    }
    output.push_str(&format!(
        "    {target} = {};\n",
        lang.typed_from_wire(&host, &temp)
    ));
}

pub fn emit_decode_computed(output: &mut String, lang: &Lang, field: &WireField) {
    if field.wire_bits == 16 && field.bit_offset.is_multiple_of(8) {
        let byte = field.bit_offset / 8;
        let next = byte + 1;
        let u16 = lang.uint_type(16);
        match lang.kind {
            LangKind::C => {
                output.push_str(&format!(
                    "    *{} = ({u16})((({u16}){} << 8) | {});\n",
                    field.name,
                    lang.buf_byte(&byte.to_string()),
                    lang.buf_byte(&next.to_string())
                ));
            }
            LangKind::Cpp => {
                output.push_str(&format!(
                    "    {} = ({u16})((({u16}){} << 8) | {});\n",
                    field.name,
                    lang.buf_byte(&byte.to_string()),
                    lang.buf_byte(&next.to_string())
                ));
            }
        }
    }
}
