use chumsky::prelude::*;

use crate::meklang2::ErrType;
use crate::meklang2::ast::*;

pub(crate) fn decimal_number<'src>() -> impl Parser<'src, &'src str, usize, ErrType<'src>> {
    text::digits(10)
        .at_least(1)
        .collect::<String>()
        .map(|s| s.parse::<usize>().unwrap())
        .labelled("decimal number")
}

pub(crate) fn hexadecimal_number<'src>() -> impl Parser<'src, &'src str, usize, ErrType<'src>> {
    just("0x")
        .ignore_then(text::digits(16).at_least(1).collect::<String>())
        .map(|s: String| usize::from_str_radix(&s, 16).unwrap())
        .labelled("hexadecimal number")
}

pub(crate) fn binary_number<'src>() -> impl Parser<'src, &'src str, usize, ErrType<'src>> {
    just("0b")
        .ignore_then(text::digits(2).at_least(1).collect::<String>())
        .map(|s: String| usize::from_str_radix(&s, 2).unwrap())
        .labelled("binary number")
}

pub(crate) fn number<'src>() -> impl Parser<'src, &'src str, usize, ErrType<'src>> {
    choice((binary_number(), hexadecimal_number(), decimal_number()))
}

fn decimal_numeric_literal<'src>() -> impl Parser<'src, &'src str, NumericLiteral, ErrType<'src>> {
    text::digits(10)
        .at_least(1)
        .collect::<String>()
        .map(|s| NumericLiteral {
            value: s.parse::<u128>().unwrap(),
            radix: Radix::Decimal,
        })
        .labelled("decimal number")
}

fn hexadecimal_numeric_literal<'src>() -> impl Parser<'src, &'src str, NumericLiteral, ErrType<'src>>
{
    just("0x")
        .ignore_then(text::digits(16).at_least(1).collect::<String>())
        .map(|s| NumericLiteral {
            value: u128::from_str_radix(&s, 16).unwrap(),
            radix: Radix::Hexadecimal,
        })
        .labelled("hexadecimal number")
}

fn binary_numeric_literal<'src>() -> impl Parser<'src, &'src str, NumericLiteral, ErrType<'src>> {
    just("0b")
        .ignore_then(text::digits(2).at_least(1).collect::<String>())
        .map(|s| NumericLiteral {
            value: u128::from_str_radix(&s, 2).unwrap(),
            radix: Radix::Binary,
        })
        .labelled("binary number")
}

pub(crate) fn numeric_literal<'src>() -> impl Parser<'src, &'src str, NumericLiteral, ErrType<'src>>
{
    choice((
        binary_numeric_literal(),
        hexadecimal_numeric_literal(),
        decimal_numeric_literal(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::zero("0", 0)]
    #[case::normal("15123", 15123)]
    fn test_decimal_number(#[case] input: &str, #[case] expected: usize) {
        let result = decimal_number().parse(input);
        assert_eq!(result.into_output().unwrap(), expected);
    }

    #[rstest]
    #[case::zero("0x0", 0x0)]
    #[case::lowercase("0xfeeef", 0xFEEEF)]
    #[case::uppercase("0xFEEEF", 0xFEEEF)]
    #[case::mixed_case("0xFeeef", 0xFEEEF)]
    fn test_hexadecimal_number(#[case] input: &str, #[case] expected: usize) {
        let result = hexadecimal_number().parse(input);
        assert_eq!(result.into_output().unwrap(), expected);
    }

    #[rstest]
    #[case::zero("0b0", 0b0)]
    #[case::normal("0b10101", 0b10101)]
    fn test_binary_number(#[case] input: &str, #[case] expected: usize) {
        let result = binary_number().parse(input);
        assert_eq!(result.into_output().unwrap(), expected);
    }

    #[rstest]
    #[case("123", 123)]
    #[case("0x1A", 0x1A)]
    #[case("0b101", 0b101)]
    fn test_number(#[case] input: &str, #[case] expected: usize) {
        let result = number().parse(input);
        assert_eq!(result.into_output().unwrap(), expected);
    }

    #[rstest]
    #[case::zero("0", NumericLiteral{ value: 0, radix: Radix::Decimal })]
    #[case::normal("15123", NumericLiteral{ value: 15123, radix: Radix::Decimal })]
    fn test_decimal_numeric_literal(#[case] input: &str, #[case] expected: NumericLiteral) {
        let result = decimal_numeric_literal().parse(input);
        assert_eq!(result.into_output().unwrap(), expected);
    }

    #[rstest]
    #[case::zero("0x0", NumericLiteral { value: 0x0, radix: Radix::Hexadecimal } )]
    #[case::lowercase("0xfeeef", NumericLiteral { value: 0xFEEEF, radix: Radix::Hexadecimal } )]
    #[case::uppercase("0xFEEEF", NumericLiteral { value: 0xFEEEF, radix: Radix::Hexadecimal } )]
    #[case::mixed_case("0xFeeef", NumericLiteral { value: 0xFEEEF, radix: Radix::Hexadecimal } )]
    fn test_hexadecimal_numeric_literal(#[case] input: &str, #[case] expected: NumericLiteral) {
        let result = hexadecimal_numeric_literal().parse(input);
        assert_eq!(result.into_output().unwrap(), expected);
    }

    #[rstest]
    #[case::zero("0b0", NumericLiteral { value: 0b0, radix: Radix::Binary } )]
    #[case::normal("0b10101", NumericLiteral { value: 0b10101, radix: Radix::Binary } )]
    fn test_binary_numeric_literal(#[case] input: &str, #[case] expected: NumericLiteral) {
        let result = binary_numeric_literal().parse(input);
        assert_eq!(result.into_output().unwrap(), expected);
    }

    #[rstest]
    #[case("123", NumericLiteral { value: 123, radix: Radix::Decimal })]
    #[case("0x1A", NumericLiteral { value: 0x1A, radix: Radix::Hexadecimal })]
    #[case("0b101", NumericLiteral { value: 0b101, radix: Radix::Binary })]
    fn test_numeric_literal(#[case] input: &str, #[case] expected: NumericLiteral) {
        let result = numeric_literal().parse(input);
        assert_eq!(result.into_output().unwrap(), expected);
    }
}
