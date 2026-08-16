use std::collections::{HashMap, HashSet};

use crate::diagnostic::{Diagnostic, DiagnosticCode, Emitter};
use crate::frontend::ast::{
    Attribute, BitOrder, Choice, ChoiceArm, ChoicePattern, Enumerated, Expr, Field, File, Item,
    Structure, TypeAlias, TypeExpr,
};
use crate::frontend::span::Spanned;
use crate::frontend::token::Literal;

pub mod checked;
pub mod const_expr;
pub mod graph;

pub use checked::CheckedFile;
pub use graph::DependencyGraph;

pub fn analyze(file: &File) -> Result<CheckedFile, crate::diagnostic::Diagnostics> {
    let mut emitter = Emitter::new();
    let mut symbols = SymbolTable::new();

    collect_definitions(file, &mut symbols, &mut emitter);
    check_definitions(file, &symbols, &mut emitter);
    let graph = DependencyGraph::build(file, &symbols, &mut emitter);
    let emit_order = graph.topological_order(&mut emitter);

    let diagnostics = emitter.into_diagnostics();
    if diagnostics.has_errors() {
        return Err(diagnostics);
    }

    Ok(CheckedFile {
        file: file.clone(),
        symbols,
        emit_order,
        graph,
    })
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    defs: HashMap<String, Definition>,
}

#[derive(Debug, Clone)]
pub enum Definition {
    Structure {
        span: crate::frontend::span::SimpleSpan,
        bit_order: BitOrder,
        params: Vec<String>,
        wire_bits: Option<u32>,
    },
    Enumerated {
        span: crate::frontend::span::SimpleSpan,
        width_bits: u32,
    },
    Choice {
        span: crate::frontend::span::SimpleSpan,
        discriminant: String,
        has_catch_all: bool,
    },
    TypeAlias {
        span: crate::frontend::span::SimpleSpan,
        wire_bits: Option<u32>,
        underlying: String,
    },
    Builtin {
        wire_bits: Option<u32>,
    },
}

impl SymbolTable {
    fn new() -> Self {
        let mut defs = HashMap::new();
        for (name, bits) in BUILTIN_TYPES {
            defs.insert(
                name.to_string(),
                Definition::Builtin {
                    wire_bits: Some(*bits),
                },
            );
        }
        defs.insert("boolean".into(), Definition::Builtin { wire_bits: Some(1) });
        defs.insert("null".into(), Definition::Builtin { wire_bits: None });
        for name in BUILTIN_GENERICS {
            defs.insert(name.to_string(), Definition::Builtin { wire_bits: None });
        }
        Self { defs }
    }

    pub fn get(&self, name: &str) -> Option<&Definition> {
        self.defs.get(name)
    }

    pub fn names(&self) -> impl Iterator<Item = &String> {
        self.defs.keys()
    }
}

const BUILTIN_TYPES: &[(&str, u32)] = &[
    ("u8", 8),
    ("u16", 16),
    ("u32", 32),
    ("u64", 64),
    ("i8", 8),
    ("i16", 16),
    ("i32", 32),
    ("i64", 64),
    ("byte", 8),
];

const BUILTIN_GENERICS: &[&str] = &[
    "DynamicArray",
    "GreedyArray",
    "StaticArray",
    "Optional",
    "BitSlice",
    "CountedArray",
];

