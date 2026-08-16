use crate::analyze::CheckedFile;
use crate::smith::codegen::lang::Lang;

use super::layout::{StructureLayout, WireField, layouts_for_file};

pub(crate) fn emit_layout_encode_body(
    output: &mut String,
    layout: &StructureLayout,
    input_accessor: &str,
) {
    for field in &layout.fields {
        if field.is_computed {
            emit_encode_field(output, field, &field.name);
            continue;
        }
        let accessor = if field.is_unused {
            "0".into()
        } else {
            format!("{input_accessor}->{}", field.name)
        };
        emit_encode_field(output, field, &accessor);
    }
}

pub(crate) fn emit_layout_decode_body(
    output: &mut String,
    layout: &StructureLayout,
    output_accessor: &str,
) {
    for field in &layout.fields {
        if field.is_computed {
            continue;
        }
        emit_decode_field(output, field, output_accessor);
    }
}

pub fn emit_codecs(output: &mut String, file: &CheckedFile, prefix: &str) {
    for layout in layouts_for_file(file) {
        emit_structure_codec(output, &layout, prefix);
    }
}

fn emit_structure_codec(output: &mut String, layout: &StructureLayout, prefix: &str) {
    let lang = Lang::c(prefix);
    let wire_bytes = layout.wire_bits / 8;
    let snake = to_snake_case(&layout.name);
    let encode_name = format!("{prefix}_{snake}_encode");
    let decode_name = format!("{prefix}_{snake}_decode");

    let computed_fields: Vec<_> = layout
        .fields
        .iter()
        .filter(|field| field.is_computed)
        .collect();

    let encode_extra: Vec<String> = computed_fields
        .iter()
        .map(|field| format!("uint16_t {}", field.name))
        .collect();

    output.push_str(&format!(
        "{} {{\n",
        lang.encode_fn(
            &encode_name,
            &layout.name,
            &encode_extra,
            !computed_fields.is_empty()
        )
    ));
    output.push_str(&format!("    if (cap < {wire_bytes}) return -1;\n"));
    output.push_str(&format!(
        "    for (size_t i = 0; i < {wire_bytes}; ++i) buf[i] = 0;\n"
    ));

    for field in &layout.fields {
        if field.is_computed {
            emit_encode_field(output, field, &field.name);
            continue;
        }
        let accessor = if field.is_unused {
            "0".into()
        } else {
            format!("in->{}", field.name)
        };
        emit_encode_field(output, field, &accessor);
    }

    output.push_str(&format!("    *out_len = {wire_bytes};\n"));
    output.push_str("    return 0;\n");
    output.push_str("}\n\n");

    let decode_extra: Vec<String> = computed_fields
        .iter()
        .map(|field| format!("uint16_t *{}", field.name))
        .collect();

    output.push_str(&format!(
        "{} {{\n",
        lang.decode_fn(&decode_name, &layout.name, &decode_extra, false)
    ));
    output.push_str(&format!("    if (len < {wire_bytes}) return -1;\n"));

    for field in &layout.fields {
        if field.is_computed {
            emit_decode_computed(output, field);
        } else {
            emit_decode_field(output, field, "out");
        }
    }

    output.push_str("    return 0;\n");
    output.push_str("}\n\n");
}

fn emit_encode_field(output: &mut String, field: &WireField, accessor: &str) {
    if field_is_spanning(field) {
        emit_encode_spanning_field(output, field, accessor);
        return;
    }

    if field.bit_offset.is_multiple_of(8)
        && field.wire_bits.is_multiple_of(8)
        && field.wire_bits > 8
    {
        emit_encode_aligned_bytes(output, field, accessor);
        return;
    }

    let mask = ((1u64 << field.wire_bits) - 1) as u32;

    if field.wire_bits == 8 && field.bit_offset.is_multiple_of(8) {
        let byte = field.bit_offset / 8;
        output.push_str(&format!(
            "    buf[{byte}] = (uint8_t)({accessor} & 0xFFu);\n"
        ));
        return;
    }

    if field.wire_bits == 16 && field.bit_offset.is_multiple_of(8) {
        let byte = field.bit_offset / 8;
        let next = byte + 1;
        output.push_str(&format!(
            "    buf[{byte}] = (uint8_t)(({accessor} >> 8) & 0xFFu);\n"
        ));
        output.push_str(&format!(
            "    buf[{next}] = (uint8_t)({accessor} & 0xFFu);\n",
        ));
        return;
    }

    let shift = 8 - (field.bit_offset % 8) - field.wire_bits;
    let byte = field.bit_offset / 8;
    output.push_str(&format!(
        "    buf[{byte}] |= (uint8_t)(({accessor} & 0x{mask:X}u) << {shift});\n"
    ));
}

