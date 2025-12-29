use chumsky::prelude::*;

use crate::meklang2::ErrType;
use crate::meklang2::ast::{Identifier, Reference};
use crate::meklang2::parser::token::DOT;

pub(crate) fn identifier<'src>() -> impl Parser<'src, &'src str, Identifier, ErrType<'src>> {
    text::ident()
        .map(|name: &str| Identifier::new(name))
        .labelled("identifier")
}

pub(crate) fn reference<'src>() -> impl Parser<'src, &'src str, Reference, ErrType<'src>> {
    identifier()
        .separated_by(just(DOT).padded())
        .at_least(1)
        .collect::<Vec<Identifier>>()
        .map(|path| Reference { path })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_identifier() {
        let result = identifier().parse("blah");
        assert_eq!(result.into_output().unwrap(), Identifier::new("blah"));
    }

    #[rstest]
    #[case::single("x", Reference { path: vec![Identifier::new("x")] })]
    #[case::double("x.y", Reference { path: vec![Identifier::new("x"), Identifier::new("y")] })]
    #[case::triple("x.y.z", Reference { path: vec![Identifier::new("x"), Identifier::new("y"), Identifier::new("z")] })]
    fn test_reference(#[case] input: &str, #[case] expected: Reference) {
        let result = reference().parse(input);
        assert_eq!(result.into_output().unwrap(), expected);
    }
}
