//! Grammar for the meklang is defined as follows:
//! ```text
//! <protocol> ::= (<definition> | <comment>)+
//! <comment> ::= '#' <text> '\n'
//! <definition> ::=
//!       <enumeration_definition>
//!     | <structure_definition>
//!     | <union_definition>
//!     | <type_definition>
//!
//! <enumeration_definition> ::= 'enum' <identifier> <left_brace> <enumeration_field>+ <right_brace> <semicolon>
//! <enumeration_field> ::= <identifier> <equal> (<unsigned_integer> | <range>) <semicolon>
//!
//! <structure_definition> ::= 'struct' <identifier> <left_brace> <structure_field>+ <right_brace> <semicolon>
//! <structure_field> ::= [<attributes>] <identifier> <colon> <type_identifier> <semicolon>
//!
//! <union_definition> ::= 'union' <identifier> <left_brace> <union_field>+ <right_brace> <semicolon>
//! <union_field> ::= (<unsigned_integer> | <range>) <maps_to> <identifier> <colon> <type_identifier> <semicolon>
//!
//! <attribute> ::=
//!       'discriminated_by' <equal> <identifier>
//!     | 'bits' <equal> <unsigned_integer>
//!     | 'bytes' <equal> <unsigned_integer>
//! <attribute_tail> ::= <comma> <attribute>
//! <attributes> ::= <left_bracket> <attribute> <attribute_tail>* <right_bracket>
//!
//! <type_definition> ::= 'using' <identifier> <equal> <type_identifier> <semicolon>
//!
//! <type_identifier> ::=
//!       <builtin_type>
//!     | <user_defined_type>
//!     | <static_array_type>
//!     | <dynamic_array_type>
//!
//! <builtin_type> ::=
//!       'int8' | 'int16' | 'int32' | 'int64'
//!     | 'uint8' | 'uint16' | 'uint32' | 'uint64'
//!     | 'float32' | 'float64'
//!     | 'bit' | 'byte'
//! <user_defined_type> ::= <identifier>
//! <static_array_type> ::=
//!       <builtin_type> <left_bracket> <unsigned_integer> <right_bracket>
//!     | <user_defined_type> <left_bracket> <unsigned_integer> <right_bracket>
//! <dynamic_array_type> ::=
//!       <builtin_type> <left_bracket> <right_bracket>
//!     | <user_defined_type> <left_bracket> <right_bracket>
//!
//! <range> ::= <unsigned_integer> <double_dot> <unsigned_integer>
//! <identifier> ::= [a-zA-Z_][a-zA-Z0-9_]*
//!
//! <unsigned_integer> ::= <hexadecimal> | <binary> | <decimal>
//! <hexadecimal> ::= "0x" [0-9a-fA-F]+
//! <binary> ::= "0b" [01]+
//! <decimal> ::= [0-9]+
//!
//! <text> ::= [^\n]*
//!
//! <left_brace> ::= '{'
//! <right_brace> ::= '}'
//! <left_bracket> ::= '['
//! <right_bracket> ::= ']'
//! <semicolon> ::= <semicolon>
//! <colon> ::= ':'
//! <maps_to> ::= '=>'
//! <equal> ::= '='
//! <comma> ::= ','
//! <double_dot> ::= '..'
//! ```
//!
//! This grammar defines the structure of a protocol of the meklang, whose
//! main purpose is to define data structures and types that can be used in code generation.
//!
//! Currently `<comment>` is supported only in between definitions, but not inside them.

use crate::ast::*;

use chumsky::prelude::*;

pub(crate) type RichError<'src> = chumsky::error::Rich<'src, char>;
pub(crate) type ErrorType<'src> = extra::Err<RichError<'src>>;

/// Parses a left brace `{` followed by optional whitespace.
pub(crate) fn left_brace<'src>() -> impl Parser<'src, &'src str, (), ErrorType<'src>> {
    just('{').padded().to(()).labelled("left brace ({)")
}

/// Parses a left brace `}` followed by optional whitespace.
pub(crate) fn right_brace<'src>() -> impl Parser<'src, &'src str, (), ErrorType<'src>> {
    just('}').padded().to(()).labelled("right brace (})")
}

/// Parses a left bracket `[` followed by optional whitespace.
pub(crate) fn left_bracket<'src>() -> impl Parser<'src, &'src str, (), ErrorType<'src>> {
    just('[').padded().to(()).labelled("left bracket ([)")
}

