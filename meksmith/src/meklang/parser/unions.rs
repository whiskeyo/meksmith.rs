use chumsky::prelude::*;

use crate::meklang::ast::{Union, UnionField};
use crate::meklang::parser::ErrType;
use crate::meklang::parser::attributes::attributes;
use crate::meklang::parser::base::{identifier, number};
use crate::meklang::parser::tokens::{COLON, COMMA, LBRACE, MAPS_TO, RBRACE};
use crate::meklang::parser::types::any_type;

pub(crate) const UNION: &str = "union";

pub(crate) fn union_field<'src>() -> impl Parser<'src, &'src str, UnionField, ErrType<'src>> {
    attributes()
        .or_not()
        .then(
            number()
                .padded()
                .then_ignore(just(MAPS_TO).padded())
                .then(identifier().padded())
                .then_ignore(just(COLON).padded())
                .then(any_type().padded()),
        )
        .map(|(attrs, ((discriminator, name), typ))| UnionField {
            name,
            typ,
            discriminator,
            attributes: attrs.map_or(vec![], |attrs| attrs),
        })
}

pub(crate) fn union<'src>() -> impl Parser<'src, &'src str, Union, ErrType<'src>> {
    let fields = union_field()
        .separated_by(just(COMMA).padded())
        .collect()
        .delimited_by(just(LBRACE).padded(), just(RBRACE).padded());

    just(UNION)
        .padded()
        .ignore_then(identifier().padded())
        .then(fields)
        .map(|(name, fields)| Union { name, fields })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::meklang::ast::{Attribute, Identifier, Type};

    #[test]
    fn test_union_field() {
        let input = "15 => field_name: FieldType";
        let expected = UnionField {
            name: Identifier::new("field_name"),
            typ: Type::UserDefined(Identifier::new("FieldType")),
            discriminator: 15,
            attributes: vec![],
        };

        let result = union_field().parse(input);
        assert_eq!(result.into_result().unwrap(), expected);
    }

    #[test]
    fn test_union_field_with_attributes() {
        let input = "[bytes=6] 15 => field_name: FieldType";
        let expected = UnionField {
            name: Identifier::new("field_name"),
            typ: Type::UserDefined(Identifier::new("FieldType")),
            discriminator: 15,
            attributes: vec![Attribute::Bytes(6)],
        };

        let result = union_field().parse(input);
        assert_eq!(result.into_result().unwrap(), expected);
    }

    #[test]
    fn test_union() {
        let input = "union MyUnion { 0 => x: X, [bits=3] 1 => y: Y, 2 => z: Z }";
        let expected = Union {
            name: Identifier::new("MyUnion"),
            fields: vec![
                UnionField {
                    name: Identifier::new("x"),
                    typ: Type::UserDefined(Identifier::new("X")),
                    discriminator: 0,
                    attributes: vec![],
                },
                UnionField {
                    name: Identifier::new("y"),
                    typ: Type::UserDefined(Identifier::new("Y")),
                    discriminator: 1,
                    attributes: vec![Attribute::Bits(3)],
                },
                UnionField {
                    name: Identifier::new("z"),
                    typ: Type::UserDefined(Identifier::new("Z")),
                    discriminator: 2,
                    attributes: vec![],
                },
            ],
        };

        let result = union().parse(input);
        assert_eq!(result.into_result().unwrap(), expected);
    }
}
