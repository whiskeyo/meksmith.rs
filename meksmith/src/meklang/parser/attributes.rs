use chumsky::prelude::*;

use crate::meklang::ast::Attribute;
use crate::meklang::parser::ErrType;
use crate::meklang::parser::base::{identifier, number};
use crate::meklang::parser::tokens::{COMMA, EQUALS, LBRACKET, RBRACKET};

pub(crate) const ATTR_BITS: &str = "bits";
pub(crate) const ATTR_BYTES: &str = "bytes";
pub(crate) const ATTR_DISCRIMINATOR: &str = "discriminator";
pub(crate) const ATTR_STATIC_ARRAY: &str = "static_array";
pub(crate) const ATTR_DYNAMIC_ARRAY: &str = "dynamic_array";

pub(crate) fn attribute<'src>() -> impl Parser<'src, &'src str, Attribute, ErrType<'src>> {
    let bits = just(ATTR_BITS)
        .ignore_then(just(EQUALS).padded())
        .ignore_then(number())
        .map(Attribute::Bits);

    let bytes = just(ATTR_BYTES)
        .ignore_then(just(EQUALS).padded())
        .ignore_then(number())
        .map(Attribute::Bytes);

    let discriminator_for = just(ATTR_DISCRIMINATOR)
        .ignore_then(just(EQUALS).padded())
        .ignore_then(identifier())
        .map(Attribute::Discriminator);

    let static_array = just(ATTR_STATIC_ARRAY)
        .ignore_then(just(EQUALS).padded())
        .ignore_then(number())
        .map(Attribute::StaticArray);

    let dynamic_array = just(ATTR_DYNAMIC_ARRAY)
        .ignore_then(just(EQUALS).padded())
        .ignore_then(identifier())
        .map(Attribute::DynamicArray);

    choice((bits, bytes, discriminator_for, static_array, dynamic_array))
}

pub(crate) fn attributes<'src>() -> impl Parser<'src, &'src str, Vec<Attribute>, ErrType<'src>> {
    attribute()
        .separated_by(just(COMMA).padded())
        .collect()
        .delimited_by(just(LBRACKET).padded(), just(RBRACKET).padded())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    use crate::meklang::ast::Identifier;

    #[rstest]
    #[case("bits = 8", Attribute::Bits(8))]
    #[case("bytes = 4", Attribute::Bytes(4))]
    #[case("discriminator = x", Attribute::Discriminator(Identifier::new("x")))]
    #[case("static_array = 10", Attribute::StaticArray(10))]
    #[case("dynamic_array = y", Attribute::DynamicArray(Identifier::new("y")))]
    fn test_attribute(#[case] input: &str, #[case] expected: Attribute) {
        let result = attribute().parse(input);
        assert_eq!(result.into_output().unwrap(), expected);
    }

    #[rstest]
    #[case("[bits = 8, bytes = 4, discriminator = foo, static_array = 10]", vec![
        Attribute::Bits(8),
        Attribute::Bytes(4),
        Attribute::Discriminator(Identifier::new("foo")),
        Attribute::StaticArray(10),
    ])]
    #[case("[]", vec![])]
    fn test_attributes(#[case] input: &str, #[case] expected: Vec<Attribute>) {
        let result = attributes().parse(input);
        assert_eq!(result.into_output().unwrap(), expected);
    }
}
