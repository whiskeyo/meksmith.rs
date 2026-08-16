use crate::frontend::ast::{BinOp, Expr};
use crate::frontend::token::Literal;

/// Evaluate simple integer expressions used in array lengths and padding.
/// Supports `param`, literals, `+`, `-`, `*`, `/`, and parentheses via nested `Binary`.
pub fn eval_u32(expr: &Expr, param_name: &str, param_value: u32) -> Option<u32> {
    match expr {
        Expr::Ident(name) if name == param_name => Some(param_value),
        Expr::Literal(Literal::Integer { value, .. }) if *value >= 0 => Some(*value as u32),
        Expr::Binary { op, lhs, rhs } => {
            let left = eval_u32(lhs, param_name, param_value)?;
            let right = eval_u32(rhs, param_name, param_value)?;
            match op {
                BinOp::Add => left.checked_add(right),
                BinOp::Sub => left.checked_sub(right),
                BinOp::Mul => left.checked_mul(right),
                BinOp::Div if right != 0 => Some(left / right),
                _ => None,
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::Expr;

    fn lit(value: i64) -> Expr {
        Expr::Literal(Literal::Integer { value, radix: 10 })
    }

    #[test]
    fn eval_param_minus_constant() {
        let expr = Expr::Binary {
            op: BinOp::Sub,
            lhs: Box::new(Expr::Ident("payload_size".into())),
            rhs: Box::new(lit(12)),
        };
        assert_eq!(eval_u32(&expr, "payload_size", 20), Some(8));
    }

    #[test]
    fn eval_param_minus_divided() {
        let expr = Expr::Binary {
            op: BinOp::Div,
            lhs: Box::new(Expr::Binary {
                op: BinOp::Sub,
                lhs: Box::new(Expr::Ident("payload_size".into())),
                rhs: Box::new(lit(4)),
            }),
            rhs: Box::new(lit(8)),
        };
        assert_eq!(eval_u32(&expr, "payload_size", 20), Some(2));
    }
}
