use chumsky::prelude::*;

use crate::meklang2::ErrType;
use crate::meklang2::ast::{BitEnum, BitEnumField};
use crate::meklang2::parser_depr::ident::identifier;
use crate::meklang2::parser_depr::numeric::number;
use crate::meklang2::parser_depr::token::{
    BIT_ENUM, COMMA, DOUBLE_DOT, EQUALS, LBRACE, LPAREN, RBRACE, RPAREN,
};

pub(crate) fn bit_enum_field<'src>() -> impl Parser<'src, &'src str, BitEnumField, ErrType<'src>> {
    let single = identifier()
        .then_ignore(just(EQUALS).padded())
        .then(number())
        .map(|(name, value)| BitEnumField::SingleValue { name, value });

    let range = identifier()
        .then_ignore(just(EQUALS).padded())
        .then(number())
        .then_ignore(just(DOUBLE_DOT).padded())
        .then(number())
        .map(|((name, from), to)| BitEnumField::RangeOfValues { name, from, to });

    choice((range, single))
}

pub(crate) fn bit_enum<'src>() -> impl Parser<'src, &'src str, BitEnum, ErrType<'src>> {
    let header = just(BIT_ENUM)
        .padded()
        .ignore_then(
            number()
                .padded()
                .delimited_by(just(LPAREN).padded(), just(RPAREN).padded()),
        )
        .then(identifier().padded());

    let fields = bit_enum_field()
        .separated_by(just(COMMA).padded())
        .collect()
        .delimited_by(just(LBRACE).padded(), just(RBRACE).padded());

    header
        .then(fields)
        .map(|((size, name), fields)| BitEnum { size, name, fields })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    use crate::meklang2::ast::Identifier;

    #[rstest]
    #[case::single_value("single = 55", BitEnumField::SingleValue { name: Identifier::new("single"), value: 55})]
    #[case::range_of_values("range = 0..5", BitEnumField::RangeOfValues { name: Identifier::new("range"), from: 0, to: 5})]
    fn test_bit_enum_field(#[case] input: &str, #[case] expected: BitEnumField) {
        let result = bit_enum_field().parse(input);
        assert_eq!(result.into_output().unwrap(), expected);
    }

    #[test]
    fn test_bit_enum() {
        let input = "bitenum(3) Enum { a = 0, b = 1, c = 2..5 }";
        let expected = BitEnum {
            name: Identifier::new("Enum"),
            size: 3,
            fields: vec![
                BitEnumField::SingleValue {
                    name: Identifier::new("a"),
                    value: 0,
                },
                BitEnumField::SingleValue {
                    name: Identifier::new("b"),
                    value: 1,
                },
                BitEnumField::RangeOfValues {
                    name: Identifier::new("c"),
                    from: 2,
                    to: 5,
                },
            ],
        };

        let result = bit_enum().parse(input);
        assert_eq!(result.into_output().unwrap(), expected);
    }
}
