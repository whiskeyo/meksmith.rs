use chumsky::prelude::*;

use crate::meklang::ast::Identifier;
use crate::meklang::parser::ErrType;

pub(crate) fn identifier<'src>() -> impl Parser<'src, &'src str, Identifier, ErrType<'src>> {
    text::ident()
        .map(|name: &str| Identifier::new(name))
        .labelled("identifier")
}

pub(crate) fn number<'src>() -> impl Parser<'src, &'src str, usize, ErrType<'src>> {
    let dec = text::digits(10)
        .at_least(1)
        .collect::<String>()
        .map(|s| s.parse::<usize>().unwrap())
        .labelled("decimal number");

    let hex = just("0x")
        .ignore_then(text::digits(16).at_least(1).collect::<String>())
        .map(|s: String| usize::from_str_radix(&s, 16).unwrap())
        .labelled("hexadecimal number");

    let bin = just("0b")
        .ignore_then(text::digits(2).at_least(1).collect::<String>())
        .map(|s: String| usize::from_str_radix(&s, 2).unwrap())
        .labelled("binary number");

    choice((bin, hex, dec))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_identifier() {
        let result = identifier().parse("blah");
        assert_eq!(result.into_output().unwrap(), Identifier::new("blah"));
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