/// Parses a right bracket `]` followed by optional whitespace.
pub(crate) fn right_bracket<'src>() -> impl Parser<'src, &'src str, (), ErrorType<'src>> {
    just(']').padded().to(()).labelled("right bracket (])")
}

/// Parses a semicolon `;` followed by optional whitespace.
pub(crate) fn semicolon<'src>() -> impl Parser<'src, &'src str, (), ErrorType<'src>> {
    just(';').padded().to(()).labelled("semicolon (;)")
}

/// Parses a colon `:` followed by optional whitespace.
pub(crate) fn colon<'src>() -> impl Parser<'src, &'src str, (), ErrorType<'src>> {
    just(':').padded().to(()).labelled("colon (:)")
}

/// Parses a maps to operator `=>` followed by optional whitespace.
pub(crate) fn maps_to<'src>() -> impl Parser<'src, &'src str, (), ErrorType<'src>> {
    just("=>").padded().to(()).labelled("maps to (=>)")
}

/// Parses an equal sign `=` followed by optional whitespace.
pub(crate) fn equal<'src>() -> impl Parser<'src, &'src str, (), ErrorType<'src>> {
    just('=').padded().to(()).labelled("equal (=)")
}

/// Parses a comma `,` followed by optional whitespace.
pub(crate) fn comma<'src>() -> impl Parser<'src, &'src str, (), ErrorType<'src>> {
    just(',').padded().to(()).labelled("comma (,)")
}

/// Parses a double dot `..` followed by optional whitespace.
pub(crate) fn double_dot<'src>() -> impl Parser<'src, &'src str, (), ErrorType<'src>> {
    just("..").padded().to(()).labelled("double dot (..)")
}

/// Parses an unsigned integer in hexadecimal format.
pub(crate) fn hexadecimal<'src>() -> impl Parser<'src, &'src str, u64, ErrorType<'src>> {
    just("0x")
        .ignore_then(text::digits(16).at_least(1).collect::<String>())
        .map(|s: String| u64::from_str_radix(&s, 16).unwrap())
        .labelled("hexadecimal")
        .padded()
}

/// Parses an unsigned integer in binary format. It supports leading zeros and
/// only allows `0` and `1` digits.
pub(crate) fn binary<'src>() -> impl Parser<'src, &'src str, u64, ErrorType<'src>> {
    just("0b")
        .ignore_then(text::digits(2).at_least(1).collect::<String>())
        .map(|s: String| u64::from_str_radix(&s, 2).unwrap())
        .labelled("binary")
        .padded()
}

/// Parses an unsigned integer in decimal format.
pub(crate) fn decimal<'src>() -> impl Parser<'src, &'src str, u64, ErrorType<'src>> {
    text::digits(10)
        .at_least(1)
        .collect::<String>()
        .map(|s: String| s.parse::<u64>().unwrap())
        .labelled("decimal")
        .padded()
}

/// Parses an unsigned integer in decimal, hexadecimal, or binary format.
pub(crate) fn unsigned_integer<'src>() -> impl Parser<'src, &'src str, u64, ErrorType<'src>> {
    choice((hexadecimal(), binary(), decimal())).labelled("unsigned_integer")
}

/// Parses an identifier from the input string. Identifier has to start with
/// either alphabetic characters or an underscore, followed by alphanumeric
/// characters or underscores.
pub(crate) fn identifier<'src>() -> impl Parser<'src, &'src str, Identifier, ErrorType<'src>> {
    text::ident()
        .map(|s: &str| Identifier::new(s))
        .labelled("identifier")
        .padded()
}

/// Parses a built-in type identifier from the input string.
pub(crate) fn builtin_type<'src>() -> impl Parser<'src, &'src str, TypeIdentifier, ErrorType<'src>>
{
    choice((
        just("int8").to(TypeIdentifier::Integer8),
        just("int16").to(TypeIdentifier::Integer16),
        just("int32").to(TypeIdentifier::Integer32),
        just("int64").to(TypeIdentifier::Integer64),
        just("uint8").to(TypeIdentifier::UnsignedInteger8),
        just("uint16").to(TypeIdentifier::UnsignedInteger16),
        just("uint32").to(TypeIdentifier::UnsignedInteger32),
        just("uint64").to(TypeIdentifier::UnsignedInteger64),
        just("float32").to(TypeIdentifier::Float32),
        just("float64").to(TypeIdentifier::Float64),
        just("bit").to(TypeIdentifier::Bit),
        just("byte").to(TypeIdentifier::Byte),
    ))
    .labelled("builtin type")
}

