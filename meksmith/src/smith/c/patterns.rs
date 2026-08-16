use crate::analyze::{CheckedFile, Definition, SymbolTable};
use crate::frontend::ast::{Attribute, BitOrder, Expr, Field, Item, Structure, TypeExpr};

use super::layout::{StructureLayout, prefix_layout_is_codec_safe, structure_prefix_layout};

#[derive(Debug, Clone)]
pub struct PrefixField {
    pub name: String,
    pub wire_bytes: u32,
    /// When set, prefix is emitted via the nested type's layout codec.
    pub nested_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ParametricBlobLayout {
    pub param_name: String,
    pub prefix_fields: Vec<PrefixField>,
    pub prefix_bytes: u32,
    pub array_field: String,
}

#[derive(Debug, Clone)]
pub struct ParametricOpaqueLayout {
    pub param_name: String,
    pub array_field: String,
}

#[derive(Debug, Clone)]
pub struct LengthPrefixedLayout {
    pub length_field: String,
    pub length_bits: u32,
    pub array_field: String,
}

#[derive(Debug, Clone)]
pub struct MessageWrapperLayout {
    pub name: String,
    pub header_type: String,
    pub header_field: String,
    pub body_field: String,
    pub choice_name: String,
    pub discriminant_field: String,
    pub size_field: Option<String>,
    pub header_wire_bytes: u32,
}

#[derive(Debug, Clone)]
pub struct GreedyRootLayout {
    pub name: String,
    pub array_field: String,
    pub element_type: String,
}

#[derive(Debug, Clone)]
pub struct PduMessageLayout {
    pub name: String,
    pub message_field: String,
    pub message_type: String,
    pub padding_field: String,
    pub align: u32,
    pub concat_field: String,
    pub header_wire_bytes: u32,
    pub header_type: String,
    pub header_field_path: String,
}

#[derive(Debug, Clone)]
pub struct ParametricWireTailLayout {
    pub param_name: String,
    pub structure_name: String,
    pub prefix_layout: StructureLayout,
    pub prefix_bytes: u32,
    pub tail: ParametricTail,
}

#[derive(Debug, Clone)]
pub enum ParametricTail {
    Bytes {
        field: String,
        subtract: u32,
    },
    Structs {
        field: String,
        element_type: String,
        element_bytes: u32,
        subtract: u32,
    },
}

pub fn parametric_wire_tail_layout(
    structure: &Structure,
    symbols: &SymbolTable,
    file: &CheckedFile,
) -> Option<ParametricWireTailLayout> {
    if structure.params.len() != 1 || structure.fields.len() < 2 {
        return None;
    }
    let param_name = structure.params[0].name.clone();
    let last = structure.fields.last()?;
    let (array_field, element_type, len_expr) = parse_dynamic_array(&last.node)?;

    let prefix_layout = structure_prefix_layout(structure, structure.fields.len() - 1, symbols)?;
    if prefix_layout.bit_order != BitOrder::Msb0 || prefix_layout.wire_bits % 8 != 0 {
        return None;
    }
    if !prefix_layout_is_codec_safe(&prefix_layout) {
        return None;
    }
    let prefix_bytes = prefix_layout.wire_bits / 8;

    let tail = if element_type == "byte" {
        let subtract = parse_tail_len_subtract(&len_expr, &param_name, prefix_bytes, 1)?;
        ParametricTail::Bytes {
            field: array_field,
            subtract,
        }
    } else {
        let element_bytes = header_wire_bytes_for_type(&element_type, file)?;
        let subtract =
            parse_tail_len_subtract(&len_expr, &param_name, prefix_bytes, element_bytes)?;
        ParametricTail::Structs {
            field: array_field,
            element_type,
            element_bytes,
            subtract,
        }
    };

    Some(ParametricWireTailLayout {
        param_name,
        structure_name: structure.name.clone(),
        prefix_layout,
        prefix_bytes,
        tail,
    })
}

pub fn parametric_blob_layout(
    structure: &Structure,
    symbols: &SymbolTable,
    file: &CheckedFile,
) -> Option<ParametricBlobLayout> {
    if structure.params.len() != 1 || structure.fields.len() < 2 {
        return None;
    }
    let param_name = structure.params[0].name.clone();

    let last = structure.fields.last()?;
    let (array_field, tail_expr) = dynamic_byte_array(&last.node)?;
    if !array_len_minus_prefix(&tail_expr, &param_name, structure, symbols, file)? {
        return None;
    }

    let mut prefix_fields = Vec::new();
    let mut prefix_bytes = 0u32;
    for field in &structure.fields[..structure.fields.len() - 1] {
        let prefix = prefix_field(&field.node, symbols, file)?;
        prefix_bytes += prefix.wire_bytes;
        prefix_fields.push(prefix);
    }

    Some(ParametricBlobLayout {
        param_name,
        prefix_fields,
        prefix_bytes,
        array_field,
    })
}

pub fn parametric_opaque_layout(
    structure: &Structure,
    symbols: &SymbolTable,
) -> Option<ParametricOpaqueLayout> {
    if structure.params.len() != 1 || structure.fields.len() != 1 {
        return None;
    }
    let param_name = structure.params[0].name.clone();
    let (array_field, len_expr) = dynamic_byte_array(&structure.fields[0].node)?;
    match len_expr {
        Expr::Ident(name) if name == param_name => {}
        _ => return None,
    }
    let _ = symbols;
    Some(ParametricOpaqueLayout {
        param_name,
        array_field,
    })
}

pub fn length_prefixed_layout(
    structure: &Structure,
    symbols: &SymbolTable,
) -> Option<LengthPrefixedLayout> {
    if !structure.params.is_empty() || structure.fields.len() != 2 {
        return None;
    }

    let length_field = &structure.fields[0].node;
    let array_field = &structure.fields[1].node;
    let length_bits = field_wire_bits(length_field, symbols)?;
    if length_bits != 16 {
        return None;
    }

    let (array_name, len_expr) = dynamic_byte_array(array_field)?;
    match len_expr {
        Expr::Ident(name) if name == length_field.name => {}
        _ => return None,
    }

    Some(LengthPrefixedLayout {
        length_field: length_field.name.clone(),
        length_bits,
        array_field: array_name,
    })
}

pub fn message_wrapper_layout(
    structure: &Structure,
    file: &CheckedFile,
) -> Option<MessageWrapperLayout> {
    if structure.fields.len() != 2 {
        return None;
    }

    let header_field = &structure.fields[0].node;
    let body_field = &structure.fields[1].node;
    let TypeExpr::Named {
        name: choice_name,
        args,
        ..
    } = &body_field.ty
    else {
        return None;
    };

    let _choice = file.file.items.iter().find_map(|item| match &item.node {
        Item::Choice(choice) if choice.name == *choice_name => Some(choice),
        _ => None,
    })?;

    if args.is_empty() {
        return None;
    }

    let discriminant_field = field_path_tail(&args[0])?;
    let size_field = args.get(1).and_then(field_path_tail);
    let header_type = match &header_field.ty {
        TypeExpr::Named { name, .. } => name.clone(),
        _ => return None,
    };
    let header_wire_bytes = header_wire_bytes_for_type(&header_type, file)?;

    Some(MessageWrapperLayout {
        name: structure.name.clone(),
        header_type,
        header_field: header_field.name.clone(),
        body_field: body_field.name.clone(),
        choice_name: choice_name.clone(),
        discriminant_field,
        size_field,
        header_wire_bytes,
    })
}

pub fn greedy_root_layout(structure: &Structure) -> Option<GreedyRootLayout> {
    if structure.fields.len() != 1 {
        return None;
    }
    let field = &structure.fields[0].node;
    let TypeExpr::Named {
        name,
        generics,
        args,
    } = &field.ty
    else {
        return None;
    };
    if name != "GreedyArray" || generics.len() != 1 || !args.is_empty() {
        return None;
    }
    let element_type = type_name_from_expr(&generics[0])?;
    Some(GreedyRootLayout {
        name: structure.name.clone(),
        array_field: field.name.clone(),
        element_type,
    })
}

pub fn pdu_message_layout(structure: &Structure, file: &CheckedFile) -> Option<PduMessageLayout> {
    if structure.fields.len() != 2 {
        return None;
    }

    let message_field = &structure.fields[0].node;
    let padding_field = &structure.fields[1].node;
    if !padding_field.attributes.contains(&Attribute::Unused) {
        return None;
    }

    let message_type = match &message_field.ty {
        TypeExpr::Named {
            name,
            generics,
            args,
        } if generics.is_empty() && args.is_empty() => name.clone(),
        _ => return None,
    };

    if !file
        .file
        .items
        .iter()
        .any(|item| matches!(&item.node, Item::Structure(s) if s.name == message_type))
    {
        return None;
    }

    let (padding_name, pad_expr) = dynamic_byte_array(padding_field)?;
    let (align, concat_field) = parse_pad_to_align_if(&pad_expr)?;

    let message_wrapper = file.file.items.iter().find_map(|item| match &item.node {
        Item::Structure(structure) if structure.name == message_type => {
            message_wrapper_layout(structure, file)
        }
        _ => None,
    })?;

    Some(PduMessageLayout {
        name: structure.name.clone(),
        message_field: message_field.name.clone(),
        message_type,
        padding_field: padding_name,
        align,
        concat_field,
        header_wire_bytes: message_wrapper.header_wire_bytes,
        header_type: message_wrapper.header_type.clone(),
        header_field_path: format!("{}.{}", message_field.name, message_wrapper.header_field),
    })
}

fn parse_dynamic_array(field: &Field) -> Option<(String, String, Expr)> {
    let TypeExpr::Named { name, generics, .. } = &field.ty else {
        return None;
    };
    if name != "DynamicArray" || generics.len() != 2 {
        return None;
    }
    let element = match &generics[0] {
        Expr::Ident(element) => element.clone(),
        _ => return None,
    };
    Some((field.name.clone(), element, generics[1].clone()))
}

fn dynamic_byte_array(field: &Field) -> Option<(String, Expr)> {
    let (name, element, len) = parse_dynamic_array(field)?;
    if element != "byte" {
        return None;
    }
    Some((name, len))
}

fn parse_tail_len_subtract(
    expr: &Expr,
    param_name: &str,
    prefix_bytes: u32,
    divisor: u32,
) -> Option<u32> {
    match expr {
        Expr::Binary {
            op: crate::frontend::ast::BinOp::Sub,
            lhs,
            rhs,
        } if divisor == 1 => match (&**lhs, &**rhs) {
            (Expr::Ident(name), Expr::Literal(lit)) if name == param_name => {
                let value = literal_u32(lit)?;
                (value == prefix_bytes).then_some(value)
            }
            _ => None,
        },
        Expr::Binary {
            op: crate::frontend::ast::BinOp::Div,
            lhs,
            rhs,
        } => match (&**lhs, &**rhs) {
            (
                Expr::Binary {
                    op: crate::frontend::ast::BinOp::Sub,
                    lhs: inner_lhs,
                    rhs: inner_rhs,
                },
                Expr::Literal(div_lit),
            ) if divisor > 1 => {
                let div_value = literal_u32(div_lit)?;
                if div_value != divisor {
                    return None;
                }
                match (&**inner_lhs, &**inner_rhs) {
                    (Expr::Ident(name), Expr::Literal(sub_lit)) if name == param_name => {
                        let sub_value = literal_u32(sub_lit)?;
                        (sub_value == prefix_bytes).then_some(sub_value)
                    }
                    _ => None,
                }
            }
            _ => None,
        },
        Expr::Ident(name) if name == param_name && prefix_bytes == 0 && divisor == 1 => Some(0),
        _ => None,
    }
}

fn prefix_field(field: &Field, symbols: &SymbolTable, file: &CheckedFile) -> Option<PrefixField> {
    let name = field.name.clone();
    if let Some(bits) = field_wire_bits(field, symbols) {
        if bits == 0 || bits % 8 != 0 {
            return None;
        }
        return Some(PrefixField {
            name,
            wire_bytes: bits / 8,
            nested_type: None,
        });
    }

    let TypeExpr::Named {
        name: type_name,
        generics,
        args,
    } = &field.ty
    else {
        return None;
    };
    if !generics.is_empty() || !args.is_empty() {
        return None;
    }
    let wire_bytes = header_wire_bytes_for_type(type_name, file)?;
    Some(PrefixField {
        name,
        wire_bytes,
        nested_type: Some(type_name.clone()),
    })
}

fn prefix_wire_bytes(field: &Field, symbols: &SymbolTable, file: &CheckedFile) -> Option<u32> {
    Some(prefix_field(field, symbols, file)?.wire_bytes)
}

fn array_len_minus_prefix(
    expr: &Expr,
    param_name: &str,
    structure: &Structure,
    symbols: &SymbolTable,
    file: &CheckedFile,
) -> Option<bool> {
    let mut prefix_bytes = 0u32;
    for field in &structure.fields[..structure.fields.len() - 1] {
        prefix_bytes += prefix_wire_bytes(&field.node, symbols, file)?;
    }
    let expected = prefix_bytes;

    match expr {
        Expr::Binary {
            op: crate::frontend::ast::BinOp::Sub,
            lhs,
            rhs,
        } => match (&**lhs, &**rhs) {
            (Expr::Ident(name), Expr::Literal(lit)) if name == param_name => {
                let value = literal_u32(lit)?;
                Some(value == expected)
            }
            _ => None,
        },
        Expr::Ident(name) if name == param_name => Some(expected == 0),
        _ => None,
    }
}

fn parse_pad_to_align_if(expr: &Expr) -> Option<(u32, String)> {
    let Expr::Call { callee, args } = expr else {
        return None;
    };
    let Expr::Ident(name) = &**callee else {
        return None;
    };
    if name != "pad_to_align_if" || args.len() != 3 {
        return None;
    }
    let align = match &args[0] {
        Expr::Literal(lit) => literal_u32(lit)?,
        _ => return None,
    };
    let concat_field = field_path_string(&args[2])?;
    Some((align, concat_field))
}

fn field_path_tail(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Field { name, .. } => Some(name.clone()),
        Expr::Ident(name) => Some(name.clone()),
        _ => None,
    }
}

