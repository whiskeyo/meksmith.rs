//! Grammar for the Meksmith Lang is defined as follows:
//! ```text
//! <protocol> := <definition>+
//! <definition> :=
//!       <enumeration_definition>
//!     | <structure_definition>
//!     | <union_definition>
//!     | <type_definition>
//!
//! <enumeration_definition> := 'enum' <identifier> '{' <enumeration_field>+ '}'
//! <enumeration_field> := <identifier> '=' (<integer> | <range>) ';'
//!
//! <structure_definition> := 'struct' <identifier> '{' <structure_field>+ '}'
//! <structure_field> := <identifier> ':' <type_identifier> ';'
//!
//! <union_definition> := 'union' <identifier> '{' <union_field>+ '}'
//! <union_field> := <integer> '=>' <identifier> ':' <type_identifier> ';'
//!
//! <type_definition> := 'using' <identifier> '=' <type_identifier> ';'
//!
//! <type_identifier> :=
//!       'int8' | 'int16' | 'int32' | 'int64'
//!     | 'uint8' | 'uint16' | 'uint32' | 'uint64'
//!     | 'float32' | 'float64'
//!     | 'bit' | 'byte'
//!     | <identifier>
//!     | <type_identifier> '[' <integer> ']' // static array
//!     | <type_identifier> '[]' // dynamic array
//!
//! <identifier> := [a-zA-Z_][a-zA-Z0-9_]*
//! <integer> := [0-9]+
//! <range> := <integer> '..' <integer>
//! ```
//!
//! This grammar defines the structure of a protocol of the Codegen Lang, whose
//! main purpose is to define data structures and types that can be used in code generation.

use crate::ast::*;

use chumsky::prelude::*;

/// Parses an identifier from the input string. Identifier has to start with
/// either alphabetic characters or an underscore, followed by alphanumeric
/// characters or underscores.
pub(crate) fn identifier<'src>() -> impl Parser<'src, &'src str, Identifier> {
    text::ident()
        .map(|s: &str| Identifier::new(s))
        .labelled("identifier")
        .padded()
}

/// Parses a type identifier from the input string. It can be a predefined type
/// like `int8`, `uint16`, `float32`, etc., or a user-defined type.
/// It can also be a static or dynamic array of a given type.
/// The static array is defined as `type[size]`, and the dynamic array is defined as `type[]`.
pub(crate) fn type_identifier<'src>() -> impl Parser<'src, &'src str, TypeIdentifier> {
    recursive(|_type_identifier| {
        let int8 = just("int8").to(TypeIdentifier::Integer8);
        let int16 = just("int16").to(TypeIdentifier::Integer16);
        let int32 = just("int32").to(TypeIdentifier::Integer32);
        let int64 = just("int64").to(TypeIdentifier::Integer64);
        let uint8 = just("uint8").to(TypeIdentifier::UnsignedInteger8);
        let uint16 = just("uint16").to(TypeIdentifier::UnsignedInteger16);
        let uint32 = just("uint32").to(TypeIdentifier::UnsignedInteger32);
        let uint64 = just("uint64").to(TypeIdentifier::UnsignedInteger64);
        let float32 = just("float32").to(TypeIdentifier::Float32);
        let float64 = just("float64").to(TypeIdentifier::Float64);
        let bit = just("bit").to(TypeIdentifier::Bit);
        let byte = just("byte").to(TypeIdentifier::Byte);
        let user_defined = identifier().map(TypeIdentifier::UserDefined).boxed();

        let base_type = choice((
            int8,
            int16,
            int32,
            int64,
            uint8,
            uint16,
            uint32,
            uint64,
            float32,
            float64,
            bit,
            byte,
            user_defined,
        ))
        .boxed();

        let static_array = base_type
            .clone()
            .then_ignore(just('[').padded())
            .then(
                text::int(10)
                    .padded()
                    .map(|s: &str| s.parse::<u64>().unwrap()),
            )
            .then_ignore(just(']'))
            .map(|(ty, size)| TypeIdentifier::StaticArray {
                r#type: Box::new(ty),
                size,
            });

        let dynamic_array =
            base_type
                .clone()
                .then_ignore(just("[]"))
                .map(|ty| TypeIdentifier::DynamicArray {
                    r#type: Box::new(ty),
                });

        choice((static_array, dynamic_array, base_type)).labelled("type_identifier")
    })
}

