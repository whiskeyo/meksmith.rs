use chumsky::pratt::*;
use chumsky::prelude::*;

use crate::meklang2::ErrType;
use crate::meklang2::ast::Radix;
use crate::meklang2::ast::{Identifier, NumericLiteral, Reference};
use crate::meklang2::parser::ident::identifier;
use crate::meklang2::parser::token::{
    AND, DOT, EQUALS_TO, GREATER_EQUAL, GREATER_THAN, LESS_EQUAL, LESS_THAN, LPAREN, NOT,
    NOT_EQUAL, OR, RPAREN,
};

// ***************************************
// SECTION FOR EXPRESSIONS
// ***************************************

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    NumericLiteral(NumericLiteral),
    Reference(Reference),
    UnaryOperator {
        operator: UnaryOperator,
        expr: Box<Expr>,
    },
    BinaryOperator {
        operator: BinaryOperator,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    Not,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    And,
    Or,

    Equals,
    NotEquals,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
}

// START to be moved to numeric.rs
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

fn numeric_literal<'src>() -> impl Parser<'src, &'src str, NumericLiteral, ErrType<'src>> {
    choice((
        binary_numeric_literal(),
        hexadecimal_numeric_literal(),
        decimal_numeric_literal(),
    ))
}
// END to be moved to numeric.rs

// START to be moved to ident.rs
fn reference<'src>() -> impl Parser<'src, &'src str, Reference, ErrType<'src>> {
    identifier()
        .separated_by(just(DOT).padded())
        .at_least(1)
        .collect::<Vec<Identifier>>()
        .map(|path| Reference { path })
}
// END to be moved to ident.rs