/// Parses a user-defined type identifier from the input string.
pub(crate) fn user_defined_type<'src>()
-> impl Parser<'src, &'src str, TypeIdentifier, ErrorType<'src>> {
    identifier()
        .map(TypeIdentifier::UserDefined)
        .labelled("user defined type")
        .padded()
}

/// Parses a static array type identifier from the input string.
pub(crate) fn static_array_type<'src>()
-> impl Parser<'src, &'src str, TypeIdentifier, ErrorType<'src>> {
    choice((builtin_type(), user_defined_type()))
        .then_ignore(left_bracket())
        .then(unsigned_integer())
        .then_ignore(right_bracket())
        .map(|(r#type, size)| TypeIdentifier::StaticArray {
            r#type: Box::new(r#type),
            size,
        })
        .labelled("static array type")
        .padded()
}

pub(crate) fn dynamic_array_type<'src>()
-> impl Parser<'src, &'src str, TypeIdentifier, ErrorType<'src>> {
    choice((builtin_type(), user_defined_type()))
        .then_ignore(left_bracket())
        .then_ignore(right_bracket())
        .map(|r#type| TypeIdentifier::DynamicArray {
            r#type: Box::new(r#type),
        })
        .labelled("dynamic array type")
        .padded()
}

/// Parses a type identifier from the input string. It can be a predefined type
/// like `int8`, `uint16`, `float32`, etc., or a user-defined type.
/// It can also be a static or dynamic array of a given type.
/// The static array is defined as `type[size]`, and the dynamic array is defined as `type[]`.
pub(crate) fn type_identifier<'src>()
-> impl Parser<'src, &'src str, TypeIdentifier, ErrorType<'src>> {
    recursive(|_| {
        choice((
            static_array_type().boxed(),
            dynamic_array_type().boxed(),
            builtin_type().boxed(),
            user_defined_type().boxed(),
        ))
    })
}

/// Parses a single value enumeration field in the format `name = value;`
pub(crate) fn enumeration_field_single_value<'src>()
-> impl Parser<'src, &'src str, EnumerationField, ErrorType<'src>> {
    identifier()
        .then_ignore(equal())
        .then(unsigned_integer())
        .then_ignore(semicolon())
        .map(|(name, value)| EnumerationField::SingleValue { name, value })
        .labelled("enumeration field single value")
        .padded()
}

/// Parses a range of values defined by `start..end`.
pub(crate) fn range<'src>() -> impl Parser<'src, &'src str, (u64, u64), ErrorType<'src>> {
    unsigned_integer()
        .then_ignore(double_dot())
        .then(unsigned_integer())
        .map(|(start, end)| (start, end))
        .labelled("range")
        .padded()
}

/// Parses a range of values enumeration field in the format `name = start..end;`
pub(crate) fn enumeration_field_range_of_values<'src>()
-> impl Parser<'src, &'src str, EnumerationField, ErrorType<'src>> {
    identifier()
        .then_ignore(equal())
        .then(range())
        .then_ignore(semicolon())
        .map(|(name, (start, end))| EnumerationField::RangeOfValues { name, start, end })
        .labelled("enumeration field range of values")
        .padded()
}

/// Parses an enumeration field from the input string.
pub(crate) fn enumeration_field<'src>()
-> impl Parser<'src, &'src str, EnumerationField, ErrorType<'src>> {
    choice((
        enumeration_field_single_value(),
        enumeration_field_range_of_values(),
    ))
    .labelled("enumeration field")
    .padded()
}

/// Parses an enumeration with fields.
pub(crate) fn enumeration_definition<'src>()
-> impl Parser<'src, &'src str, EnumerationDefinition, ErrorType<'src>> {
    just("enum")
        .padded()
        .ignore_then(identifier())
        .then_ignore(left_brace())
        .then(
            enumeration_field()
                .repeated()
                .at_least(1)
                .collect::<Vec<EnumerationField>>(),
        )
        .then_ignore(right_brace())
        .then_ignore(semicolon())
        .map(|(name, fields)| EnumerationDefinition { name, fields })
        .labelled("enumeration")
        .padded()
}

/// Parses a single structure field attribute, which consists of a name and a value.
pub(crate) fn attribute<'src>() -> impl Parser<'src, &'src str, Attribute, ErrorType<'src>> {
    choice((
        just("discriminated_by")
            .ignore_then(equal())
            .ignore_then(identifier())
            .map(|field| Attribute::DiscriminatedBy { field }),
        just("bits")
            .ignore_then(equal())
            .ignore_then(unsigned_integer())
            .map(|size| Attribute::BitsSize { size }),
        just("bytes")
            .ignore_then(equal())
            .ignore_then(unsigned_integer())
            .map(|size| Attribute::BytesSize { size }),
    ))
    .labelled("attribute")
    .padded()
}

