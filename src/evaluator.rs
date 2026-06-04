use crate::ast::{Expr, BinOp};

pub fn eval(expr: &Expr) -> i32 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Binary(op, left, right) => {
            let left_value = eval(left);
            let right_value = eval(right);
            match op {
                BinOp::Add => left_value + right_value,
                BinOp::Subtract => left_value - right_value,
                BinOp::Multiply => left_value * right_value,
                BinOp::Divide => left_value / right_value,
                BinOp::Modulo => left_value % right_value,
            }
        }
    }
}