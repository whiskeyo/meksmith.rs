use super::span::{SimpleSpan, Spanned};
use super::token::Literal;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    pub span: SimpleSpan,
    pub protocol: Option<Spanned<String>>,
    pub items: Vec<Spanned<Item>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    Structure(Structure),
    Enumerated(Enumerated),
    Choice(Choice),
    TypeAlias(TypeAlias),
}

impl Item {
    pub fn name(&self) -> &str {
        match self {
            Item::Structure(s) => &s.name,
            Item::Enumerated(e) => &e.name,
            Item::Choice(c) => &c.name,
            Item::TypeAlias(t) => &t.name,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitOrder {
    Msb0,
    Lsb0,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Structure {
    pub name: String,
    pub bit_order: BitOrder,
    pub params: Vec<Param>,
    pub fields: Vec<Spanned<Field>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enumerated {
    pub name: String,
    pub bit_order: BitOrder,
    pub width_bits: Spanned<Expr>,
    pub variants: Vec<Spanned<EnumVariant>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Choice {
    pub name: String,
    pub bit_order: BitOrder,
    pub params: Vec<Param>,
    pub discriminant: String,
    pub arms: Vec<Spanned<ChoiceArm>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeAlias {
    pub name: String,
    pub width_bits: Option<Spanned<Expr>>,
    pub ty: TypeExpr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    pub name: String,
    pub ty: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub width_bits: Option<Expr>,
    pub ty: TypeExpr,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Attribute {
    Unused,
    Computed,
    NoDebug,
    Debug,
    Rename(String),
    Unknown(String),
}

impl Attribute {
    pub fn from_ident(name: &str) -> Self {
        match name {
            "unused" => Self::Unused,
            "computed" => Self::Computed,
            "no_debug" => Self::NoDebug,
            "debug" => Self::Debug,
            other => Self::Unknown(other.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumVariant {
    pub name: String,
    pub value: EnumValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumValue {
    Integer(Literal),
    Range { start: Literal, end: Literal },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChoiceArm {
    pub pattern: ChoicePattern,
    pub ty: TypeExpr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChoicePattern {
    Path { ty: String, variant: String },
    Wildcard,
    Integer(Literal),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeExpr {
    Null,
    Named {
        name: String,
        generics: Vec<Expr>,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Literal(Literal),
    Ident(String),
    Field {
        base: Box<Expr>,
        name: String,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Binary {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}