fn collect_definitions(file: &File, symbols: &mut SymbolTable, emitter: &mut Emitter) {
    for item in &file.items {
        let name = item.node.name().to_string();
        if is_builtin(&name) {
            emitter.emit(Diagnostic::error(
                DiagnosticCode::S0001DuplicateDefinition,
                item.span,
                format!("cannot redefine builtin type `{name}`"),
            ));
            continue;
        }
        if symbols.defs.contains_key(&name) {
            emitter.emit(Diagnostic::error(
                DiagnosticCode::S0001DuplicateDefinition,
                item.span,
                format!("duplicate definition: `{name}`"),
            ));
            continue;
        }

        let def = match &item.node {
            Item::Structure(s) => Definition::Structure {
                span: item.span,
                bit_order: s.bit_order,
                params: s.params.iter().map(|p| p.name.clone()).collect(),
                wire_bits: None,
            },
            Item::Enumerated(e) => Definition::Enumerated {
                span: item.span,
                width_bits: expr_as_u32(&e.width_bits.node).unwrap_or(0),
            },
            Item::Choice(c) => Definition::Choice {
                span: item.span,
                discriminant: c.discriminant.clone(),
                has_catch_all: c
                    .arms
                    .iter()
                    .any(|a| matches!(a.node.pattern, ChoicePattern::Wildcard)),
            },
            Item::TypeAlias(t) => Definition::TypeAlias {
                span: item.span,
                wire_bits: t.width_bits.as_ref().and_then(|w| expr_as_u32(&w.node)),
                underlying: type_name(&t.ty).unwrap_or_default(),
            },
        };

        symbols.defs.insert(name, def);
    }
}

fn is_builtin(name: &str) -> bool {
    BUILTIN_TYPES.iter().any(|(n, _)| *n == name)
        || name == "boolean"
        || name == "null"
        || BUILTIN_GENERICS.contains(&name)
}

fn check_definitions(file: &File, symbols: &SymbolTable, emitter: &mut Emitter) {
    for item in &file.items {
        match &item.node {
            Item::Structure(s) => check_structure(item.span, s, symbols, emitter),
            Item::Enumerated(e) => check_enumerated(item.span, e, emitter),
            Item::Choice(c) => check_choice(item.span, c, file, symbols, emitter),
            Item::TypeAlias(t) => check_type_alias(item.span, t, symbols, emitter),
        }
    }
}

fn check_structure(
    span: crate::frontend::span::SimpleSpan,
    structure: &Structure,
    symbols: &SymbolTable,
    emitter: &mut Emitter,
) {
    let mut seen = HashSet::new();
    for param in &structure.params {
        if !symbols.defs.contains_key(&param.ty) && !is_builtin(&param.ty) {
            emitter.emit(Diagnostic::error(
                DiagnosticCode::S0002UnknownType,
                span,
                format!(
                    "unknown parameter type `{}` on `{}`",
                    param.ty, structure.name
                ),
            ));
        }
        if structure
            .fields
            .iter()
            .all(|f| !field_mentions_param(&f.node, &param.name))
        {
            emitter.emit(Diagnostic::warning(
                DiagnosticCode::W0601UnusedParameter,
                span,
                format!("unused parameter `{}` on `{}`", param.name, structure.name),
            ));
        }
    }

    for field in &structure.fields {
        if !seen.insert(field.node.name.clone()) {
            emitter.emit(Diagnostic::error(
                DiagnosticCode::S0106DuplicateField,
                field.span,
                format!(
                    "duplicate field `{}` in structure `{}`",
                    field.node.name, structure.name
                ),
            ));
        }
        check_field(field, symbols, emitter);
    }
}

fn check_field(field: &Spanned<Field>, symbols: &SymbolTable, emitter: &mut Emitter) {
    for attr in &field.node.attributes {
        if let Attribute::Unknown(name) = attr {
            emitter.emit(Diagnostic::error(
                DiagnosticCode::S0701UnknownAttribute,
                field.span,
                format!(
                    "unknown attribute `[{name}]` on field `{}`",
                    field.node.name
                ),
            ));
        }
    }

    let type_name = match type_name(&field.node.ty) {
        Some(n) => n,
        None => return,
    };

    if field.node.attributes.contains(&Attribute::Unused)
        && !matches!(field.node.ty, TypeExpr::Null)
        && !is_generic(&type_name)
    {
        emitter.emit(Diagnostic::error(
            DiagnosticCode::S0104UnusedInvalid,
            field.span,
            format!(
                "`[unused]` field `{}` must have type `null` or a generic container",
                field.node.name
            ),
        ));
    }

    if symbols.get(&type_name).is_none() && !is_builtin(&type_name) {
        emitter.emit(Diagnostic::error(
            DiagnosticCode::S0002UnknownType,
            field.span,
            format!("unknown type `{type_name}` on field `{}`", field.node.name),
        ));
        return;
    }

    let explicit = field.node.width_bits.as_ref().and_then(expr_as_u32);
    let inferred = resolve_wire_bits(symbols, &type_name, field.node.width_bits.as_ref());

    match (explicit, inferred) {
        (Some(exp), Some(inf)) if exp > inf => {
            emitter.emit(Diagnostic::error(
                DiagnosticCode::S0101WidthExceedsType,
                field.span,
                format!(
                    "field `{}`: wire width {exp} bits exceeds type `{type_name}` capacity ({inf} bits)",
                    field.node.name
                ),
            ));
        }
        (None, None) if !is_generic(&type_name) && !is_composite(symbols, &type_name) => {
            emitter.emit(Diagnostic::error(
                DiagnosticCode::S0110WidthNotInferable,
                field.span,
                format!(
                    "field `{}`: cannot infer wire width for type `{type_name}` - add `(N bits)`",
                    field.node.name
                ),
            ));
        }
        (None, Some(_)) | (Some(_), None) | (Some(_), Some(_)) => {}
        (None, None) => {}
    }
}

