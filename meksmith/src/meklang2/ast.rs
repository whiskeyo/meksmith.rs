// ***************************************
// SECTION FOR ATOMS
// ***************************************

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NumericLiteral {
    pub value: u128,
    pub radix: Radix,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Radix {
    Decimal,
    Hexadecimal,
    Binary,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub name: String,
}

impl Identifier {
    pub fn new(name: &str) -> Self {
        Identifier {
            name: name.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Reference {
    pub path: Vec<Identifier>,
}

// ***************************************
// SECTION FOR EXPRESSIONS
// ***************************************

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    NumericLiteral(NumericLiteral),
    Reference(Reference),
    UnaryOperator {
        operator: UnaryOperator,
        expr: Box<Expr>,
    },
    BinaryOperator {
        operator: BinaryOperator,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    Not,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    And,
    Or,

    Equals,
    NotEquals,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
}

// ***************************************
// SECTION FOR BIT ENUMS
// ***************************************

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BitEnum {
    pub size: usize,
    pub name: Identifier,
    pub fields: Vec<BitEnumField>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BitEnumField {
    SingleValue {
        name: Identifier,
        value: usize,
    },
    RangeOfValues {
        name: Identifier,
        from: usize,
        to: usize,
    },
}

// ***************************************
// SECTION FOR BIT STRUCTURES
// ***************************************

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BitStruct {
    pub name: Identifier,
    pub bit_order: BitStructBitOrder,
    pub fields: Vec<BitStructField>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BitStructBitOrder {
    MostSignificantBitIsBit0,
    LeastSignificantBitIsBit0,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BitStructField {
    Ordinary {
        name: Identifier,
        typ: BitStructFieldType,
        bits: BitStructFieldBitsType,
        attributes: Vec<BitStructOrdinaryFieldAttribute>,
    },
    Union {
        name: Identifier,
        bits: BitStructFieldBitsType,
        when_expr: Option<Expr>,
        fields: Vec<BitStructFieldUnion>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BitStructFieldType {
    Builtin(BitStructBuiltinType),
    UserDefined(Identifier),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BitStructBuiltinType {
    UnsignedInteger8,
    UnsignedInteger16,
    UnsignedInteger32,
    UnsignedInteger64,
    SignedInteger8,
    SignedInteger16,
    SignedInteger32,
    SignedInteger64,
    Boolean,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BitStructFieldBitsType {
    /// Represented by `bit N..M` where `N` and `M` are numbers.
    Range { from: usize, to: usize },
    /// Represented by `bit +N`, where `N` is a number.
    Increment { amount: usize },
    /// Represented by `bit derived`.
    Derived,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BitStructOrdinaryFieldAttribute {
    LittleEndian,
    BigEndian,
    StaticArray { size: usize },
    DynamicArray { reference: Reference },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BitStructFieldUnion {
    pub discriminator: usize,
    pub name: Identifier,
    pub typ: BitStructFieldType,
    // maybe add attributes in future?
}

// ***************************************
// SECTION FOR PROTOCOL & MODULES
// ***************************************
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module {
    pub metadata: Metadata,
    pub definitions: Vec<Definition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Metadata {
    pub module_name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub docs: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Definition {
    Enum(BitEnum),
    Struct(BitStruct),
}