/// Parses a structure field attribute tail, which is a comma followed by another attribute.
pub(crate) fn attribute_tail<'src>() -> impl Parser<'src, &'src str, Attribute, ErrorType<'src>> {
    comma()
        .padded()
        .ignore_then(attribute())
        .labelled("attribute tail")
        .padded()
}

/// Parses a collection of structure field attributes, which are enclosed in square brackets
/// and separated by commas.
pub(crate) fn attributes<'src>() -> impl Parser<'src, &'src str, Vec<Attribute>, ErrorType<'src>> {
    left_bracket()
        .padded()
        .ignore_then(
            attribute()
                .then(attribute_tail().repeated().collect::<Vec<_>>())
                .map(|(first, rest)| {
                    let mut attrs = vec![first];
                    attrs.extend(rest);
                    attrs
                }),
        )
        .then_ignore(right_bracket())
        .labelled("attributes")
        .padded()
}

/// Parses a structure field, which consists of a name and a type identifier.
pub(crate) fn structure_field<'src>()
-> impl Parser<'src, &'src str, StructureField, ErrorType<'src>> {
    attributes()
        .or_not()
        .map(|attrs| attrs.unwrap_or_default())
        .then(identifier())
        .then_ignore(colon())
        .then(type_identifier())
        .then_ignore(semicolon())
        .map(|((attributes, name), r#type)| StructureField {
            attributes,
            name,
            r#type,
        })
        .labelled("structure field")
        .padded()
}

/// Parses a structure definition, which consists of a name and a collection of fields.
pub(crate) fn structure_definition<'src>()
-> impl Parser<'src, &'src str, StructureDefinition, ErrorType<'src>> {
    just("struct")
        .padded()
        .ignore_then(identifier())
        .then_ignore(left_brace())
        .then(
            structure_field()
                .repeated()
                .at_least(1)
                .collect::<Vec<StructureField>>(),
        )
        .then_ignore(right_brace())
        .then_ignore(semicolon())
        .map(|(name, fields)| StructureDefinition { name, fields })
        .labelled("structure definition")
        .padded()
}

/// Parses a union field with a single discriminator, which consists of a discriminator, name, and type identifier.
pub(crate) fn union_field_single_value<'src>()
-> impl Parser<'src, &'src str, UnionField, ErrorType<'src>> {
    unsigned_integer()
        .then_ignore(maps_to())
        .then(identifier())
        .then_ignore(colon())
        .then(type_identifier())
        .then_ignore(semicolon())
        .map(|((discriminator, name), r#type)| UnionField::SingleValue {
            name,
            r#type,
            discriminator,
        })
        .labelled("union field")
        .padded()
}

/// Parses a union field with a range of discriminators, which consists of a start and end discriminator, name, and type identifier.
pub(crate) fn union_field_range_of_values<'src>()
-> impl Parser<'src, &'src str, UnionField, ErrorType<'src>> {
    range()
        .then_ignore(maps_to())
        .then(identifier())
        .then_ignore(colon())
        .then(type_identifier())
        .then_ignore(semicolon())
        .map(
            |(((start_discriminator, end_discriminator), name), r#type)| {
                UnionField::RangeOfValues {
                    name,
                    r#type,
                    start_discriminator,
                    end_discriminator,
                }
            },
        )
        .labelled("union field range of values")
        .padded()
}

/// Parses a union field, which can either be a single value or a range of values.
pub(crate) fn union_field<'src>() -> impl Parser<'src, &'src str, UnionField, ErrorType<'src>> {
    choice((union_field_single_value(), union_field_range_of_values()))
        .labelled("union field")
        .padded()
}

/// Parses a union definition, which consists of a name and a collection of union fields.
pub(crate) fn union_definition<'src>()
-> impl Parser<'src, &'src str, UnionDefinition, ErrorType<'src>> {
    just("union")
        .padded()
        .ignore_then(identifier())
        .then_ignore(left_brace())
        .then(
            union_field()
                .repeated()
                .at_least(1)
                .collect::<Vec<UnionField>>(),
        )
        .then_ignore(right_brace())
        .then_ignore(semicolon())
        .map(|(name, fields)| UnionDefinition { name, fields })
        .labelled("union")
        .padded()
}

