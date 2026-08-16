use std::collections::HashSet;

use crate::analyze::CheckedFile;
use crate::frontend::ast::{Attribute, ChoiceArm, ChoicePattern, Expr, Item, TypeExpr};
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
            output.push_str("typedef enum {\n");
            for variant in &enumerated.variants {
                for (variant_name, value) in
                    expand_enum_variant(&variant.node.name, &variant.node.value)
                {
                    output.push_str(&format!(
                        "    {}_{} = {},\n",
                        enumerated.name, variant_name, value
                    ));
                }
            }
            output.push_str(&format!("}} {};\n\n", enumerated.name));
        }
        Item::TypeAlias(alias) => {
            if let Some(c_ty) = map_type_expr(&alias.ty) {
                output.push_str(&format!("typedef {} {};\n\n", c_ty, alias.name));
            }
        }
        Item::Structure(structure) => {
            output.push_str("typedef struct {\n");
            for field in &structure.fields {
                if field.node.attributes.contains(&Attribute::Computed) {
                    continue;
                }
                if let Some(c_ty) = map_type_expr(&field.node.ty) {
                    output.push_str(&format!("    {} {};\n", c_ty, field.node.name));
                }
            }
            output.push_str(&format!("}} {};\n\n", structure.name));
        }
        Item::Choice(choice) => {
            output.push_str("typedef struct {\n");
            let disc_ty = choice
                .params
                .iter()
                .find(|param| param.name == choice.discriminant)
                .and_then(|param| map_type_name(&param.ty))
                .or_else(|| map_type_name(&choice.discriminant));
            if let Some(disc_ty) = disc_ty {
                output.push_str(&format!("    {} tag;\n", disc_ty));
            }
            output.push_str("    union {\n");
            for (index, arm_name) in unique_choice_arm_names(&choice.arms) {
                let arm = &choice.arms[index];
                if let Some(c_ty) = map_type_expr(&arm.node.ty) {
                    output.push_str(&format!("        {} {};\n", c_ty, arm_name));
                }
            }
            output.push_str(&format!("    }} body;\n}} {};\n\n", choice.name));
        }
    }
}

fn unique_choice_arm_names(
    arms: &[crate::frontend::span::Spanned<ChoiceArm>],
) -> Vec<(usize, String)> {
    let mut used = HashSet::new();
    arms.iter()
        .enumerate()
        .map(|(index, arm)| {
            let base = pattern_arm_name(&arm.node.pattern);
            let mut name = base.clone();
            let mut suffix = 2u32;
            while used.contains(&name) {
                name = format!("{base}_{suffix}");
                suffix += 1;
            }
            used.insert(name.clone());
            (index, name)
        })
        .collect()
}

fn pattern_arm_name(pattern: &ChoicePattern) -> String {
    match pattern {
        ChoicePattern::Path { variant, .. } => variant.clone(),
        ChoicePattern::Wildcard => "catch_all".into(),
        ChoicePattern::Integer(lit) => format!("arm_{}", literal_from_token(lit)),
    }
}

fn literal_from_token(lit: &crate::frontend::token::Literal) -> String {
    match lit {
        crate::frontend::token::Literal::Integer { value, .. } => value.to_string(),
        other => format!("{other:?}"),
    }
}

pub fn map_type_expr(ty: &TypeExpr) -> Option<String> {
    match ty {
        TypeExpr::Null => Some("uint8_t".into()),
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
                return Some(format!("struct {{ {} *data; size_t len; }}", element));
            }
            if name == "GreedyArray" && generics.len() == 1 {
                let element = map_type_expr(&TypeExpr::Named {
                    name: type_name_from_expr(&generics[0])?,
                    generics: Vec::new(),
                    args: Vec::new(),
                })?;
                return Some(format!("struct {{ {} *data; size_t len; }}", element));
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

pub fn map_type_name(name: &str) -> Option<String> {
    Some(
        match name {
            "u8" | "byte" => "uint8_t",
            "u16" => "uint16_t",
            "u32" => "uint32_t",
            "u64" => "uint64_t",
            "i8" => "int8_t",
            "i16" => "int16_t",
            "i32" => "int32_t",
            "i64" => "int64_t",
            "boolean" => "uint8_t",
            other => other,
        }
        .to_string(),
    )
}
