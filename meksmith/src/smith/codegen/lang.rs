use crate::smith::c::emit::to_snake_case;

/// One parameter per line: `(\n    a,\n    b\n)`.
pub fn multiline_params(params: &[impl AsRef<str>]) -> String {
    if params.is_empty() {
        "()".to_string()
    } else {
        let body = params
            .iter()
            .map(|param| format!("    {}", param.as_ref()))
            .collect::<Vec<_>>()
            .join(",\n");
        format!("(\n{body}\n)")
    }
}

/// Multiline call when there is more than one argument.
pub fn multiline_call(name: &str, args: &[impl AsRef<str>]) -> String {
    if args.is_empty() {
        format!("{name}()")
    } else if args.len() == 1 {
        format!("{name}({})", args[0].as_ref())
    } else {
        let body = args
            .iter()
            .map(|arg| format!("        {}", arg.as_ref()))
            .collect::<Vec<_>>()
            .join(",\n");
        format!("{name}(\n{body}\n    )")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LangKind {
    C,
    Cpp,
}

#[derive(Debug, Clone)]
pub struct Lang {
    pub kind: LangKind,
    /// C function prefix (`ecpri`) or C++ namespace name.
    pub id: String,
}

impl Lang {
    pub fn c(prefix: &str) -> Self {
        Self {
            kind: LangKind::C,
            id: prefix.to_string(),
        }
    }

    pub fn cpp(namespace: &str) -> Self {
        Self {
            kind: LangKind::Cpp,
            id: namespace.to_string(),
        }
    }

    pub fn open_scope(&self, output: &mut String) {
        if self.kind == LangKind::Cpp {
            output.push_str(&format!("namespace {} {{\n\n", self.id));
        }
    }

    pub fn close_scope(&self, output: &mut String) {
        if self.kind == LangKind::Cpp {
            output.push_str(&format!("}} // namespace {}\n", self.id));
        }
    }

    pub fn preamble(&self, output: &mut String) {
        match self.kind {
            LangKind::C => {
                output.push_str("#include <stdint.h>\n#include <stddef.h>\n\n");
            }
            LangKind::Cpp => {
                output.push_str(
                    "#pragma once\n\
#include <version>\n\
#if !defined(__cpp_lib_expected) || __cpp_lib_expected < 202202L\n\
#error \"meksmith C++ backend requires C++23 <expected> (compile with -std=c++23)\"\n\
#endif\n\
#include <algorithm>\n\
#include <cstddef>\n\
#include <cstdint>\n\
#include <cstring>\n\
#include <expected>\n\
#include <span>\n\
#include <utility>\n\
#include <variant>\n\
#include <vector>\n\n",
                );
            }
        }
    }

    pub fn codec_preamble(&self, output: &mut String) {
        match self.kind {
            LangKind::C => output.push_str("#include <string.h>\n\n"),
            LangKind::Cpp => output.push_str("using mek_wire_err = int;\n\n"),
        }
    }

    pub fn fn_name(&self, type_name: &str, suffix: &str) -> String {
        let snake = to_snake_case(type_name);
        match self.kind {
            LangKind::C => format!("{}_{snake}_{suffix}", self.id),
            LangKind::Cpp => format!("{snake}_{suffix}"),
        }
    }

    pub fn ret_fail(&self) -> &'static str {
        match self.kind {
            LangKind::C => "return -1;",
            LangKind::Cpp => "return std::unexpected(-1);",
        }
    }

    pub fn ret_ok_zero(&self) -> &'static str {
        match self.kind {
            LangKind::C => "return 0;",
            LangKind::Cpp => "return {};",
        }
    }

    pub fn finish_encode(&self, len_var: &str) -> String {
        match self.kind {
            LangKind::C => format!("*out_len = {len_var};\n    return 0;"),
            LangKind::Cpp => format!("return {len_var};"),
        }
    }

    pub fn input_type(&self, ty: &str) -> String {
        match self.kind {
            LangKind::C => format!("const {ty} *in"),
            LangKind::Cpp => format!("const {ty}& in"),
        }
    }

    pub fn output_type(&self, ty: &str) -> String {
        match self.kind {
            LangKind::C => format!("{ty} *out"),
            LangKind::Cpp => format!("{ty}& out"),
        }
    }

    pub fn member(&self, base: &str, field: &str) -> String {
        match self.kind {
            LangKind::C => format!("{base}->{field}"),
            LangKind::Cpp => format!("{base}.{field}"),
        }
    }

    pub fn vec_len(&self, access: &str) -> String {
        match self.kind {
            LangKind::C => format!("{access}.len"),
            LangKind::Cpp => format!("{access}.size()"),
        }
    }

    pub fn vec_data(&self, access: &str) -> String {
        match self.kind {
            LangKind::C => format!("{access}.data"),
            LangKind::Cpp => format!("{access}.data()"),
        }
    }

    pub fn set_vec_len(&self, access: &str, len_expr: &str) -> String {
        match self.kind {
            LangKind::C => format!("{access}.len = {len_expr};"),
            LangKind::Cpp => format!("{access}.resize({len_expr});"),
        }
    }

    pub fn null_check_data(&self, access: &str) -> Option<String> {
        match self.kind {
            LangKind::C => Some(format!(
                "if ({data} == NULL) {fail}",
                data = self.vec_data(access),
                fail = self.ret_fail()
            )),
            LangKind::Cpp => None,
        }
    }

    pub fn host_type(&self, ty: &str) -> String {
        match self.kind {
            LangKind::C => ty.to_string(),
            LangKind::Cpp => match ty {
                "uint8_t" => "std::uint8_t".into(),
                "uint16_t" => "std::uint16_t".into(),
                "uint32_t" => "std::uint32_t".into(),
                "uint64_t" => "std::uint64_t".into(),
                "int8_t" => "std::int8_t".into(),
                "int16_t" => "std::int16_t".into(),
                "int32_t" => "std::int32_t".into(),
                "int64_t" => "std::int64_t".into(),
                other => other.to_string(),
            },
        }
    }

    pub fn uint_type(&self, bits: u32) -> &'static str {
        match self.kind {
            LangKind::C => match bits {
                8 => "uint8_t",
                16 => "uint16_t",
                32 => "uint32_t",
                64 => "uint64_t",
                _ => "uint32_t",
            },
            LangKind::Cpp => match bits {
                8 => "std::uint8_t",
                16 => "std::uint16_t",
                32 => "std::uint32_t",
                64 => "std::uint64_t",
                _ => "std::uint32_t",
            },
        }
    }

    pub fn buf_zero_prefix(&self, bytes: u32) -> String {
        match self.kind {
            LangKind::C => format!("for (size_t i = 0; i < {bytes}; ++i) buf[i] = 0;\n"),
            LangKind::Cpp => {
                format!("std::fill(buf.begin(), buf.begin() + {bytes}, std::uint8_t{{0}});\n")
            }
        }
    }

    pub fn buf_byte(&self, index: &str) -> String {
        match self.kind {
            LangKind::C => format!("buf[{index}]"),
            LangKind::Cpp => format!("buf[{index}]"),
        }
    }

    pub fn choice_body_encode(&self, payload_type: &str, member: &str) -> String {
        match self.kind {
            LangKind::C => format!("&in->body.{member}"),
            LangKind::Cpp => format!("std::get<{payload_type}>(in.body)"),
        }
    }

    pub fn choice_body_decode(&self, payload_type: &str, member: &str) -> String {
        match self.kind {
            LangKind::C => format!("&out->body.{member}"),
            LangKind::Cpp => format!("std::get<{payload_type}>(out.body)"),
        }
    }

    pub fn choice_set_tag(&self) -> String {
        match self.kind {
            LangKind::C => "out->tag = tag;\n".into(),
            LangKind::Cpp => "out.tag = tag;\n".into(),
        }
    }

    pub fn enum_case(&self, enum_case: &str) -> String {
        match self.kind {
            LangKind::C => enum_case.to_string(),
            LangKind::Cpp => {
                if let Some(pos) = enum_case.find('_') {
                    let (ty, rest) = enum_case.split_at(pos);
                    format!("{ty}::{}", &rest[1..])
                } else {
                    enum_case.to_string()
                }
            }
        }
    }

    pub fn size_type(&self) -> &'static str {
        match self.kind {
            LangKind::C => "size_t",
            LangKind::Cpp => "std::size_t",
        }
    }

    pub fn encode_fn(
        &self,
        name: &str,
        ty: &str,
        extra_params: &[String],
        _has_computed_encode: bool,
    ) -> String {
        let mut params = vec![self.input_type(ty)];
        params.extend(extra_params.iter().cloned());
        match self.kind {
            LangKind::C => {
                params.extend([
                    "uint8_t *buf".into(),
                    "size_t cap".into(),
                    "size_t *out_len".into(),
                ]);
                format!("int {name}{}", multiline_params(&params))
            }
            LangKind::Cpp => {
                params.push("std::span<std::uint8_t> buf".into());
                format!(
                    "[[nodiscard]] inline std::expected<std::size_t, int> {name}{}",
                    multiline_params(&params)
                )
            }
        }
    }

    pub fn decode_fn(
        &self,
        name: &str,
        ty: &str,
        extra_params: &[String],
        payload_size: bool,
    ) -> String {
        match self.kind {
            LangKind::C => {
                let mut params = vec!["const uint8_t *buf".into(), "size_t len".into()];
                if payload_size {
                    params.push("uint16_t payload_size".into());
                }
                params.extend(extra_params.iter().cloned());
                params.push(self.output_type(ty));
                format!("int {name}{}", multiline_params(&params))
            }
            LangKind::Cpp => {
                let mut params = vec!["std::span<const std::uint8_t> buf".into()];
                if payload_size {
                    params.push("std::uint16_t payload_size".into());
                }
                params.extend(extra_params.iter().cloned());
                params.push(self.output_type(ty));
                format!(
                    "[[nodiscard]] inline std::expected<void, int> {name}{}",
                    multiline_params(&params)
                )
            }
        }
    }

    pub fn choice_encode_sig(&self, name: &str, choice_ty: &str, disc_ty: &str) -> String {
        match self.kind {
            LangKind::C => format!(
                "int {name}{}",
                multiline_params(&[
                    format!("const {choice_ty} *in"),
                    format!("{disc_ty} tag"),
                    "uint8_t *buf".into(),
                    "size_t cap".into(),
                    "size_t *out_len".into(),
                ])
            ),
            LangKind::Cpp => format!(
                "[[nodiscard]] inline std::expected<std::size_t, int> {name}{}",
                multiline_params(&[
                    format!("const {choice_ty}& in"),
                    format!("{disc_ty} tag"),
                    "std::span<std::uint8_t> buf".into(),
                ])
            ),
        }
    }

    pub fn choice_decode_sig(&self, name: &str, choice_ty: &str, disc_ty: &str) -> String {
        match self.kind {
            LangKind::C => format!(
                "int {name}{}",
                multiline_params(&[
                    "const uint8_t *buf".into(),
                    "size_t len".into(),
                    format!("{disc_ty} tag"),
                    "uint16_t payload_size".into(),
                    format!("{choice_ty} *out"),
                ])
            ),
            LangKind::Cpp => format!(
                "[[nodiscard]] inline std::expected<void, int> {name}{}",
                multiline_params(&[
                    "std::span<const std::uint8_t> buf".into(),
                    format!("{disc_ty} tag"),
                    "std::uint16_t payload_size".into(),
                    format!("{choice_ty}& out"),
                ])
            ),
        }
    }

    /// Decode that reports total bytes consumed (length-prefixed layouts, PDU messages).
    pub fn decode_fn_consumed(&self, name: &str, ty: &str) -> String {
        match self.kind {
            LangKind::C => format!(
                "int {name}{}",
                multiline_params(&[
                    "const uint8_t *buf".into(),
                    "size_t len".into(),
                    self.output_type(ty),
                    "size_t *consumed".into(),
                ])
            ),
            LangKind::Cpp => format!(
                "[[nodiscard]] inline std::expected<std::size_t, int> {name}{}",
                multiline_params(&[
                    "std::span<const std::uint8_t> buf".into(),
                    self.output_type(ty),
                ])
            ),
        }
    }

    pub fn fn_call(&self, name: &str, args: &[String]) -> String {
        multiline_call(name, args)
    }

    pub fn cap_check(&self, needed: &str) -> String {
        match self.kind {
            LangKind::C => format!("if (cap < ({needed})) {fail}", fail = self.ret_fail()),
            LangKind::Cpp => format!(
                "if (buf.size() < ({needed})) {fail}",
                fail = self.ret_fail()
            ),
        }
    }

    pub fn len_check(&self, needed: &str) -> String {
        match self.kind {
            LangKind::C => format!("if (len < ({needed})) {fail}", fail = self.ret_fail()),
            LangKind::Cpp => format!(
                "if (buf.size() < ({needed})) {fail}",
                fail = self.ret_fail()
            ),
        }
    }

    pub fn subspan(&self, offset: &str) -> String {
        match self.kind {
            LangKind::C => format!("buf + {offset}"),
            LangKind::Cpp => format!("buf.subspan({offset})"),
        }
    }

    pub fn subspan_cap(&self, offset: &str) -> String {
        match self.kind {
            LangKind::C => format!("cap - {offset}"),
            LangKind::Cpp => format!("buf.subspan({offset})"),
        }
    }

    pub fn typed_from_wire(&self, host: &str, expr: &str) -> String {
        match self.kind {
            LangKind::C => format!("({host})({expr})"),
            LangKind::Cpp => format!("static_cast<{host}>({expr})"),
        }
    }
}
