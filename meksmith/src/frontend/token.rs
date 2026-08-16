use std::fmt;

use super::span::SimpleSpan;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tokens(pub Vec<Token>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: SimpleSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Keyword(Keyword),
    Identifier(String),
    Literal(Literal),
    Delimiter(Delimiter),
    Operator(Operator),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Protocol,
    Structure,
    Enumerated,
    Choice,
    Type,
    Msb0,
    Lsb0,
    On,
    Bits,
}

impl Keyword {
    pub fn from_ident(ident: &str) -> Option<Self> {
        Some(match ident {
            "protocol" => Self::Protocol,
            "structure" => Self::Structure,
            "enumerated" => Self::Enumerated,
            "choice" => Self::Choice,
            "type" => Self::Type,
            "msb0" => Self::Msb0,
            "lsb0" => Self::Lsb0,
            "on" => Self::On,
            "bit" | "bits" => Self::Bits,
            _ => return None,
        })
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Protocol => "protocol",
            Self::Structure => "structure",
            Self::Enumerated => "enumerated",
            Self::Choice => "choice",
            Self::Type => "type",
            Self::Msb0 => "msb0",
            Self::Lsb0 => "lsb0",
            Self::On => "on",
            Self::Bits => "bits",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delimiter {
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LAngle,
    RAngle,
}

impl Delimiter {
    pub fn from_char(c: char) -> Option<Self> {
        Some(match c {
            '(' => Self::LParen,
            ')' => Self::RParen,
            '{' => Self::LBrace,
            '}' => Self::RBrace,
            '[' => Self::LBracket,
            ']' => Self::RBracket,
            '<' => Self::LAngle,
            '>' => Self::RAngle,
            _ => return None,
        })
    }

    pub fn as_char(self) -> char {
        match self {
            Self::LParen => '(',
            Self::RParen => ')',
            Self::LBrace => '{',
            Self::RBrace => '}',
            Self::LBracket => '[',
            Self::RBracket => ']',
            Self::LAngle => '<',
            Self::RAngle => '>',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Dot,
    Plus,
    Minus,
    Star,
    Slash,
    Comma,
    Semicolon,
    Colon,
    Equal,
    DoubleDot,
    DoubleColon,
    FatArrow,
}

impl Operator {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Dot => ".",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Star => "*",
            Self::Slash => "/",
            Self::Comma => ",",
            Self::Semicolon => ";",
            Self::Colon => ":",
            Self::Equal => "=",
            Self::DoubleDot => "..",
            Self::DoubleColon => "::",
            Self::FatArrow => "=>",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Integer { value: i64, radix: u32 },
    Boolean(bool),
    Null,
}

impl Literal {
    pub fn new_integer(value: i64, radix: u32) -> Self {
        Literal::Integer { value, radix }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Keyword(keyword) => write!(f, "{}", keyword.as_str()),
            TokenKind::Identifier(name) => write!(f, "{name}"),
            TokenKind::Literal(literal) => write!(f, "{literal}"),
            TokenKind::Delimiter(delimiter) => write!(f, "{}", delimiter.as_char()),
            TokenKind::Operator(operator) => write!(f, "{}", operator.as_str()),
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Integer { value, radix } => match radix {
                2 => write!(f, "0b{value:b}"),
                16 => write!(f, "0x{value:X}"),
                _ => write!(f, "{value}"),
            },
            Literal::Boolean(value) => write!(f, "{value}"),
            Literal::Null => write!(f, "null"),
        }
    }
}