/// Parses a single value enumeration field in the format `name = value;`
pub(crate) fn enumeration_field_single_value<'src>()
-> impl Parser<'src, &'src str, EnumerationField> {
    let name = identifier();
    let equals = just('=').padded();
    let value = text::int(10)
        .padded()
        .map(|s: &str| s.parse::<u64>().unwrap());
    let semicolon = just(';').padded();

    name.then_ignore(equals)
        .then(value)
        .then_ignore(semicolon)
        .map(|(name, value)| EnumerationField::SingleValue { name, value })
        .labelled("enumeration_field_single_value")
        .padded()
}

/// Parses a range of values defined by `start..end`.
pub(crate) fn range<'src>() -> impl Parser<'src, &'src str, (u64, u64)> {
    let start = text::int(10)
        .padded()
        .map(|s: &str| s.parse::<u64>().unwrap());
    let range = just("..").padded();
    let end = text::int(10)
        .padded()
        .map(|s: &str| s.parse::<u64>().unwrap());

    start
        .then_ignore(range)
        .then(end)
        .map(|(start, end)| (start, end))
        .labelled("range")
        .padded()
}

/// Parses a range of values enumeration field in the format `name = start..end;`
pub(crate) fn enumeration_field_range_of_values<'src>()
-> impl Parser<'src, &'src str, EnumerationField> {
    let name = identifier();
    let equals = just('=').padded();
    let range = range();
    let semicolon = just(';').padded();

    name.then_ignore(equals)
        .then(range)
        .then_ignore(semicolon)
        .map(|(name, (start, end))| EnumerationField::RangeOfValues { name, start, end })
        .labelled("enumeration_field_range_of_values")
        .padded()
}

/// Parses an enumeration field from the input string.
pub(crate) fn enumeration_field<'src>() -> impl Parser<'src, &'src str, EnumerationField> {
    choice((
        enumeration_field_single_value(),
        enumeration_field_range_of_values(),
    ))
    .labelled("enumeration_field")
    .padded()
}

/// Parses an enumeration with fields.
pub(crate) fn enumeration_definition<'src>() -> impl Parser<'src, &'src str, Enumeration> {
    let enum_keyword = just("enum").padded();
    let name = identifier();
    let open_brace = just("{").padded();
    let fields = enumeration_field()
        .repeated()
        .at_least(1)
        .collect::<Vec<EnumerationField>>();
    let close_brace = just("};").padded();

    enum_keyword
        .ignore_then(name)
        .then_ignore(open_brace)
        .then(fields)
        .then_ignore(close_brace)
        .map(|(name, fields)| Enumeration { name, fields })
        .labelled("enumeration")
        .padded()
}

