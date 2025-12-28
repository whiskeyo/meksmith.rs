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
}
