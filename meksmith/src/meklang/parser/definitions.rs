use chumsky::prelude::*;

use crate::meklang::ast::{Definition, Enumeration, Structure, Union};
use crate::meklang::parser::ErrType;
use crate::meklang::parser::enumerations::enumeration;
use crate::meklang::parser::structures::structure;
use crate::meklang::parser::unions::union;

pub(crate) fn definition<'src>() -> impl Parser<'src, &'src str, Definition, ErrType<'src>> {
    choice((
        enumeration().map(Definition::Enumeration),
        structure().map(Definition::Structure),
        union().map(Definition::Union),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_definition_with_enumeration() {
        let input = "enum MyEnum { x = 0 }";
        let result = definition().parse(input).into_result().unwrap();
        assert!(matches!(result, Definition::Enumeration(_)));
    }

    #[test]
    fn test_definition_with_structure() {
        let input = "struct MyStruct { x: typeeeee }";
        let result = definition().parse(input).into_result().unwrap();
        assert!(matches!(result, Definition::Structure(_)));
    }

    #[test]
    fn test_definition_with_union() {
        let input = "union MyUnion { 0 => x: type }";
        let result = definition().parse(input).into_result().unwrap();
        assert!(matches!(result, Definition::Union(_)));
    }
}