fn emit_decode_field(output: &mut String, field: &WireField, output_prefix: &str) {
    if field_is_spanning(field) {
        emit_decode_spanning_field(output, field, output_prefix);
        return;
    }

    if field.bit_offset.is_multiple_of(8)
        && field.wire_bits.is_multiple_of(8)
        && field.wire_bits > 16
    {
        emit_decode_aligned_bytes(output, field, output_prefix);
        return;
    }

    let mask = (1u64 << field.wire_bits) - 1;

    if field.wire_bits == 8 && field.bit_offset.is_multiple_of(8) {
        let byte = field.bit_offset / 8;
        output.push_str(&format!(
            "    {output_prefix}->{name} = ({ty})(buf[{byte}] & 0xFFu);\n",
            name = field.name,
            ty = field.host_type,
        ));
        return;
    }

    if field.wire_bits == 16 && field.bit_offset.is_multiple_of(8) {
        let byte = field.bit_offset / 8;
        let next = byte + 1;
        output.push_str(&format!(
            "    {output_prefix}->{name} = ({ty})(((uint16_t)buf[{byte}] << 8) | buf[{next}]);\n",
            name = field.name,
            ty = field.host_type,
        ));
        return;
    }

    let shift = 8 - (field.bit_offset % 8) - field.wire_bits;
    let byte = field.bit_offset / 8;
    output.push_str(&format!(
        "    {output_prefix}->{name} = ({ty})((buf[{byte}] >> {shift}) & 0x{mask:X}u);\n",
        name = field.name,
        ty = field.host_type,
    ));
}

fn field_is_spanning(field: &WireField) -> bool {
    if field.bit_offset.is_multiple_of(8) && field.wire_bits.is_multiple_of(8) {
        return false;
    }
    (field.bit_offset % 8) + field.wire_bits > 8
}

fn emit_encode_spanning_field(output: &mut String, field: &WireField, accessor: &str) {
    let start = field.bit_offset;
    let bits = field.wire_bits;
    output.push_str(&format!(
        "    {{\n        uint64_t __bits = (uint64_t)({accessor}) & ((1ull << {bits}) - 1);\n        for (uint32_t __i = 0; __i < {bits}; ++__i) {{\n            uint32_t __bit = {start}u + __i;\n            uint32_t __byte = __bit / 8u;\n            uint32_t __pos = 7u - (__bit % 8u);\n            if ((__bits >> ({bits}u - 1u - __i)) & 1ull)\n                buf[__byte] |= (uint8_t)(1u << __pos);\n        }}\n    }}\n"
    ));
}

fn emit_decode_spanning_field(output: &mut String, field: &WireField, output_prefix: &str) {
    let start = field.bit_offset;
    let bits = field.wire_bits;
    let mask = (1u64 << bits) - 1;
    let temp = format!("__mek_{}", field.name);
    output.push_str(&format!(
        "    {{\n        uint64_t {temp} = 0;\n        for (uint32_t __i = 0; __i < {bits}; ++__i) {{\n            uint32_t __bit = {start}u + __i;\n            uint32_t __byte = __bit / 8u;\n            uint32_t __pos = 7u - (__bit % 8u);\n            if (buf[__byte] & (1u << __pos))\n                {temp} |= 1ull << ({bits}u - 1u - __i);\n        }}\n        {output_prefix}->{name} = ({ty})({temp} & 0x{mask:X}ull);\n    }}\n",
        name = field.name,
        ty = field.host_type,
    ));
}

fn emit_encode_aligned_bytes(output: &mut String, field: &WireField, accessor: &str) {
    let byte = field.bit_offset / 8;
    let bytes = field.wire_bits / 8;
    for index in 0..bytes {
        let shift = (bytes - 1 - index) * 8;
        output.push_str(&format!(
            "    buf[{index}] = (uint8_t)(({accessor} >> {shift}) & 0xFFu);\n",
            index = byte + index,
        ));
    }
}

fn emit_decode_aligned_bytes(output: &mut String, field: &WireField, output_prefix: &str) {
    let byte = field.bit_offset / 8;
    let bytes = field.wire_bits / 8;
    let temp = format!("__mek_{}", field.name);
    output.push_str(&format!("    {ty} {temp} = 0;\n", ty = field.host_type));
    for index in 0..bytes {
        let shift = (bytes - 1 - index) * 8;
        output.push_str(&format!(
            "    {temp} = ({ty})(({temp} << 8) | buf[{index}]);\n",
            ty = field.host_type,
            index = byte + index,
        ));
        let _ = shift;
    }
    output.push_str(&format!(
        "    {output_prefix}->{name} = {temp};\n",
        name = field.name,
    ));
}

fn emit_decode_computed(output: &mut String, field: &WireField) {
    if field.wire_bits == 16 && field.bit_offset.is_multiple_of(8) {
        let byte = field.bit_offset / 8;
        let next = byte + 1;
        output.push_str(&format!(
            "    *{name} = (uint16_t)(((uint16_t)buf[{byte}] << 8) | buf[{next}]);\n",
            name = field.name,
        ));
    }
}

fn to_snake_case(name: &str) -> String {
    super::emit::to_snake_case(name)
}