/// Parses a type definition, which consists of a new type name and an existing type.
pub(crate) fn type_definition<'src>()
-> impl Parser<'src, &'src str, TypeDefinition, ErrorType<'src>> {
    just("using")
        .padded()
        .ignore_then(identifier())
        .then_ignore(equal())
        .then(type_identifier())
        .then_ignore(semicolon())
        .map(|(new_type, r#type)| TypeDefinition { new_type, r#type })
        .labelled("type definition")
        .padded()
}

/// Parses a single definition, which can be an enumeration, structure, union, or type definition.
pub(crate) fn definition<'src>() -> impl Parser<'src, &'src str, Definition, ErrorType<'src>> {
    choice((
        enumeration_definition().map(Definition::Enumeration),
        structure_definition().map(Definition::Structure),
        union_definition().map(Definition::Union),
        type_definition().map(Definition::Type),
    ))
    .labelled("definition")
    .padded()
}

/// Parses a comment which is the whole line starting with `#` and ending with a newline.
pub(crate) fn comment<'src>() -> impl Parser<'src, &'src str, (), ErrorType<'src>> {
    just('#')
        .ignore_then(
            any()
                .filter(|c| *c != '\n' && *c != '\r')
                .repeated()
                .ignore_then(text::newline().or(end())),
        )
        .map(|_| ())
        .labelled("comment")
        .padded()
}

/// Parses the entire protocol, which consists of multiple definitions and comments
/// that can be mixed (i.e. definition, comment, definition, definition, comment, etc.).
pub(crate) fn protocol<'src>() -> impl Parser<'src, &'src str, Protocol, ErrorType<'src>> {
    // Accept either a definition or a comment, and collect only definitions
    choice((definition().map(Some), comment().to(None)))
        .repeated()
        .collect::<Vec<Option<Definition>>>()
        .map(|items| {
            let definitions = items.into_iter().flatten().collect();
            Protocol { definitions }
        })
        .labelled("protocol")
        .padded()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_brace() {
        let result = left_brace().parse("{");
        assert!(!result.has_errors() && result.has_output());
    }

    #[test]
    fn test_left_brace_with_whitespaces() {
        let result = left_brace().parse("   {   ");
        assert!(!result.has_errors() && result.has_output());
    }

    #[test]
    fn test_right_brace() {
        let result = right_brace().parse("}");
        assert!(!result.has_errors() && result.has_output());
    }

    #[test]
    fn test_right_brace_with_whitespaces() {
        let result = right_brace().parse("   }   ");
        assert!(!result.has_errors() && result.has_output());
    }

    #[test]
    fn test_left_bracket() {
        let result = left_bracket().parse("[");
        assert!(!result.has_errors() && result.has_output());
    }

    #[test]
    fn test_left_bracket_with_whitespaces() {
        let result = left_bracket().parse("   [   ");
        assert!(!result.has_errors() && result.has_output());
    }

    #[test]
    fn test_right_bracket() {
        let result = right_bracket().parse("]");
        assert!(!result.has_errors() && result.has_output());
    }

    #[test]
    fn test_right_bracket_with_whitespaces() {
        let result = right_bracket().parse("   ]   ");
        assert!(!result.has_errors() && result.has_output());
    }

    #[test]
    fn test_semicolon() {
        let result = semicolon().parse(";");
        assert!(!result.has_errors() && result.has_output());
    }

    #[test]
    fn test_semicolon_with_whitespaces() {
        let result = semicolon().parse("   ;   ");
        assert!(!result.has_errors() && result.has_output());
    }

    #[test]
    fn test_colon() {
        let result = colon().parse(":");
        assert!(!result.has_errors() && result.has_output());
    }

    #[test]
    fn test_colon_with_whitespaces() {
        let result = colon().parse("   :   ");
        assert!(!result.has_errors() && result.has_output());
    }

    #[test]
    fn test_maps_to() {
        let result = maps_to().parse("=>");
        assert!(!result.has_errors() && result.has_output());
    }

    #[test]
    fn test_maps_to_with_whitespaces() {
        let result = maps_to().parse("   =>   ");
        assert!(!result.has_errors() && result.has_output());
    }

    #[test]
    fn test_equal() {
        let result = equal().parse("=");
        assert!(!result.has_errors() && result.has_output());
    }

    #[test]
    fn test_equal_with_whitespaces() {
        let result = equal().parse("   =   ");
        assert!(!result.has_errors() && result.has_output());
    }

    #[test]
    fn test_comma() {
        let result = comma().parse(",");
        assert!(!result.has_errors() && result.has_output());
    }

    #[test]
    fn test_comma_with_whitespaces() {
        let result = comma().parse("   ,   ");
        assert!(!result.has_errors() && result.has_output());
    }

    #[test]
    fn test_double_dot() {
        let result = double_dot().parse("..");
        assert!(!result.has_errors() && result.has_output());
    }

    #[test]
    fn test_double_dot_with_whitespaces() {
        let result = double_dot().parse("   ..   ");
        assert!(!result.has_errors() && result.has_output());
    }

    #[test]
    fn test_hexadecimal() {
        let result = hexadecimal().parse("0x1A3F");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(result.into_output().unwrap(), 0x1A3F);

        let result = hexadecimal().parse("0x1a3f");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(result.into_output().unwrap(), 0x1A3F);
    }

    #[test]
    fn test_hexadecimal_with_zero_padding() {
        let result = hexadecimal().parse("0x00FF");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(result.into_output().unwrap(), 0xFF);
    }

    #[test]
    fn test_binary() {
        let result = binary().parse("0b1101");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(result.into_output().unwrap(), 0b1101);
    }

    #[test]
    fn test_binary_with_zero_padding() {
        let result = binary().parse("0b00001101");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(result.into_output().unwrap(), 0b1101);
    }

    #[test]
    fn test_decimal() {
        let result = decimal().parse("12345");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(result.into_output().unwrap(), 12345);
    }

    #[test]
    fn test_decimal_with_leading_zeros() {
        let result = decimal().parse("0012345");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(result.into_output().unwrap(), 12345);
    }

    #[test]
    fn test_unsigned_integer() {
        for value in ["5589", "0x15D5", "0b1010111010101"] {
            let result = unsigned_integer().parse(value);
            assert!(!result.has_errors() && result.has_output());
            assert_eq!(result.into_output().unwrap(), 5589);
        }
    }

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
    fn test_builtin_type() {
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
            let result = builtin_type().parse(type_str);
            assert!(!result.has_errors() && result.has_output());
            assert_eq!(result.into_output().unwrap(), expected_type);
        }
    }

    #[test]
    fn test_user_defined_type() {
        let result = user_defined_type().parse("MyCustomType");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            TypeIdentifier::UserDefined(Identifier::new("MyCustomType"))
        );
    }

    #[test]
    fn test_static_array_type_with_builtin_type() {
        let result = static_array_type().parse("int32[5]");
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
    fn test_static_array_type_with_custom_type() {
        let result = static_array_type().parse("MyType[10]");
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
    fn test_static_array_type_with_wrong_size() {
        let result = static_array_type().parse("int32[invalid]");
        assert!(result.has_errors());
        assert!(!result.has_output());
    }

    #[test]
    fn test_static_array_type_with_negative_size() {
        let result = static_array_type().parse("int32[-5]");
        assert!(result.has_errors());
        assert!(!result.has_output());
    }

    #[test]
    fn test_dynamic_array_type_with_builtin_type() {
        let result = dynamic_array_type().parse("uint64[]");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            TypeIdentifier::DynamicArray {
                r#type: Box::new(TypeIdentifier::UnsignedInteger64),
            }
        );
    }

    #[test]
    fn test_dynamic_array_type_with_user_defined_type() {
        let result = dynamic_array_type().parse("MyType[]");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            TypeIdentifier::DynamicArray {
                r#type: Box::new(TypeIdentifier::UserDefined(Identifier::new("MyType"))),
            }
        );
    }

    #[test]
    fn test_type_identifier_with_builtin_type() {
        let result = type_identifier().parse("int32");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(result.into_output().unwrap(), TypeIdentifier::Integer32);
    }

    #[test]
    fn test_type_identifier_with_user_defined_type() {
        let result = type_identifier().parse("MyCustomType");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            TypeIdentifier::UserDefined(Identifier::new("MyCustomType"))
        );
    }

    #[test]
    fn test_type_identifier_with_static_array() {
        let result = type_identifier().parse("int32[10]");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            TypeIdentifier::StaticArray {
                r#type: Box::new(TypeIdentifier::Integer32),
                size: 10,
            }
        );
    }

    #[test]
    fn test_type_identifier_with_dynamic_array() {
        let result = type_identifier().parse("uint64[]");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            TypeIdentifier::DynamicArray {
                r#type: Box::new(TypeIdentifier::UnsignedInteger64),
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
    fn test_range_with_hexadecimal_numbers() {
        let result = range().parse("0xA..0x14");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(result.into_output().unwrap(), (0xA, 0x14));
    }

    #[test]
    fn test_range_with_binary_numbers() {
        let result = range().parse("0b1010..0b1110");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(result.into_output().unwrap(), (0b1010, 0b1110));
    }

    #[test]
    fn test_range_with_mixed_numbers() {
        let result = range().parse("10..0x14");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(result.into_output().unwrap(), (10, 0x14));
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
            EnumerationDefinition {
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
            EnumerationDefinition {
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
            EnumerationDefinition {
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
    fn test_attribute_discriminated_by() {
        let result = attribute().parse("discriminated_by = discrimnatorField");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            Attribute::DiscriminatedBy {
                field: Identifier::new("discrimnatorField")
            }
        );
    }

    #[test]
    fn test_attribute_invalid_syntax() {
        let result = attribute().parse("myAttribute myValue");
        assert!(result.has_errors());
        assert!(!result.has_output());
    }

    #[test]
    fn test_attribute_without_spaces() {
        let result = attribute().parse("discriminated_by=discrimnatorField");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            Attribute::DiscriminatedBy {
                field: Identifier::new("discrimnatorField")
            }
        );
    }

    #[test]
    fn test_attribute_tail() {
        let result = attribute_tail().parse(", bits = 10");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            Attribute::BitsSize { size: 10 }
        );
    }

    #[test]
    fn test_attributes() {
        let input = "[discriminated_by = discriminatorField, bits = 10]";
        let result = attributes().parse(input);

        println!("{:?}", result);
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            vec![
                Attribute::DiscriminatedBy {
                    field: Identifier::new("discriminatorField")
                },
                Attribute::BitsSize { size: 10 },
            ]
        );
    }

    #[test]
    fn test_structure_field() {
        let result = structure_field().parse("myField: int32;");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            StructureField {
                attributes: vec![],
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
                attributes: vec![],
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
                attributes: vec![],
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
                attributes: vec![],
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
            StructureDefinition {
                name: Identifier::new("MyStruct"),
                fields: vec![
                    StructureField {
                        attributes: vec![],
                        name: Identifier::new("myField"),
                        r#type: TypeIdentifier::Integer32,
                    },
                    StructureField {
                        attributes: vec![],
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
            StructureDefinition {
                name: Identifier::new("MyStruct"),
                fields: vec![
                    StructureField {
                        attributes: vec![],
                        name: Identifier::new("myField"),
                        r#type: TypeIdentifier::Integer32,
                    },
                    StructureField {
                        attributes: vec![],
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
    fn test_structure_with_invalid_field() {
        let result = structure_definition().parse("struct Xyz {\n\n myField: int32 };");
        assert!(result.has_errors());
        assert!(!result.has_output());
    }

    #[test]
    fn test_union_field_single_value() {
        let result = union_field_single_value().parse("1 => myField: int32;");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            UnionField::SingleValue {
                name: Identifier::new("myField"),
                r#type: TypeIdentifier::Integer32,
                discriminator: 1,
            }
        );
    }

    #[test]
    fn test_union_field_single_value_with_user_defined_type() {
        let result = union_field_single_value().parse("2 => myField: MyCustomType;");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            UnionField::SingleValue {
                name: Identifier::new("myField"),
                r#type: TypeIdentifier::UserDefined(Identifier::new("MyCustomType")),
                discriminator: 2,
            }
        );
    }

    #[test]
    fn test_union_field_single_value_with_static_array() {
        let result = union_field_single_value().parse("3 => myField: int32[10];");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            UnionField::SingleValue {
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
    fn test_union_field_single_value_with_dynamic_array() {
        let result = union_field_single_value().parse("4 => myField: uint64[];");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            UnionField::SingleValue {
                name: Identifier::new("myField"),
                r#type: TypeIdentifier::DynamicArray {
                    r#type: Box::new(TypeIdentifier::UnsignedInteger64),
                },
                discriminator: 4,
            }
        );
    }

    #[test]
    fn test_union_field_range_of_values() {
        let result = union_field_range_of_values().parse("1..3 => myField: int32;");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            UnionField::RangeOfValues {
                name: Identifier::new("myField"),
                r#type: TypeIdentifier::Integer32,
                start_discriminator: 1,
                end_discriminator: 3,
            }
        );
    }

    #[test]
    fn test_union_field() {
        let result = union_field().parse("5 => myField: int32;");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            UnionField::SingleValue {
                name: Identifier::new("myField"),
                r#type: TypeIdentifier::Integer32,
                discriminator: 5,
            }
        );

        let result = union_field().parse("6..8 => myArray: uint64[];");
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            UnionField::RangeOfValues {
                name: Identifier::new("myArray"),
                r#type: TypeIdentifier::DynamicArray {
                    r#type: Box::new(TypeIdentifier::UnsignedInteger64),
                },
                start_discriminator: 6,
                end_discriminator: 8,
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
            UnionDefinition {
                name: Identifier::new("MyUnion"),
                fields: vec![
                    UnionField::SingleValue {
                        name: Identifier::new("myField"),
                        r#type: TypeIdentifier::Integer32,
                        discriminator: 1,
                    },
                    UnionField::SingleValue {
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
            UnionDefinition {
                name: Identifier::new("MyUnion"),
                fields: vec![
                    UnionField::SingleValue {
                        name: Identifier::new("myField"),
                        r#type: TypeIdentifier::Integer32,
                        discriminator: 1,
                    },
                    UnionField::SingleValue {
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
            Definition::Enumeration(EnumerationDefinition {
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
            Definition::Structure(StructureDefinition {
                name: Identifier::new("MyStruct"),
                fields: vec![
                    StructureField {
                        attributes: vec![],
                        name: Identifier::new("myField"),
                        r#type: TypeIdentifier::Integer32,
                    },
                    StructureField {
                        attributes: vec![],
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
            Definition::Union(UnionDefinition {
                name: Identifier::new("MyUnion"),
                fields: vec![
                    UnionField::SingleValue {
                        name: Identifier::new("myField"),
                        r#type: TypeIdentifier::Integer32,
                        discriminator: 1,
                    },
                    UnionField::SingleValue {
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
            Definition::Type(TypeDefinition {
                new_type: Identifier::new("MyType"),
                r#type: TypeIdentifier::Integer32,
            })
        );
    }

    #[test]
    fn test_comment_starting_after_space() {
        let input = "# This is a comment\n";
        let result = comment().parse(input);
        assert!(!result.has_errors() && result.has_output());
    }

    #[test]
    fn test_comment_starting_after_indent() {
        let input = "    # This is a comment with leading spaces\n";
        let result = comment().parse(input);
        assert!(!result.has_errors() && result.has_output());
    }

    #[test]
    fn test_comment_without_space() {
        let input = "#This is a comment without leading space";
        let result = comment().parse(input);
        assert!(!result.has_errors() && result.has_output());
    }

    #[test]
    fn test_protocol() {
        let input = r#"
using MyType = int32[10];
# full line comment does not break things
enum MyEnum {
    myField = 42;
    myRange = 10..20;
};
        # some strange formatted comment also works
struct MyStruct {
    myField: int32;
    myArray: uint64[];
    [bits = 5, bytes = 10, discriminated_by = myType]
    myType: MyType;
};
#and without space it also does work
union MyUnion {
    1 => myField: int32;
    2 => myArray: uint64[];
};
"#;

        let result = protocol().parse(input);
        for error in result.errors() {
            eprintln!("Error: {}", error);
        }
        assert!(!result.has_errors() && result.has_output());
        assert_eq!(
            result.into_output().unwrap(),
            Protocol {
                definitions: vec![
                    Definition::Type(TypeDefinition {
                        new_type: Identifier::new("MyType"),
                        r#type: TypeIdentifier::StaticArray {
                            r#type: Box::new(TypeIdentifier::Integer32),
                            size: 10,
                        },
                    }),
                    Definition::Enumeration(EnumerationDefinition {
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
                    Definition::Structure(StructureDefinition {
                        name: Identifier::new("MyStruct"),
                        fields: vec![
                            StructureField {
                                attributes: vec![],
                                name: Identifier::new("myField"),
                                r#type: TypeIdentifier::Integer32,
                            },
                            StructureField {
                                attributes: vec![],
                                name: Identifier::new("myArray"),
                                r#type: TypeIdentifier::DynamicArray {
                                    r#type: Box::new(TypeIdentifier::UnsignedInteger64),
                                },
                            },
                            StructureField {
                                attributes: vec![
                                    Attribute::BitsSize { size: 5 },
                                    Attribute::BytesSize { size: 10 },
                                    Attribute::DiscriminatedBy {
                                        field: Identifier::new("myType"),
                                    }
                                ],
                                name: Identifier::new("myType"),
                                r#type: TypeIdentifier::UserDefined(Identifier::new("MyType")),
                            }
                        ],
                    }),
                    Definition::Union(UnionDefinition {
                        name: Identifier::new("MyUnion"),
                        fields: vec![
                            UnionField::SingleValue {
                                name: Identifier::new("myField"),
                                r#type: TypeIdentifier::Integer32,
                                discriminator: 1,
                            },
                            UnionField::SingleValue {
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
