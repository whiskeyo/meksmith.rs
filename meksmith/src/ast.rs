/// Represents an identifier, which is a name used to refer to types, fields, etc.
#[derive(Debug, Clone, PartialEq)]
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

/// Represents a type identifier, which can be a built-in type, a user-defined type,
/// a static array, or a dynamic array.
#[derive(Debug, Clone, PartialEq)]
pub enum TypeIdentifier {
    Integer8,
    Integer16,
    Integer32,
    Integer64,
    UnsignedInteger8,
    UnsignedInteger16,
    UnsignedInteger32,
    UnsignedInteger64,
    Float32,
    Float64,
    Bit,
    Byte,
    UserDefined(Identifier),
    StaticArray {
        r#type: Box<TypeIdentifier>,
        size: u64,
    },
    DynamicArray {
        r#type: Box<TypeIdentifier>,
    },
}

/// Represents a single field in an enumeration, which can either be a single value
/// or a range of values. Each field has a name and either a single value or a start
/// and end value for the range.
#[derive(Debug, Clone, PartialEq)]
pub enum EnumerationField {
    SingleValue {
        name: Identifier,
        value: u64,
    },
    RangeOfValues {
        name: Identifier,
        start: u64,
        end: u64,
    },
}

/// Represents an enumeration, which is a user-defined type that consists of
/// a set of named values, each of which can be a single value or a range of values.
#[derive(Debug, Clone, PartialEq)]
pub struct EnumerationDefinition {
    pub name: Identifier,
    pub fields: Vec<EnumerationField>,
}

/// Represents a single attribute of a field in a structure or union.
#[derive(Debug, Clone, PartialEq)]
pub enum Attribute {
    DiscriminatedBy { field: Identifier },
    BitsSize { size: u64 },
    BytesSize { size: u64 },
}

/// Represents a single field in a structure, which consists of an attribute list, name and a type.
#[derive(Debug, Clone, PartialEq)]
pub struct StructureField {
    pub name: Identifier,
    pub r#type: TypeIdentifier,
    pub attributes: Vec<Attribute>,
}

/// Represents a structure, which is a user-defined type that consists of
/// a collection of fields, each with a name and a type.
#[derive(Debug, Clone, PartialEq)]
pub struct StructureDefinition {
    pub name: Identifier,
    pub fields: Vec<StructureField>,
}

/// Represents a single field in a union, which consists of a name, type, and
/// a discriminator value that identifies which type the field holds.
/// The discriminator is an integer value that is unique for each field in the union.
#[derive(Debug, Clone, PartialEq)]
pub enum UnionField {
    SingleValue {
        name: Identifier,
        r#type: TypeIdentifier,
        discriminator: u64,
    },
    RangeOfValues {
        name: Identifier,
        r#type: TypeIdentifier,
        start_discriminator: u64,
        end_discriminator: u64,
    },
}

/// Represents a union, which is a user-defined type that can hold one of several
/// values, each identified by a discriminator.
#[derive(Debug, Clone, PartialEq)]
pub struct UnionDefinition {
    pub name: Identifier,
    pub fields: Vec<UnionField>,
}

/// Represents a type definition, which is a user-defined type that can be
/// an alias for a built-in type or another user-defined type.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeDefinition {
    pub new_type: Identifier,
    pub r#type: TypeIdentifier,
}

/// Represents a single definition in the protocol, which can be an [`EnumerationDefinition`],
/// [`StructureDefinition`], [`UnionDefinition`], or [`TypeDefinition`].
#[derive(Debug, Clone, PartialEq)]
pub enum Definition {
    Enumeration(EnumerationDefinition),
    Structure(StructureDefinition),
    Union(UnionDefinition),
    Type(TypeDefinition),
}

/// Represents the entire protocol, which consists of multiple definitions.
#[derive(Debug, Clone, PartialEq)]
pub struct Protocol {
    pub definitions: Vec<Definition>,
}