fn expr<'src>() -> impl Parser<'src, &'src str, Expr, ErrType<'src>> {
    recursive(|expr| {
        let atom = choice((
            numeric_literal().map(Expr::NumericLiteral).boxed(),
            reference().map(Expr::Reference).boxed(),
            expr.delimited_by(just(LPAREN).padded(), just(RPAREN).padded())
                .boxed(),
        ));

        let operator = |op| just(op).padded();

        atom.pratt((
            // comparison operators have the highest precedence,
            // chaining is forbidden thus Associativity::None is used
            infix(
                Associativity::None(4),
                operator(EQUALS_TO),
                |lhs, _, rhs, _| Expr::BinaryOperator {
                    operator: BinaryOperator::Equals,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
            ),
            infix(
                Associativity::None(4),
                operator(NOT_EQUAL),
                |lhs, _, rhs, _| Expr::BinaryOperator {
                    operator: BinaryOperator::NotEquals,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
            ),
            infix(
                Associativity::None(4),
                operator(LESS_THAN),
                |lhs, _, rhs, _| Expr::BinaryOperator {
                    operator: BinaryOperator::LessThan,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
            ),
            infix(
                Associativity::None(4),
                operator(LESS_EQUAL),
                |lhs, _, rhs, _| Expr::BinaryOperator {
                    operator: BinaryOperator::LessEqual,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
            ),
            infix(
                Associativity::None(4),
                operator(GREATER_THAN),
                |lhs, _, rhs, _| Expr::BinaryOperator {
                    operator: BinaryOperator::GreaterThan,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
            ),
            infix(
                Associativity::None(4),
                operator(GREATER_EQUAL),
                |lhs, _, rhs, _| Expr::BinaryOperator {
                    operator: BinaryOperator::GreaterEqual,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
            ),
            // negation has smaller precedence, it's prefix operator with right associativity
            prefix(3, operator(NOT), |_, expr, _| Expr::UnaryOperator {
                operator: UnaryOperator::Not,
                expr: Box::new(expr),
            }),
            // and should be done after comparisons/negations, it's infix operator with left associativity
            infix(Associativity::Left(2), operator(AND), |lhs, _, rhs, _| {
                Expr::BinaryOperator {
                    operator: BinaryOperator::And,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }
            }),
            // or comes at the end, similar to and
            infix(Associativity::Left(1), operator(OR), |lhs, _, rhs, _| {
                Expr::BinaryOperator {
                    operator: BinaryOperator::Or,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }
            }),
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

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

    #[rstest]
    #[case::single("x", Reference { path: vec![Identifier::new("x")] })]
    #[case::double("x.y", Reference { path: vec![Identifier::new("x"), Identifier::new("y")] })]
    #[case::triple("x.y.z", Reference { path: vec![Identifier::new("x"), Identifier::new("y"), Identifier::new("z")] })]
    fn test_reference(#[case] input: &str, #[case] expected: Reference) {
        let result = reference().parse(input);
        assert_eq!(result.into_output().unwrap(), expected);
    }

    #[rstest]
    #[case::equals(
        "x.z == 0x1337",
        Expr::BinaryOperator {
            operator: BinaryOperator::Equals,
            lhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("x"), Identifier::new("z")] })),
            rhs: Box::new(Expr::NumericLiteral(NumericLiteral { value: 0x1337, radix: Radix::Hexadecimal }))
        }
    )]
    #[case::not_equals(
        "y != 0b1010",
        Expr::BinaryOperator {
            operator: BinaryOperator::NotEquals,
            lhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("y")] })),
            rhs: Box::new(Expr::NumericLiteral(NumericLiteral { value: 0b1010, radix: Radix::Binary }))
        }
    )]
    #[case::less_than(
        "a < b",
        Expr::BinaryOperator {
            operator: BinaryOperator::LessThan,
            lhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("a")] })),
            rhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("b")] })),
        }
    )]
    #[case::less_equal(
        "a <= b",
        Expr::BinaryOperator {
            operator: BinaryOperator::LessEqual,
            lhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("a")] })),
            rhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("b")] })),
        }
    )]
    #[case::greater_than(
        "a > b",
        Expr::BinaryOperator {
            operator: BinaryOperator::GreaterThan,
            lhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("a")] })),
            rhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("b")] })),
        }
    )]
    #[case::greater_equal(
        "a >= b",
        Expr::BinaryOperator {
            operator: BinaryOperator::GreaterEqual,
            lhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("a")] })),
            rhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("b")] })),
        }
    )]
    #[case::not(
        "not a",
        Expr::UnaryOperator {
            operator: UnaryOperator::Not,
            expr:  Box::new(Expr::Reference(Reference { path: vec![Identifier::new("a")] })),
        }
    )]
    #[case::greater_equal(
        "a and b",
        Expr::BinaryOperator {
            operator: BinaryOperator::And,
            lhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("a")] })),
            rhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("b")] })),
        }
    )]
    #[case::greater_equal(
        "a or b",
        Expr::BinaryOperator {
            operator: BinaryOperator::Or,
            lhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("a")] })),
            rhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("b")] })),
        }
    )]
    #[case::parentheses_on_the_left(
        "(a and b) and c",
        Expr::BinaryOperator {
            operator: BinaryOperator::And,
            lhs: Box::new(
                Expr::BinaryOperator {
                    operator: BinaryOperator::And,
                    lhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("a")] })),
                    rhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("b")] })),
                }
            ),
            rhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("c")] })),
        }
    )]
    #[case::parentheses_on_the_right(
        "a and (b and c)",
        Expr::BinaryOperator {
            operator: BinaryOperator::And,
            lhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("a")] })),
            rhs: Box::new(
                Expr::BinaryOperator {
                    operator: BinaryOperator::And,
                    lhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("b")] })),
                    rhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("c")] })),
                }
            ),
        }
    )]
    #[case::parentheses_nested_on_the_left(
        "(((a and b) and c) or d)",
        Expr::BinaryOperator {
            operator: BinaryOperator::Or,
            lhs: Box::new(
                Expr::BinaryOperator {
                    operator: BinaryOperator::And,
                    lhs: Box::new(Expr::BinaryOperator {
                        operator: BinaryOperator::And,
                        lhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("a")] })),
                        rhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("b")] })),
                    }),
                    rhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("c")] })),
                }
            ),
            rhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("d")] })),
        }
    )]
    #[case::parentheses_nested_on_the_right(
        "(a and (b and (c or d)))",
        Expr::BinaryOperator {
            operator: BinaryOperator::And,
            lhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("a")] })),
            rhs: Box::new(
                Expr::BinaryOperator {
                    operator: BinaryOperator::And,
                    lhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("b")] })),
                    rhs: Box::new(Expr::BinaryOperator {
                        operator: BinaryOperator::Or,
                        lhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("c")] })),
                        rhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("d")] })),
                    }),
                }
            ),
        }
    )]
    #[case::multiple_comparisons(
        "not (a > 10 and a <= 20 or a == 30)",
        Expr::UnaryOperator {
            operator: UnaryOperator::Not,
            expr: Box::new(
                Expr::BinaryOperator {
                    operator: BinaryOperator::Or,
                    lhs: Box::new(
                        Expr::BinaryOperator {
                            operator: BinaryOperator::And,
                            lhs: Box::new(
                                Expr::BinaryOperator {
                                    operator: BinaryOperator::GreaterThan,
                                    lhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("a")] })),
                                    rhs: Box::new(Expr::NumericLiteral(NumericLiteral { value: 10, radix: Radix::Decimal }))
                                }
                            ),
                            rhs: Box::new(
                                Expr::BinaryOperator {
                                    operator: BinaryOperator::LessEqual,
                                    lhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("a")] })),
                                    rhs: Box::new(Expr::NumericLiteral(NumericLiteral { value: 20, radix: Radix::Decimal }))
                                }
                            ),
                        }
                    ),
                    rhs: Box::new(
                        Expr::BinaryOperator {
                            operator: BinaryOperator::Equals,
                            lhs: Box::new(Expr::Reference(Reference { path: vec![Identifier::new("a")] })),
                            rhs: Box::new(Expr::NumericLiteral(NumericLiteral { value: 30, radix: Radix::Decimal }))
                        }
                    ),
                }
            ),
        }
    )]
    fn test_expr(#[case] input: &str, #[case] expected: Expr) {
        let result = expr().parse(input);
        assert_eq!(result.into_output().unwrap(), expected);
    }
}
