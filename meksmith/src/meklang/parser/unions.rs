use chumsky::prelude::*;

use crate::meklang::ast::{Union, UnionField};
use crate::meklang::parser::ErrType;
use crate::meklang::parser::attributes::attributes;
use crate::meklang::parser::base::{identifier, number};
use crate::meklang::parser::tokens::{COLON, COMMA, DOUBLE_DOT, LBRACE, MAPS_TO, RBRACE};
use crate::meklang::parser::types::any_type;

pub(crate) const UNION: &str = "union";

pub(crate) fn union_field<'src>() -> impl Parser<'src, &'src str, UnionField, ErrType<'src>> {
    let single_value = attributes()
        .or_not()
        .then(
            number()
                .padded()
                .then_ignore(just(MAPS_TO).padded())
                .then(identifier().padded())
                .then_ignore(just(COLON).padded())
                .then(any_type().padded()),
        )
        .map(
            |(attrs, ((discriminator, name), typ))| UnionField::SingleValue {
                name,
                typ,
                discriminator,
                attributes: attrs.map_or(vec![], |attrs| attrs),
            },
        );

    let range_of_values = attributes()
        .or_not()
        .then(
            number()
                .padded()
                .then_ignore(just(DOUBLE_DOT).padded())
                .then(number().padded())
                .then_ignore(just(MAPS_TO).padded())
                .then(identifier().padded())
                .then_ignore(just(COLON).padded())
                .then(any_type().padded()),
        )
        .map(
            |(attrs, (((discriminator_from, discriminator_to), name), typ))| {
                UnionField::RangeOfValues {
                    name,
                    typ,
                    discriminator_from,
                    discriminator_to,
                    attributes: attrs.map_or(vec![], |attrs| attrs),
                }
            },
        );

    choice((range_of_values, single_value))
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
    fn test_union_field_single_value() {
        let input = "15 => field_name: FieldType";
        let expected = UnionField::SingleValue {
            name: Identifier::new("field_name"),
            typ: Type::UserDefined(Identifier::new("FieldType")),
            discriminator: 15,
            attributes: vec![],
        };

        let result = union_field().parse(input);
        assert_eq!(result.into_result().unwrap(), expected);
    }

    #[test]
    fn test_union_field_single_value_with_attributes() {
        let input = "[bytes=6] 15 => field_name: FieldType";
        let expected = UnionField::SingleValue {
            name: Identifier::new("field_name"),
            typ: Type::UserDefined(Identifier::new("FieldType")),
            discriminator: 15,
            attributes: vec![Attribute::Bytes(6)],
        };

        let result = union_field().parse(input);
        assert_eq!(result.into_result().unwrap(), expected);
    }

    #[test]
    fn test_union_field_range_of_values() {
        let input = "15..20 => field_name: FieldType";
        let expected = UnionField::RangeOfValues {
            name: Identifier::new("field_name"),
            typ: Type::UserDefined(Identifier::new("FieldType")),
            discriminator_from: 15,
            discriminator_to: 20,
            attributes: vec![],
        };

        let result = union_field().parse(input);
        assert_eq!(result.into_result().unwrap(), expected);
    }

    #[test]
    fn test_union_field_range_of_values_with_attributes() {
        let input = "[bytes=6] 15..16 => field_name: FieldType";
        let expected = UnionField::RangeOfValues {
            name: Identifier::new("field_name"),
            typ: Type::UserDefined(Identifier::new("FieldType")),
            discriminator_from: 15,
            discriminator_to: 16,
            attributes: vec![Attribute::Bytes(6)],
        };

        let result = union_field().parse(input);
        assert_eq!(result.into_result().unwrap(), expected);
    }

    #[test]
    fn test_union() {
        let input = "union MyUnion { 0 => x: X, [bits=3] 1..2 => y: Y, 3 => z: Z }";
        let expected = Union {
            name: Identifier::new("MyUnion"),
            fields: vec![
                UnionField::SingleValue {
                    name: Identifier::new("x"),
                    typ: Type::UserDefined(Identifier::new("X")),
                    discriminator: 0,
                    attributes: vec![],
                },
                UnionField::RangeOfValues {
                    name: Identifier::new("y"),
                    typ: Type::UserDefined(Identifier::new("Y")),
                    discriminator_from: 1,
                    discriminator_to: 2,
                    attributes: vec![Attribute::Bits(3)],
                },
                UnionField::SingleValue {
                    name: Identifier::new("z"),
                    typ: Type::UserDefined(Identifier::new("Z")),
                    discriminator: 3,
                    attributes: vec![],
                },
            ],
        };

        let result = union().parse(input);
        assert_eq!(result.into_result().unwrap(), expected);
    }
}
