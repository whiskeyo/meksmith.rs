use chumsky::pratt::*;
use chumsky::prelude::*;

use crate::meklang2::ErrType;
use crate::meklang2::ast::{BinaryOperator, Expr, NumericLiteral, Radix, Reference, UnaryOperator};
use crate::meklang2::parser::ident::reference;
use crate::meklang2::parser::numeric::numeric_literal;
use crate::meklang2::parser::token::{
    AND, DOT, EQUALS_TO, GREATER_EQUAL, GREATER_THAN, LESS_EQUAL, LESS_THAN, LPAREN, NOT,
    NOT_EQUAL, OR, RPAREN,
};

pub(crate) fn expr<'src>() -> impl Parser<'src, &'src str, Expr, ErrType<'src>> {
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
            infix(left(2), operator(AND), |lhs, _, rhs, _| {
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

    use crate::meklang2::ast::Identifier;

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
