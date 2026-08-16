use std::collections::HashSet;

use crate::analyze::CheckedFile;
use crate::frontend::ast::{Attribute, Expr, Item, TypeExpr};
use crate::frontend::token::Literal;
use crate::smith::codegen::enum_variants::expand_enum_variant;

pub fn emit_types(output: &mut String, file: &CheckedFile) {
    for type_name in &file.emit_order {
        let Some(item) = file
            .file
            .items
            .iter()
            .find(|item| item.node.name() == type_name)
        else {
            continue;
        };
        emit_item(output, &item.node);
    }
}

fn emit_item(output: &mut String, item: &Item) {
    match item {
        Item::Enumerated(enumerated) => {
            let width = map_underlying_uint(Some(&enumerated.width_bits.node));
            output.push_str(&format!("enum class {} : {width} {{\n", enumerated.name));
            for variant in &enumerated.variants {
                for (variant_name, value) in
                    expand_enum_variant(&variant.node.name, &variant.node.value)
                {
                    output.push_str(&format!("    {variant_name} = {value},\n"));
                }
            }
            output.push_str("};\n\n");
        }
        Item::TypeAlias(alias) => {
            if let Some(cpp_ty) = map_type_expr(&alias.ty) {
                output.push_str(&format!("using {} = {};\n\n", alias.name, cpp_ty));
            }
        }
        Item::Structure(structure) => {
            output.push_str(&format!("struct {} {{\n", structure.name));
            for field in &structure.fields {
                if field.node.attributes.contains(&Attribute::Computed) {
                    continue;
                }
                if let Some(cpp_ty) = map_type_expr(&field.node.ty) {
                    output.push_str(&format!("    {} {};\n", cpp_ty, field.node.name));
                }
            }
            output.push_str("};\n\n");
        }
        Item::Choice(choice) => {
            let disc_ty = choice
                .params
                .iter()
                .find(|param| param.name == choice.discriminant)
                .and_then(|param| map_type_name(&param.ty))
                .or_else(|| map_type_name(&choice.discriminant))
                .unwrap_or_else(|| "std::uint32_t".into());

            output.push_str(&format!(
                "struct {} {{\n    {} tag;\n    std::variant<\n",
                choice.name, disc_ty
            ));
            let mut emitted = HashSet::new();
            for arm in &choice.arms {
                if let TypeExpr::Named { name, .. } = &arm.node.ty
                    && emitted.insert(name.clone())
                {
                    output.push_str(&format!("        {},\n", name));
                }
            }
            output.push_str("    > body;\n};\n\n");
        }
    }
}

fn map_type_expr(ty: &TypeExpr) -> Option<String> {
    match ty {
        TypeExpr::Null => Some("std::uint8_t".into()),
        TypeExpr::Named {
            name,
            generics,
            args,
        } => {
            if name == "DynamicArray" && generics.len() == 2 {
                let element = map_type_expr(&TypeExpr::Named {
                    name: type_name_from_expr(&generics[0])?,
                    generics: Vec::new(),
                    args: Vec::new(),
                })?;
                return Some(format!("std::vector<{element}>"));
            }
            if name == "GreedyArray" && generics.len() == 1 {
                let element = map_type_expr(&TypeExpr::Named {
                    name: type_name_from_expr(&generics[0])?,
                    generics: Vec::new(),
                    args: Vec::new(),
                })?;
                return Some(format!("std::vector<{element}>"));
            }
            if !args.is_empty() {
                return map_type_name(name);
            }
            map_type_name(name)
        }
    }
}

fn type_name_from_expr(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Ident(name) => Some(name.clone()),
        _ => None,
    }
}

fn map_type_name(name: &str) -> Option<String> {
    Some(
        match name {
            "u8" | "byte" => "std::uint8_t",
            "u16" => "std::uint16_t",
            "u32" => "std::uint32_t",
            "u64" => "std::uint64_t",
            "i8" => "std::int8_t",
            "i16" => "std::int16_t",
            "i32" => "std::int32_t",
            "i64" => "std::int64_t",
            "boolean" => "bool",
            other => other,
        }
        .to_string(),
    )
}

fn map_underlying_uint(width: Option<&Expr>) -> &'static str {
    match width.and_then(|expr| match expr {
        Expr::Literal(Literal::Integer { value, .. }) if *value > 0 => Some(*value as u32),
        _ => None,
    }) {
        Some(8) => "std::uint8_t",
        Some(16) => "std::uint16_t",
        Some(32) => "std::uint32_t",
        _ => "std::uint32_t",
    }
}