fn check_enumerated(
    span: crate::frontend::span::SimpleSpan,
    enumerated: &Enumerated,
    emitter: &mut Emitter,
) {
    if expr_as_u32(&enumerated.width_bits.node).is_none() {
        emitter.emit(Diagnostic::error(
            DiagnosticCode::S0108WidthNotInteger,
            enumerated.width_bits.span,
            format!(
                "enumeration `{}` requires a constant bit width",
                enumerated.name
            ),
        ));
    }
    let _ = span;
}

fn check_choice(
    span: crate::frontend::span::SimpleSpan,
    choice: &Choice,
    file: &File,
    symbols: &SymbolTable,
    emitter: &mut Emitter,
) {
    let has_catch_all = choice
        .arms
        .iter()
        .any(|a| matches!(a.node.pattern, ChoicePattern::Wildcard));

    if has_catch_all {
        // catch-all makes the choice exhaustive on the wire
    } else if choice.arms.is_empty() {
        emitter.emit(
            Diagnostic::error(
                DiagnosticCode::S0304NonExhaustiveChoice,
                span,
                format!(
                    "choice `{}` is not exhaustive - add arms or `_ => ...` catch-all",
                    choice.name
                ),
            )
            .with_help("add a `_ => OpaquePayload(...)` arm or cover every discriminant variant"),
        );
    } else if let Some(enum_ty) = discriminant_enum_type(choice) {
        if let Some(enumerated) = find_enumerated(file, &enum_ty) {
            let covered = covered_choice_variants(&choice.arms, &enum_ty);
            let missing: Vec<String> = enumerated
                .variants
                .iter()
                .map(|variant| variant.node.name.clone())
                .filter(|name| !covered.contains(name))
                .collect();

            if !missing.is_empty() {
                emitter.emit(
                    Diagnostic::error(
                        DiagnosticCode::S0304NonExhaustiveChoice,
                        span,
                        format!(
                            "choice `{}` does not cover enum variant(s): {}",
                            choice.name,
                            missing.join(", ")
                        ),
                    )
                    .with_help("add matching arms or a `_ => ...` catch-all"),
                );
            }
        }
    } else {
        emitter.emit(
            Diagnostic::error(
                DiagnosticCode::S0304NonExhaustiveChoice,
                span,
                format!(
                    "choice `{}` needs a `_ => ...` catch-all for discriminant `{}`",
                    choice.name, choice.discriminant
                ),
            )
            .with_help("non-enum discriminants cannot be proven exhaustive without `_ =>`"),
        );
    }

    for arm in &choice.arms {
        if let Some(ty) = type_name(&arm.node.ty)
            && symbols.get(&ty).is_none()
            && !is_builtin(&ty)
        {
            emitter.emit(Diagnostic::error(
                DiagnosticCode::S0002UnknownType,
                arm.span,
                format!("unknown type `{ty}` in choice `{}`", choice.name),
            ));
        }
    }
}

