pub mod bitenum;
pub mod bitstruct;
pub mod ident;
pub mod numeric;
pub mod token;

// use chumsky::prelude::*;

// use crate::meklang2::ErrType;
// use crate::meklang2::ast::*;
// use crate::meklang2::parser::numeric::number;

// // operators
// static ASTERISK: &str = "*";
// static DOUBLE_DOT: &str = "..";

// // keywords
// static MOST_SIGNIFICANT_BIT_IS_BIT_0: &str = "msb0";
// static LEAST_SIGNIFICANT_BIT_IS_BIT_0: &str = "lsb0";

// pub(crate) fn identifier<'src>() -> impl Parser<'src, &'src str, Identifier, ErrType<'src>> {
//     text::ident()
//         .map(|name: &str| Identifier::new(name))
//         .labelled("identifier")
// }

// fn bit_order<'src>() -> impl Parser<'src, &'src str, BitOrder, ErrType<'src>> {
//     choice((
//         just(MOST_SIGNIFICANT_BIT_IS_BIT_0).to(BitOrder::MostSignificantBitIsBit0),
//         just(LEAST_SIGNIFICANT_BIT_IS_BIT_0).to(BitOrder::LeastSignificantBitIsBit0),
//     ))
//     .labelled("bit order")
// }

// fn bit_range<'src>() -> impl Parser<'src, &'src str, BitRange, ErrType<'src>> {
//     let single_bit = number().padded();

//     let closed_range = number()
//         .then_ignore(just(DOUBLE_DOT).padded())
//         .then(number());

//     let unclosed_range = number()
//         .then_ignore(just(DOUBLE_DOT).padded())
//         .then_ignore(just(ASTERISK));

//     choice((
//         unclosed_range.map(|from| BitRange::UnclosedRange { from }),
//         closed_range.map(|(from, to)| BitRange::ClosedRange { from, to }),
//         single_bit.map(|value| BitRange::SingleBit { value }),
//     ))
//     .labelled("bit range")
// }

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // #[test]
    // fn test_identifier() {
    //     let result = identifier().parse("blah");
    //     assert_eq!(result.into_output().unwrap(), Identifier::new("blah"));
    // }

    // #[rstest]
    // #[case(MOST_SIGNIFICANT_BIT_IS_BIT_0, BitOrder::MostSignificantBitIsBit0)]
    // #[case(LEAST_SIGNIFICANT_BIT_IS_BIT_0, BitOrder::LeastSignificantBitIsBit0)]
    // fn test_bit_order(#[case] input: &str, #[case] expected: BitOrder) {
    //     let result = bit_order().parse(input);
    //     assert_eq!(result.into_output().unwrap(), expected);
    // }

    // #[rstest]
    // #[case("15", BitRange::SingleBit { value: 15 })]
    // #[case("16..32", BitRange::ClosedRange { from: 16, to: 32 })]
    // #[case("32..*", BitRange::UnclosedRange { from: 32 })]
    // fn test_bit_range(#[case] input: &str, #[case] expected: BitRange) {
    //     let result = bit_range().parse(input);
    //     assert_eq!(result.into_output().unwrap(), expected);
    // }
}
