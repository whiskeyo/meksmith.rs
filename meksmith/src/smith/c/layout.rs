use crate::analyze::{CheckedFile, Definition, SymbolTable};
use crate::frontend::ast::{Attribute, BitOrder, Expr, Field, Item, Structure, TypeExpr};
use crate::frontend::token::Literal;

#[derive(Debug, Clone)]
pub struct WireField {
    pub name: String,
    pub wire_bits: u32,
    pub bit_offset: u32,
    pub is_computed: bool,
    pub is_unused: bool,
    pub host_type: String,
}

#[derive(Debug, Clone)]
pub struct StructureLayout {
    pub name: String,
    pub bit_order: BitOrder,
    pub fields: Vec<WireField>,
    pub wire_bits: u32,
}

pub fn structure_layout(structure: &Structure, symbols: &SymbolTable) -> Option<StructureLayout> {
    structure_layout_fields(structure, structure.fields.len(), symbols)
}

pub fn structure_prefix_layout(
    structure: &Structure,
    prefix_field_count: usize,
    symbols: &SymbolTable,
) -> Option<StructureLayout> {
    if prefix_field_count == 0 || prefix_field_count > structure.fields.len() {
        return None;
    }
    structure_layout_fields(structure, prefix_field_count, symbols)
}

fn structure_layout_fields(
    structure: &Structure,
    field_count: usize,
    symbols: &SymbolTable,
) -> Option<StructureLayout> {
    let mut fields = Vec::new();
    let mut bit_offset = 0u32;

    for field in structure.fields.iter().take(field_count) {
        if !field_is_codec_scalar(&field.node, symbols)
            && !field.node.attributes.contains(&Attribute::Computed)
        {
            return None;
        }

        let wire_bits = resolve_field_width(&field.node, symbols)?;
        let is_computed = field.node.attributes.contains(&Attribute::Computed);
        let is_unused = field.node.attributes.contains(&Attribute::Unused);
        let host_type = host_type_for_field(&field.node, symbols)?;

        fields.push(WireField {
            name: field.node.name.clone(),
            wire_bits,
            bit_offset,
            is_computed,
            is_unused,
            host_type,
        });
        bit_offset += wire_bits;
    }

    Some(StructureLayout {
        name: structure.name.clone(),
        bit_order: structure.bit_order,
        fields,
        wire_bits: bit_offset,
    })
}

pub fn layouts_for_file(file: &CheckedFile) -> Vec<StructureLayout> {
    let mut layouts = Vec::new();
    for item in &file.file.items {
        if let Item::Structure(structure) = &item.node
            && let Some(layout) = structure_layout(structure, &file.symbols)
            && layout.bit_order == BitOrder::Msb0
            && layout.wire_bits % 8 == 0
            && layout_is_codec_safe(&layout)
        {
            layouts.push(layout);
        }
    }
    layouts
}

fn layout_is_codec_safe(layout: &StructureLayout) -> bool {
    layout.fields.iter().all(|field| {
        if field.is_computed {
            return true;
        }
        if field.bit_offset % 8 == 0 && field.wire_bits % 8 == 0 {
            return true;
        }
        field.wire_bits <= 64
    })
}

pub(crate) fn prefix_layout_is_codec_safe(layout: &StructureLayout) -> bool {
    layout_is_codec_safe(layout)
}

fn field_is_codec_scalar(field: &Field, symbols: &SymbolTable) -> bool {
    let type_name = match &field.ty {
        TypeExpr::Null => return true,
        TypeExpr::Named { name, generics, .. } => {
            if matches!(
                name.as_str(),
                "DynamicArray"
                    | "GreedyArray"
                    | "StaticArray"
                    | "Optional"
                    | "BitSlice"
                    | "CountedArray"
            ) {
                return false;
            }
            if !generics.is_empty() {
                return false;
            }
            name.clone()
        }
    };

    matches!(
        symbols.get(&type_name),
        Some(Definition::Builtin { .. })
            | Some(Definition::Enumerated { .. })
            | Some(Definition::TypeAlias { .. })
    )
}

fn host_type_for_field(field: &Field, symbols: &SymbolTable) -> Option<String> {
    let type_name = match &field.ty {
        TypeExpr::Null => "uint8_t".into(),
        TypeExpr::Named { name, .. } => name.clone(),
    };

    if matches!(field.ty, TypeExpr::Null) {
        return Some("uint8_t".into());
    }

    match symbols.get(&type_name)? {
        Definition::Builtin { .. } => super::types::map_type_name(&type_name),
        Definition::Enumerated { .. } | Definition::TypeAlias { .. } => Some(type_name),
        _ => None,
    }
}

fn resolve_field_width(field: &Field, symbols: &SymbolTable) -> Option<u32> {
    if let Some(width) = field.width_bits.as_ref().and_then(expr_as_u32) {
        return Some(width);
    }

    let type_name = match &field.ty {
        TypeExpr::Null => "null".to_string(),
        TypeExpr::Named { name, .. } => name.clone(),
    };

    match symbols.get(&type_name)? {
        Definition::Builtin {
            wire_bits: Some(bits),
        } => Some(*bits),
        Definition::Builtin { wire_bits: None } => None,
        Definition::Enumerated { width_bits, .. } => Some(*width_bits),
        Definition::TypeAlias {
            wire_bits: Some(bits),
            underlying,
            ..
        } => Some(*bits).or_else(|| resolve_type_width(symbols, underlying)),
        Definition::TypeAlias {
            wire_bits: None,
            underlying,
            ..
        } => resolve_type_width(symbols, underlying),
        Definition::Structure { .. } | Definition::Choice { .. } => None,
    }
}

fn resolve_type_width(symbols: &SymbolTable, name: &str) -> Option<u32> {
    match symbols.get(name)? {
        Definition::Builtin {
            wire_bits: Some(bits),
        } => Some(*bits),
        Definition::Enumerated { width_bits, .. } => Some(*width_bits),
        Definition::TypeAlias {
            wire_bits: Some(bits),
            ..
        } => Some(*bits),
        Definition::TypeAlias {
            wire_bits: None,
            underlying,
            ..
        } => resolve_type_width(symbols, underlying),
        _ => None,
    }
}

fn expr_as_u32(expr: &Expr) -> Option<u32> {
    match expr {
        Expr::Literal(Literal::Integer { value, .. }) if *value >= 0 => Some(*value as u32),
        _ => None,
    }
}
