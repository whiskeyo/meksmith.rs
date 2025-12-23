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
pub enum Attribute {
    Bits(usize),
    Bytes(usize),
    Discriminator(Identifier),
    StaticArray(usize),
    DynamicArray(Identifier),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuiltinType {
    SignedInteger8,
    SignedInteger16,
    SignedInteger32,
    SignedInteger64,
    UnsignedInteger8,
    UnsignedInteger16,
    UnsignedInteger32,
    UnsignedInteger64,
    Float32,
    Float64,
    Boolean,
    Bit,
    Byte,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Builtin(BuiltinType),
    UserDefined(Identifier),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructureField {
    pub name: Identifier,
    pub typ: Type,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Structure {
    pub name: Identifier,
    pub fields: Vec<StructureField>,
    // maybe add attributes in future if needed?
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EnumerationField {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Enumeration {
    pub name: Identifier,
    pub fields: Vec<EnumerationField>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnionField {
    pub name: Identifier,
    pub typ: Type,
    pub discriminator: usize,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Union {
    pub name: Identifier,
    pub fields: Vec<UnionField>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Definition {
    Enumeration(Enumeration),
    Structure(Structure),
    Union(Union),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module {
    // maybe add a name/version here?
    pub definitions: Vec<Definition>,
}
