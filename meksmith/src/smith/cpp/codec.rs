use crate::analyze::CheckedFile;
use crate::smith::c::layout::{StructureLayout, WireField, layouts_for_file};
use crate::smith::codegen::lang::Lang;
use crate::smith::codegen::wire::{emit_decode_computed, emit_layout_encode_body};

use super::emit::fn_name;

pub fn emit_codecs(output: &mut String, file: &CheckedFile, namespace: &str) {
    let lang = Lang::cpp(namespace);

    for layout in layouts_for_file(file) {
        emit_structure_codec(output, &layout, &lang);
    }
}

fn emit_structure_codec(output: &mut String, layout: &StructureLayout, lang: &Lang) {
    let wire_bytes = layout.wire_bits / 8;
    let encode_name = fn_name(&layout.name, "encode");
    let decode_name = fn_name(&layout.name, "decode");

    let computed_fields: Vec<_> = layout
        .fields
        .iter()
        .filter(|field| field.is_computed)
        .collect();

    let encode_extra: Vec<String> = computed_fields
        .iter()
        .map(|field| format!("std::uint16_t {}", field.name))
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
    output.push_str(&format!(
        "    if (buf.size() < {wire_bytes}) {fail}\n",
        fail = lang.ret_fail()
    ));
    output.push_str(&lang.buf_zero_prefix(wire_bytes));
    emit_layout_encode_body(output, lang, &layout.fields, "in");
    output.push_str(&format!(
        "    {finish};\n}}\n\n",
        finish = lang.finish_encode(&wire_bytes.to_string())
    ));

    let decode_extra: Vec<String> = computed_fields
        .iter()
        .map(|field| format!("std::uint16_t& {}", field.name))
        .collect();

    output.push_str(&format!(
        "{} {{\n",
        lang.decode_fn(&decode_name, &layout.name, &decode_extra, false)
    ));
    output.push_str(&format!(
        "    if (buf.size() < {wire_bytes}) {fail}\n",
        fail = lang.ret_fail()
    ));

    for field in &layout.fields {
        if field.is_computed {
            emit_decode_computed(output, lang, field);
        } else {
            emit_decode_field(output, lang, field, "out");
        }
    }

    output.push_str(&format!("    {ok}\n}}\n\n", ok = lang.ret_ok_zero()));
}

fn emit_decode_field(output: &mut String, lang: &Lang, field: &WireField, output_prefix: &str) {
    crate::smith::codegen::wire::emit_decode_field(output, lang, field, output_prefix);
}
