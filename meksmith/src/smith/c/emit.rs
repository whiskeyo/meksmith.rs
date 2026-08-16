use crate::analyze::CheckedFile;
use crate::frontend::ast::{Choice, ChoiceArm, ChoicePattern, Item, Structure, TypeExpr};
use crate::smith::codegen::enum_variants::expanded_prefixed_enum_cases;

#[derive(Debug, Clone)]
pub struct ChoiceArmEmit {
    pub enum_case: String,
    pub union_member: String,
    pub payload_type: String,
}

pub fn choice_arm_emits(choice: &Choice, file: &CheckedFile) -> Vec<ChoiceArmEmit> {
    let mut used = std::collections::HashSet::new();
    let mut emits = Vec::new();

    for arm in &choice.arms {
        let union_member = pattern_union_member(&arm.node.pattern, &mut used);
        let payload_type = arm_type_name(&arm.node);
        for enum_case in pattern_enum_cases(&arm.node.pattern, &choice.discriminant, file) {
            emits.push(ChoiceArmEmit {
                enum_case,
                union_member: union_member.clone(),
                payload_type: payload_type.clone(),
            });
        }
    }

    emits
}

fn pattern_enum_cases(
    pattern: &ChoicePattern,
    discriminant_ty: &str,
    file: &CheckedFile,
) -> Vec<String> {
    match pattern {
        ChoicePattern::Path { ty, variant } => {
            let enum_name = if ty.is_empty() {
                discriminant_ty.to_string()
            } else {
                ty.clone()
            };
            if let Some(value) = find_enum_variant_value(file, &enum_name, variant) {
                expanded_prefixed_enum_cases(&enum_name, variant, &value)
            } else {
                vec![format!("{enum_name}_{variant}")]
            }
        }
        ChoicePattern::Wildcard => vec!["/* wildcard */".into()],
        ChoicePattern::Integer(lit) => vec![format!(
            "arm_{}",
            match lit {
                crate::frontend::token::Literal::Integer { value, .. } => value.to_string(),
                other => format!("{other:?}"),
            }
        )],
    }
}

fn find_enum_variant_value(
    file: &CheckedFile,
    enum_name: &str,
    variant_name: &str,
) -> Option<crate::frontend::ast::EnumValue> {
    let item = file.file.items.iter().find_map(|item| match &item.node {
        Item::Enumerated(enumerated) if enumerated.name == enum_name => Some(enumerated),
        _ => None,
    })?;
    item.variants
        .iter()
        .find(|variant| variant.node.name == variant_name)
        .map(|variant| variant.node.value.clone())
}

fn pattern_union_member(
    pattern: &ChoicePattern,
    used: &mut std::collections::HashSet<String>,
) -> String {
    let base = match pattern {
        ChoicePattern::Path { variant, .. } => variant.clone(),
        ChoicePattern::Wildcard => "catch_all".into(),
        ChoicePattern::Integer(lit) => format!(
            "arm_{}",
            match lit {
                crate::frontend::token::Literal::Integer { value, .. } => value.to_string(),
                other => format!("{other:?}"),
            }
        ),
    };
    let mut name = base.clone();
    let mut suffix = 2u32;
    while used.contains(&name) {
        name = format!("{base}_{suffix}");
        suffix += 1;
    }
    used.insert(name.clone());
    name
}

fn arm_type_name(arm: &ChoiceArm) -> String {
    match &arm.ty {
        TypeExpr::Named { name, .. } => name.clone(),
        TypeExpr::Null => "null".into(),
    }
}

pub fn find_structure<'a>(file: &'a CheckedFile, name: &str) -> Option<&'a Structure> {
    file.file.items.iter().find_map(|item| match &item.node {
        Item::Structure(structure) if structure.name == name => Some(structure),
        _ => None,
    })
}

pub fn find_choice<'a>(file: &'a CheckedFile, name: &str) -> Option<&'a Choice> {
    file.file.items.iter().find_map(|item| match &item.node {
        Item::Choice(choice) if choice.name == name => Some(choice),
        _ => None,
    })
}

pub fn to_snake_case(name: &str) -> String {
    let mut result = String::new();
    for (i, ch) in name.chars().enumerate() {
        if ch.is_ascii_uppercase() && i > 0 {
            let prev = name.chars().nth(i - 1).unwrap();
            let next = name.chars().nth(i + 1);
            if prev.is_ascii_lowercase()
                || (prev.is_ascii_uppercase()
                    && next.map(|c| c.is_ascii_lowercase()).unwrap_or(false))
            {
                result.push('_');
            }
        }
        result.push(ch.to_ascii_lowercase());
    }
    result
}

pub fn fn_name(prefix: &str, type_name: &str, suffix: &str) -> String {
    format!("{prefix}_{}_{suffix}", to_snake_case(type_name))
}
