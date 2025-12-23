use chumsky::prelude::*;

use crate::meklang::ast::{Enumeration, EnumerationField};
use crate::meklang::parser::ErrType;
use crate::meklang::parser::attributes::attributes;
use crate::meklang::parser::base::{identifier, number};
use crate::meklang::parser::tokens::{COMMA, DOUBLE_DOT, EQUALS, LBRACE, RBRACE};

pub(crate) const ENUMERATION: &str = "enum";

pub(crate) fn enumeration_field<'src>()
-> impl Parser<'src, &'src str, EnumerationField, ErrType<'src>> {
    let single_value = identifier()
        .then_ignore(just(EQUALS).padded())
        .then(number().padded())
        .map(|(name, value)| EnumerationField::SingleValue { name, value });

    let range_of_values = identifier()
        .then_ignore(just(EQUALS).padded())
        .then(number().padded())
        .then_ignore(just(DOUBLE_DOT).padded())
        .then(number().padded())
        .map(|((name, from), to)| EnumerationField::RangeOfValues { name, from, to });

    choice((range_of_values, single_value))
}

pub(crate) fn enumeration<'src>() -> impl Parser<'src, &'src str, Enumeration, ErrType<'src>> {
    let fields = enumeration_field()
        .separated_by(just(COMMA).padded())
        .collect()
        .delimited_by(just(LBRACE).padded(), just(RBRACE).padded());

    attributes()
        .or_not()
        .then(
            just(ENUMERATION)
                .padded()
                .ignore_then(identifier().padded())
                .then(fields),
        )
        .map(|(attrs, (name, fields))| Enumeration {
            name,
            fields,
            attributes: attrs.map_or(vec![], |attrs| attrs),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meklang::ast::{Attribute, Identifier};

    #[test]
    fn test_enumeration_field_single_value() {
        let input = "foo = 42";
        let expected = EnumerationField::SingleValue {
            name: Identifier::new("foo"),
            value: 42,
        };

        let result = enumeration_field().parse(input);
        assert_eq!(result.into_output().unwrap(), expected);
    }

    #[test]
    fn test_enumeration_field_range_of_values() {
        let input = "bar = 10..20";
        let expected = EnumerationField::RangeOfValues {
            name: Identifier::new("bar"),
            from: 10,
            to: 20,
        };

        let result = enumeration_field().parse(input);
        assert_eq!(result.into_output().unwrap(), expected);
    }

    #[test]
    fn test_enumeration() {
        let input = "enum MyEnum { foo = 0, bar = 1, baz = 2..3 }";
        let expected = Enumeration {
            name: Identifier::new("MyEnum"),
            fields: vec![
                EnumerationField::SingleValue {
                    name: Identifier::new("foo"),
                    value: 0,
                },
                EnumerationField::SingleValue {
                    name: Identifier::new("bar"),
                    value: 1,
                },
                EnumerationField::RangeOfValues {
                    name: Identifier::new("baz"),
                    from: 2,
                    to: 3,
                },
            ],
            attributes: vec![],
        };

        let result = enumeration().parse(input);
        assert_eq!(result.into_result().unwrap(), expected);
    }

    #[test]
    fn test_enumeration_with_bits_attribute() {
        let input = "[bits = 2] enum MyEnum { foo = 0, bar = 1, baz = 2..3 }";
        let expected = Enumeration {
            name: Identifier::new("MyEnum"),
            fields: vec![
                EnumerationField::SingleValue {
                    name: Identifier::new("foo"),
                    value: 0,
                },
                EnumerationField::SingleValue {
                    name: Identifier::new("bar"),
                    value: 1,
                },
                EnumerationField::RangeOfValues {
                    name: Identifier::new("baz"),
                    from: 2,
                    to: 3,
                },
            ],
            attributes: vec![Attribute::Bits(2)],
        };

        let result = enumeration().parse(input);
        assert_eq!(result.into_result().unwrap(), expected);
    }
}
