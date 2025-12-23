use chumsky::prelude::*;

use crate::meklang::ast::{Structure, StructureField};
use crate::meklang::parser::ErrType;
use crate::meklang::parser::attributes::attributes;
use crate::meklang::parser::base::identifier;
use crate::meklang::parser::tokens::{COLON, COMMA, LBRACE, RBRACE};
use crate::meklang::parser::types::any_type;

pub(crate) const STRUCTURE: &str = "struct";

pub(crate) fn structure_field<'src>() -> impl Parser<'src, &'src str, StructureField, ErrType<'src>>
{
    attributes()
        .or_not()
        .then(
            identifier()
                .padded()
                .then_ignore(just(COLON).padded())
                .then(any_type().padded()),
        )
        .map(|(attrs, (name, typ))| StructureField {
            name,
            typ,
            attributes: attrs.map_or(vec![], |attrs| attrs),
        })
}

pub(crate) fn structure<'src>() -> impl Parser<'src, &'src str, Structure, ErrType<'src>> {
    let fields = structure_field()
        .separated_by(just(COMMA).padded())
        .collect()
        .delimited_by(just(LBRACE).padded(), just(RBRACE).padded());

    just(STRUCTURE)
        .padded()
        .ignore_then(identifier().padded())
        .then(fields)
        .map(|(name, fields)| Structure { name, fields })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::meklang::ast::{Attribute, BuiltinType, Identifier, Type};

    #[test]
    fn test_structure_field() {
        let input = "name: my_type_t";
        let expected = StructureField {
            name: Identifier::new("name"),
            typ: Type::UserDefined(Identifier::new("my_type_t")),
            attributes: vec![],
        };
        let result = structure_field().parse(input);
        assert_eq!(result.into_result().unwrap(), expected);
    }

    #[test]
    fn test_structure_field_with_attributes() {
        let input = "[static_array = 10] name: my_type_t";
        let expected = StructureField {
            name: Identifier::new("name"),
            typ: Type::UserDefined(Identifier::new("my_type_t")),
            attributes: vec![Attribute::StaticArray(10)],
        };
        let result = structure_field().parse(input);
        assert_eq!(result.into_result().unwrap(), expected);
    }

    #[test]
    fn test_structure() {
        let input = "struct MyStruct { name: my_type_t }";
        let expected = Structure {
            name: Identifier::new("MyStruct"),
            fields: vec![StructureField {
                name: Identifier::new("name"),
                typ: Type::UserDefined(Identifier::new("my_type_t")),
                attributes: vec![],
            }],
        };
        let result = structure().parse(input);
        assert_eq!(result.into_result().unwrap(), expected);
    }

    #[test]
    fn test_structure_with_many_fields() {
        let input = "struct MyStruct { name: my_type_t, age: u32, [bytes = 16, static_array = 3] hobby: char }";
        let expected = Structure {
            name: Identifier::new("MyStruct"),
            fields: vec![
                StructureField {
                    name: Identifier::new("name"),
                    typ: Type::UserDefined(Identifier::new("my_type_t")),
                    attributes: vec![],
                },
                StructureField {
                    name: Identifier::new("age"),
                    typ: Type::Builtin(BuiltinType::UnsignedInteger32),
                    attributes: vec![],
                },
                StructureField {
                    name: Identifier::new("hobby"),
                    typ: Type::UserDefined(Identifier::new("char")),
                    attributes: vec![Attribute::Bytes(16), Attribute::StaticArray(3)],
                },
            ],
        };
        let result = structure().parse(input);
        assert_eq!(result.into_result().unwrap(), expected);
    }
}
