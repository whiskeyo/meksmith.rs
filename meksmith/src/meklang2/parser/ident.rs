use chumsky::prelude::*;

use crate::meklang2::ErrType;
use crate::meklang2::ast::Identifier;

pub(crate) fn identifier<'src>() -> impl Parser<'src, &'src str, Identifier, ErrType<'src>> {
    text::ident()
        .map(|name: &str| Identifier::new(name))
        .labelled("identifier")
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
}