/// Parses a structure field, which consists of a name and a type identifier.
pub(crate) fn structure_field<'src>() -> impl Parser<'src, &'src str, StructureField> {
    let name = identifier();
    let colon = just(':').padded();
    let r#type = type_identifier();
    let semicolon = just(';').padded();

    name.then_ignore(colon)
        .then(r#type)
        .then_ignore(semicolon)
        .map(|(name, r#type)| StructureField { name, r#type })
        .labelled("structure_field")
        .padded()
}

/// Parses a structure definition, which consists of a name and a collection of fields.
pub(crate) fn structure_definition<'src>() -> impl Parser<'src, &'src str, Structure> {
    let struct_keyword = just("struct").padded();
    let name = identifier();
    let open_brace = just("{").padded();
    let fields = structure_field()
        .repeated()
        .at_least(1)
        .collect::<Vec<StructureField>>();
    let close_brace = just("};").padded();

    struct_keyword
        .ignore_then(name)
        .then_ignore(open_brace)
        .then(fields)
        .then_ignore(close_brace)
        .map(|(name, fields)| Structure { name, fields })
        .labelled("structure_definition")
        .padded()
}

/// Parses a union field, which consists of a discriminator, name, and type identifier.
pub(crate) fn union_field<'src>() -> impl Parser<'src, &'src str, UnionField> {
    let discriminator = text::int(10)
        .padded()
        .map(|s: &str| s.parse::<i64>().unwrap());
    let assigned_to = just("=>").padded();
    let name = identifier();
    let colon = just(':').padded();
    let r#type = type_identifier();
    let semicolon = just(';').padded();

    discriminator
        .then_ignore(assigned_to)
        .then(name)
        .then_ignore(colon)
        .then(r#type)
        .then_ignore(semicolon)
        .map(|((discriminator, name), r#type)| UnionField {
            name,
            r#type,
            discriminator,
        })
        .labelled("union_field")
        .padded()
}

/// Parses a union definition, which consists of a name and a collection of union fields.
pub(crate) fn union_definition<'src>() -> impl Parser<'src, &'src str, Union> {
    let union_keyword = just("union").padded();
    let name = identifier();
    let open_brace = just("{").padded();
    let fields = union_field()
        .repeated()
        .at_least(1)
        .collect::<Vec<UnionField>>();
    let close_brace = just("};").padded();

    union_keyword
        .ignore_then(name)
        .then_ignore(open_brace)
        .then(fields)
        .then_ignore(close_brace)
        .map(|(name, fields)| Union { name, fields })
        .labelled("union")
        .padded()
}