fn discriminant_enum_type(choice: &Choice) -> Option<String> {
    choice
        .params
        .iter()
        .find(|param| param.name == choice.discriminant)
        .map(|param| param.ty.clone())
}

fn find_enumerated<'a>(file: &'a File, name: &str) -> Option<&'a Enumerated> {
    file.items.iter().find_map(|item| match &item.node {
        Item::Enumerated(enumerated) if enumerated.name == name => Some(enumerated),
        _ => None,
    })
}

fn covered_choice_variants(
    arms: &[Spanned<ChoiceArm>],
    enum_ty: &str,
) -> std::collections::HashSet<String> {
    arms.iter()
        .filter_map(|arm| match &arm.node.pattern {
            ChoicePattern::Path { ty, variant } if ty.is_empty() || ty == enum_ty => {
                Some(variant.clone())
            }
            _ => None,
        })
        .collect()
}

fn check_type_alias(
    span: crate::frontend::span::SimpleSpan,
    alias: &TypeAlias,
    symbols: &SymbolTable,
    emitter: &mut Emitter,
) {
    let underlying = match type_name(&alias.ty) {
        Some(n) => n,
        None => return,
    };
    if symbols.get(&underlying).is_none() && !is_builtin(&underlying) {
        emitter.emit(Diagnostic::error(
            DiagnosticCode::S0002UnknownType,
            span,
            format!(
                "type alias `{}`: unknown underlying type `{underlying}`",
                alias.name
            ),
        ));
    }
}

fn resolve_wire_bits(
    symbols: &SymbolTable,
    type_name: &str,
    explicit: Option<&Expr>,
) -> Option<u32> {
    if let Some(expr) = explicit {
        return expr_as_u32(expr);
    }

    match symbols.get(type_name)? {
        Definition::Builtin { wire_bits } => *wire_bits,
        Definition::Enumerated { width_bits, .. } => Some(*width_bits),
        Definition::TypeAlias {
            wire_bits: Some(w),
            underlying,
            ..
        } => Some(*w).or_else(|| resolve_wire_bits(symbols, underlying, None)),
        Definition::TypeAlias {
            wire_bits: None,
            underlying,
            ..
        } => resolve_wire_bits(symbols, underlying, None),
        Definition::Structure { .. } | Definition::Choice { .. } => None,
    }
}

fn type_name(ty: &TypeExpr) -> Option<String> {
    match ty {
        TypeExpr::Null => Some("null".into()),
        TypeExpr::Named { name, .. } => Some(name.clone()),
    }
}

fn is_generic(name: &str) -> bool {
    BUILTIN_GENERICS.contains(&name)
}

fn is_composite(symbols: &SymbolTable, name: &str) -> bool {
    matches!(
        symbols.get(name),
        Some(Definition::Structure { .. }) | Some(Definition::Choice { .. })
    )
}

fn expr_as_u32(expr: &Expr) -> Option<u32> {
    match expr {
        Expr::Literal(Literal::Integer { value, .. }) if *value >= 0 => Some(*value as u32),
        _ => None,
    }
}

fn expr_mentions_ident(ty: &TypeExpr, name: &str) -> bool {
    match ty {
        TypeExpr::Null => false,
        TypeExpr::Named { generics, args, .. } => generics
            .iter()
            .chain(args.iter())
            .any(|expr| expr_contains_ident(expr, name)),
    }
}

fn field_mentions_param(field: &Field, name: &str) -> bool {
    expr_mentions_ident(&field.ty, name)
        || field
            .width_bits
            .as_ref()
            .is_some_and(|expr| expr_contains_ident(expr, name))
}

fn expr_contains_ident(expr: &Expr, name: &str) -> bool {
    match expr {
        Expr::Ident(id) => id == name,
        Expr::Field { base, .. } => expr_contains_ident(base, name),
        Expr::Call { args, .. } => args.iter().any(|a| expr_contains_ident(a, name)),
        Expr::Binary { lhs, rhs, .. } => {
            expr_contains_ident(lhs, name) || expr_contains_ident(rhs, name)
        }
        Expr::Literal(_) => false,
    }
}