fn field_path_string(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Field { base, name } => Some(format!("{}.{}", field_path_string(base)?, name)),
        Expr::Ident(name) => Some(name.clone()),
        _ => None,
    }
}

pub fn header_wire_bytes_for_type(header_type: &str, file: &CheckedFile) -> Option<u32> {
    let structure = file.file.items.iter().find_map(|item| match &item.node {
        Item::Structure(structure) if structure.name == header_type => Some(structure),
        _ => None,
    })?;

    let mut bits = 0u32;
    for field in &structure.fields {
        bits += field_wire_bits(&field.node, &file.symbols)?;
    }
    if !bits.is_multiple_of(8) {
        return None;
    }
    Some(bits / 8)
}

fn field_wire_bits(field: &Field, symbols: &SymbolTable) -> Option<u32> {
    if let Some(width) = field.width_bits.as_ref().and_then(literal_u32_expr) {
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
        Definition::Enumerated { width_bits, .. } => Some(*width_bits),
        Definition::TypeAlias {
            wire_bits: Some(bits),
            underlying,
            ..
        } => Some(*bits).or_else(|| type_wire_bits(symbols, underlying)),
        Definition::TypeAlias {
            wire_bits: None,
            underlying,
            ..
        } => type_wire_bits(symbols, underlying),
        _ => None,
    }
}

fn type_wire_bits(symbols: &SymbolTable, name: &str) -> Option<u32> {
    match symbols.get(name)? {
        Definition::Builtin {
            wire_bits: Some(bits),
        } => Some(*bits),
        Definition::Enumerated { width_bits, .. } => Some(*width_bits),
        Definition::TypeAlias {
            wire_bits: Some(bits),
            underlying,
            ..
        } => Some(*bits).or_else(|| type_wire_bits(symbols, underlying)),
        Definition::TypeAlias {
            wire_bits: None,
            underlying,
            ..
        } => type_wire_bits(symbols, underlying),
        _ => None,
    }
}

fn literal_u32_expr(expr: &Expr) -> Option<u32> {
    match expr {
        Expr::Literal(lit) => literal_u32(lit),
        _ => None,
    }
}

fn literal_u32(lit: &crate::frontend::token::Literal) -> Option<u32> {
    match lit {
        crate::frontend::token::Literal::Integer { value, .. } if *value >= 0 => {
            Some(*value as u32)
        }
        _ => None,
    }
}

fn type_name_from_expr(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Ident(name) => Some(name.clone()),
        _ => None,
    }
}