/// Parses a type definition, which consists of a new type name and an existing type.
pub(crate) fn type_definition<'src>() -> impl Parser<'src, &'src str, TypeDefinition> {
    let using = just("using").padded();
    let new_type = identifier();
    let equals = just('=').padded();
    let r#type = type_identifier();
    let semicolon = just(';').padded();

    using
        .ignore_then(new_type)
        .then_ignore(equals)
        .then(r#type)
        .then_ignore(semicolon)
        .map(|(new_type, r#type)| TypeDefinition { new_type, r#type })
        .labelled("type_definition")
        .padded()
}

/// Parses a single definition, which can be an enumeration, structure, union, or type definition.
pub(crate) fn definition<'src>() -> impl Parser<'src, &'src str, Definition> {
    choice((
        enumeration_definition().map(Definition::Enumeration),
        structure_definition().map(Definition::Structure),
        union_definition().map(Definition::Union),
        type_definition().map(Definition::TypeDefinition),
    ))
    .labelled("definition")
    .padded()
}

/// Parses the entire protocol, which consists of multiple definitions.
pub(crate) fn protocol<'src>() -> impl Parser<'src, &'src str, Protocol> {
    definition()
        .repeated()
        .collect::<Vec<Definition>>()
        .map(|definitions| Protocol { definitions })
        .labelled("protocol")
        .padded()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier() {
        let result = identifier().parse("myIdentifier");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            Identifier {
                name: "myIdentifier".to_string()
            }
        );
    }

    #[test]
    fn test_identifier_starting_with_underscore() {
        let result = identifier().parse("_myIdentifier");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            Identifier {
                name: "_myIdentifier".to_string()
            }
        );
    }

    #[test]
    fn test_identifier_with_numbers_at_the_end() {
        let result = identifier().parse("myIdentifier123");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            Identifier {
                name: "myIdentifier123".to_string()
            }
        );
    }

    #[test]
    fn test_identifier_starting_with_numbers() {
        let result = identifier().parse("123InvalidIdentifier");
        assert!(result.has_errors());
        assert!(!result.has_output());
    }

    #[test]
    fn test_identifier_empty() {
        let result = identifier().parse("");
        assert!(result.has_errors());
        assert!(!result.has_output());
    }

    #[test]
    fn test_identifier_with_special_characters() {
        let result = identifier().parse("myIdentifier@");
        assert!(result.has_errors());
        assert!(!result.has_output());
    }

    #[test]
    fn test_type_identifier_builtin_types() {
        for (type_str, expected_type) in [
            ("int8", TypeIdentifier::Integer8),
            ("int16", TypeIdentifier::Integer16),
            ("int32", TypeIdentifier::Integer32),
            ("int64", TypeIdentifier::Integer64),
            ("uint8", TypeIdentifier::UnsignedInteger8),
            ("uint16", TypeIdentifier::UnsignedInteger16),
            ("uint32", TypeIdentifier::UnsignedInteger32),
            ("uint64", TypeIdentifier::UnsignedInteger64),
            ("float32", TypeIdentifier::Float32),
            ("float64", TypeIdentifier::Float64),
            ("bit", TypeIdentifier::Bit),
            ("byte", TypeIdentifier::Byte),
        ] {
            let result = type_identifier().parse(type_str);
            assert!(!result.has_errors() && result.has_output());
            assert_eq!(result.into_output().unwrap(), expected_type);
        }
    }

    #[test]
    fn test_type_identifier_user_defined() {
        let result = type_identifier().parse("MyCustomType");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            TypeIdentifier::UserDefined(Identifier::new("MyCustomType"))
        );
    }

    #[test]
    fn test_type_identifier_static_array() {
        let result = type_identifier().parse("MyType[10]");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            TypeIdentifier::StaticArray {
                r#type: Box::new(TypeIdentifier::UserDefined(Identifier::new("MyType"))),
                size: 10,
            }
        );
    }

    #[test]
    fn test_type_identifier_static_array_with_builtin_type() {
        let result = type_identifier().parse("int32[5]");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            TypeIdentifier::StaticArray {
                r#type: Box::new(TypeIdentifier::Integer32),
                size: 5,
            }
        );
    }

    #[test]
    fn test_type_identifier_static_array_with_not_a_number() {
        let result = type_identifier().parse("int32[invalid]");
        assert!(result.has_errors());
        assert!(!result.has_output());
    }

    #[test]
    fn test_type_identifier_static_array_with_negative_size() {
        let result = type_identifier().parse("int32[-5]");
        assert!(result.has_errors());
        assert!(!result.has_output());
    }

    #[test]
    fn test_type_identifier_dynamic_array() {
        let result = type_identifier().parse("MyType[]");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            TypeIdentifier::DynamicArray {
                r#type: Box::new(TypeIdentifier::UserDefined(Identifier::new("MyType"))),
            }
        );
    }

    #[test]
    fn test_type_identifier_dynamic_array_with_builtin_type() {
        let result = type_identifier().parse("int32[]");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            TypeIdentifier::DynamicArray {
                r#type: Box::new(TypeIdentifier::Integer32),
            }
        );
    }

    #[test]
    fn test_enumeration_field_single_value() {
        let result = enumeration_field_single_value().parse("myField = 42;");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            EnumerationField::SingleValue {
                name: Identifier::new("myField"),
                value: 42
            }
        );
    }

    #[test]
    fn test_enumeration_field_signle_value_invalid_syntax() {
        let result = enumeration_field_single_value().parse("myField 42;");
        assert!(result.has_errors());
        assert!(!result.has_output());
    }

    #[test]
    fn test_range() {
        let result = range().parse("10..20");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(result.into_output().unwrap(), (10, 20));
    }

    #[test]
    fn test_range_with_too_many_dots() {
        let result = range().parse("10...20");
        assert!(result.has_errors());
        assert!(!result.has_output());
    }

    #[test]
    fn test_range_without_start() {
        let result = range().parse("..20");
        assert!(result.has_errors());
        assert!(!result.has_output());
    }

    #[test]
    fn test_range_without_end() {
        let result = range().parse("10..");
        assert!(result.has_errors());
        assert!(!result.has_output());
    }

    #[test]
    fn test_range_with_spaces() {
        let result = range().parse(" 10 .. 20 ");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(result.into_output().unwrap(), (10, 20));
    }

    #[test]
    fn test_enumeration_field_range_of_values() {
        let result = enumeration_field_range_of_values().parse("myRange = 10..20;");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            EnumerationField::RangeOfValues {
                name: Identifier::new("myRange"),
                start: 10,
                end: 20
            }
        );
    }

    #[test]
    fn test_enumeration_field_range_of_values_invalid_syntax() {
        let result = enumeration_field_range_of_values().parse("myRange 10..20;");
        assert!(result.has_errors());
        assert!(!result.has_output());
    }

    #[test]
    fn test_enumeration_field_range_of_values_too_much_dots() {
        let result = enumeration_field_range_of_values().parse("myRange = 10...20;");
        assert!(result.has_errors());
        assert!(!result.has_output());
    }

    #[test]
    fn test_enumeration_field_range_of_values_without_start() {
        let result = enumeration_field_range_of_values().parse("myRange = ..20;");
        assert!(result.has_errors());
        assert!(!result.has_output());
    }

    #[test]
    fn test_enumeration_field_range_of_values_without_end() {
        let result = enumeration_field_range_of_values().parse("myRange = 10..;");
        assert!(result.has_errors());
        assert!(!result.has_output());
    }

    #[test]
    fn test_enumeration_field_uses_proper_choice() {
        let result = enumeration_field().parse("myField = 42;");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            EnumerationField::SingleValue {
                name: Identifier::new("myField"),
                value: 42
            }
        );

        let result = enumeration_field().parse("myRange = 10..20;");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            EnumerationField::RangeOfValues {
                name: Identifier::new("myRange"),
                start: 10,
                end: 20
            }
        );
    }

    #[test]
    fn test_enumeration() {
        let result =
            enumeration_definition().parse("enum MyEnum { myField = 42; myRange = 10..20; };");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            Enumeration {
                name: Identifier::new("MyEnum"),
                fields: vec![
                    EnumerationField::SingleValue {
                        name: Identifier::new("myField"),
                        value: 42
                    },
                    EnumerationField::RangeOfValues {
                        name: Identifier::new("myRange"),
                        start: 10,
                        end: 20
                    }
                ],
            }
        );
    }

    #[test]
    fn test_enumeration_with_multiline_input() {
        let input = "enum MyEnum {
            myField = 42;
            myRange = 10..20;
        };";
        let result = enumeration_definition().parse(input);
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            Enumeration {
                name: Identifier::new("MyEnum"),
                fields: vec![
                    EnumerationField::SingleValue {
                        name: Identifier::new("myField"),
                        value: 42
                    },
                    EnumerationField::RangeOfValues {
                        name: Identifier::new("myRange"),
                        start: 10,
                        end: 20
                    }
                ],
            }
        );
    }

    #[test]
    fn test_enumeration_with_newline_breaks() {
        let input = "enum MyEnum {\r\nmyField = 42;\nmyRange = 10..20;\n};";
        let result = enumeration_definition().parse(input);
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            Enumeration {
                name: Identifier::new("MyEnum"),
                fields: vec![
                    EnumerationField::SingleValue {
                        name: Identifier::new("myField"),
                        value: 42
                    },
                    EnumerationField::RangeOfValues {
                        name: Identifier::new("myRange"),
                        start: 10,
                        end: 20
                    }
                ],
            }
        );
    }

    #[test]
    fn test_enumeration_without_identifier() {
        let result = enumeration_definition().parse("enum { myField = 42; };");
        assert!(result.has_errors());
        assert!(!result.has_output());
    }

    #[test]
    fn test_enumeration_without_fields() {
        let result = enumeration_definition().parse("enum MyEnum { };");
        assert!(result.has_errors());
        assert!(!result.has_output());
    }

    #[test]
    fn test_structure_field() {
        let result = structure_field().parse("myField: int32;");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            StructureField {
                name: Identifier::new("myField"),
                r#type: TypeIdentifier::Integer32,
            }
        );
    }

    #[test]
    fn test_structure_field_with_user_defined_type() {
        let result = structure_field().parse("myField: MyCustomType;");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            StructureField {
                name: Identifier::new("myField"),
                r#type: TypeIdentifier::UserDefined(Identifier::new("MyCustomType")),
            }
        );
    }

    #[test]
    fn test_structure_with_static_array() {
        let result = structure_field().parse("myField: int32[10];");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            StructureField {
                name: Identifier::new("myField"),
                r#type: TypeIdentifier::StaticArray {
                    r#type: Box::new(TypeIdentifier::Integer32),
                    size: 10,
                },
            }
        );
    }

    #[test]
    fn test_structure_with_dynamic_array() {
        let result = structure_field().parse("myField: uint64[];");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            StructureField {
                name: Identifier::new("myField"),
                r#type: TypeIdentifier::DynamicArray {
                    r#type: Box::new(TypeIdentifier::UnsignedInteger64),
                },
            }
        );
    }

    #[test]
    fn test_structure() {
        let result =
            structure_definition().parse("struct MyStruct { myField: int32; myArray: uint64[]; };");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            Structure {
                name: Identifier::new("MyStruct"),
                fields: vec![
                    StructureField {
                        name: Identifier::new("myField"),
                        r#type: TypeIdentifier::Integer32,
                    },
                    StructureField {
                        name: Identifier::new("myArray"),
                        r#type: TypeIdentifier::DynamicArray {
                            r#type: Box::new(TypeIdentifier::UnsignedInteger64),
                        },
                    }
                ],
            }
        );
    }

    #[test]
    fn test_structure_with_multiline_input() {
        let input = "struct MyStruct {\n    myField: int32;\n    myArray: uint64[];\n};";
        let result = structure_definition().parse(input);
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            Structure {
                name: Identifier::new("MyStruct"),
                fields: vec![
                    StructureField {
                        name: Identifier::new("myField"),
                        r#type: TypeIdentifier::Integer32,
                    },
                    StructureField {
                        name: Identifier::new("myArray"),
                        r#type: TypeIdentifier::DynamicArray {
                            r#type: Box::new(TypeIdentifier::UnsignedInteger64),
                        },
                    }
                ],
            }
        );
    }

    #[test]
    fn test_structure_without_identifier() {
        let result = structure_definition().parse("struct { myField: int32; };");
        assert!(result.has_errors());
        assert!(!result.has_output());
    }

    #[test]
    fn test_structure_without_fields() {
        let result = structure_definition().parse("struct MyStruct { };");
        assert!(result.has_errors());
        assert!(!result.has_output());
    }

    #[test]
    fn test_union_field() {
        let result = union_field().parse("1 => myField: int32;");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            UnionField {
                name: Identifier::new("myField"),
                r#type: TypeIdentifier::Integer32,
                discriminator: 1,
            }
        );
    }

    #[test]
    fn test_union_field_with_user_defined_type() {
        let result = union_field().parse("2 => myField: MyCustomType;");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            UnionField {
                name: Identifier::new("myField"),
                r#type: TypeIdentifier::UserDefined(Identifier::new("MyCustomType")),
                discriminator: 2,
            }
        );
    }

    #[test]
    fn test_union_field_with_static_array() {
        let result = union_field().parse("3 => myField: int32[10];");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            UnionField {
                name: Identifier::new("myField"),
                r#type: TypeIdentifier::StaticArray {
                    r#type: Box::new(TypeIdentifier::Integer32),
                    size: 10,
                },
                discriminator: 3,
            }
        );
    }

    #[test]
    fn test_union_field_with_dynamic_array() {
        let result = union_field().parse("4 => myField: uint64[];");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            UnionField {
                name: Identifier::new("myField"),
                r#type: TypeIdentifier::DynamicArray {
                    r#type: Box::new(TypeIdentifier::UnsignedInteger64),
                },
                discriminator: 4,
            }
        );
    }

    #[test]
    fn test_union() {
        let input = "union MyUnion { 1 => myField: int32; 2 => myArray: uint64[]; };";
        let result = union_definition().parse(input);
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            Union {
                name: Identifier::new("MyUnion"),
                fields: vec![
                    UnionField {
                        name: Identifier::new("myField"),
                        r#type: TypeIdentifier::Integer32,
                        discriminator: 1,
                    },
                    UnionField {
                        name: Identifier::new("myArray"),
                        r#type: TypeIdentifier::DynamicArray {
                            r#type: Box::new(TypeIdentifier::UnsignedInteger64),
                        },
                        discriminator: 2,
                    }
                ],
            }
        );
    }

    #[test]
    fn test_union_with_multiline_input() {
        let input = "union MyUnion {\n    1 => myField: int32;\n    2 => myArray: uint64[];\n};";
        let result = union_definition().parse(input);
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            Union {
                name: Identifier::new("MyUnion"),
                fields: vec![
                    UnionField {
                        name: Identifier::new("myField"),
                        r#type: TypeIdentifier::Integer32,
                        discriminator: 1,
                    },
                    UnionField {
                        name: Identifier::new("myArray"),
                        r#type: TypeIdentifier::DynamicArray {
                            r#type: Box::new(TypeIdentifier::UnsignedInteger64),
                        },
                        discriminator: 2,
                    }
                ],
            }
        );
    }

    #[test]
    fn test_union_without_identifier() {
        let result = union_definition().parse("union { 1 => myField: int32; };");
        assert!(result.has_errors());
        assert!(!result.has_output());
    }

    #[test]
    fn test_type_definition() {
        let result = type_definition().parse("using MyType = int32;");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            TypeDefinition {
                new_type: Identifier::new("MyType"),
                r#type: TypeIdentifier::Integer32,
            }
        );
    }

    #[test]
    fn test_type_definition_with_user_defined_type() {
        let result = type_definition().parse("using MyType = MyCustomType;");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            TypeDefinition {
                new_type: Identifier::new("MyType"),
                r#type: TypeIdentifier::UserDefined(Identifier::new("MyCustomType")),
            }
        );
    }

    #[test]
    fn test_type_definition_with_static_array() {
        let result = type_definition().parse("using MyType = int32[10];");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            TypeDefinition {
                new_type: Identifier::new("MyType"),
                r#type: TypeIdentifier::StaticArray {
                    r#type: Box::new(TypeIdentifier::Integer32),
                    size: 10,
                },
            }
        );
    }

    #[test]
    fn test_type_definition_with_dynamic_array() {
        let result = type_definition().parse("using MyType = uint64[];");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            TypeDefinition {
                new_type: Identifier::new("MyType"),
                r#type: TypeIdentifier::DynamicArray {
                    r#type: Box::new(TypeIdentifier::UnsignedInteger64),
                },
            }
        );
    }

    #[test]
    fn test_definition_with_enumeration() {
        let input = "enum MyEnum { myField = 42; myRange = 10..20; };";
        let result = definition().parse(input);
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            Definition::Enumeration(Enumeration {
                name: Identifier::new("MyEnum"),
                fields: vec![
                    EnumerationField::SingleValue {
                        name: Identifier::new("myField"),
                        value: 42
                    },
                    EnumerationField::RangeOfValues {
                        name: Identifier::new("myRange"),
                        start: 10,
                        end: 20
                    }
                ],
            })
        );
    }

    #[test]
    fn test_definition_with_structure() {
        let input = "struct MyStruct { myField: int32; myArray: uint64[]; };";
        let result = definition().parse(input);
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            Definition::Structure(Structure {
                name: Identifier::new("MyStruct"),
                fields: vec![
                    StructureField {
                        name: Identifier::new("myField"),
                        r#type: TypeIdentifier::Integer32,
                    },
                    StructureField {
                        name: Identifier::new("myArray"),
                        r#type: TypeIdentifier::DynamicArray {
                            r#type: Box::new(TypeIdentifier::UnsignedInteger64),
                        },
                    }
                ],
            })
        );
    }

    #[test]
    fn test_definition_with_union() {
        let input = "union MyUnion { 1 => myField: int32; 2 => myArray: uint64[]; };";
        let result = definition().parse(input);
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            Definition::Union(Union {
                name: Identifier::new("MyUnion"),
                fields: vec![
                    UnionField {
                        name: Identifier::new("myField"),
                        r#type: TypeIdentifier::Integer32,
                        discriminator: 1,
                    },
                    UnionField {
                        name: Identifier::new("myArray"),
                        r#type: TypeIdentifier::DynamicArray {
                            r#type: Box::new(TypeIdentifier::UnsignedInteger64),
                        },
                        discriminator: 2,
                    }
                ],
            })
        );
    }

    #[test]
    fn test_definition_with_type_definition() {
        let input = "using MyType = int32;";
        let result = definition().parse(input);
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            Definition::TypeDefinition(TypeDefinition {
                new_type: Identifier::new("MyType"),
                r#type: TypeIdentifier::Integer32,
            })
        );
    }

    #[test]
    fn test_protocol() {
        let input = r#"
using MyType = int32[10];

enum MyEnum {
    myField = 42;
    myRange = 10..20;
};

struct MyStruct {
    myField: int32;
    myArray: uint64[];
    myType: MyType;
};

union MyUnion {
    1 => myField: int32;
    2 => myArray: uint64[];
};
"#;

        let result = protocol().parse(input);
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            Protocol {
                definitions: vec![
                    Definition::TypeDefinition(TypeDefinition {
                        new_type: Identifier::new("MyType"),
                        r#type: TypeIdentifier::StaticArray {
                            r#type: Box::new(TypeIdentifier::Integer32),
                            size: 10,
                        },
                    }),
                    Definition::Enumeration(Enumeration {
                        name: Identifier::new("MyEnum"),
                        fields: vec![
                            EnumerationField::SingleValue {
                                name: Identifier::new("myField"),
                                value: 42
                            },
                            EnumerationField::RangeOfValues {
                                name: Identifier::new("myRange"),
                                start: 10,
                                end: 20
                            }
                        ],
                    }),
                    Definition::Structure(Structure {
                        name: Identifier::new("MyStruct"),
                        fields: vec![
                            StructureField {
                                name: Identifier::new("myField"),
                                r#type: TypeIdentifier::Integer32,
                            },
                            StructureField {
                                name: Identifier::new("myArray"),
                                r#type: TypeIdentifier::DynamicArray {
                                    r#type: Box::new(TypeIdentifier::UnsignedInteger64),
                                },
                            },
                            StructureField {
                                name: Identifier::new("myType"),
                                r#type: TypeIdentifier::UserDefined(Identifier::new("MyType")),
                            }
                        ],
                    }),
                    Definition::Union(Union {
                        name: Identifier::new("MyUnion"),
                        fields: vec![
                            UnionField {
                                name: Identifier::new("myField"),
                                r#type: TypeIdentifier::Integer32,
                                discriminator: 1,
                            },
                            UnionField {
                                name: Identifier::new("myArray"),
                                r#type: TypeIdentifier::DynamicArray {
                                    r#type: Box::new(TypeIdentifier::UnsignedInteger64),
                                },
                                discriminator: 2,
                            }
                        ],
                    }),
                ],
            }
        );
    }
}
