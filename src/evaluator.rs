use crate::ast::{Expr, BinOp, UnaryOp};
use crate::value::Value;

pub fn eval(expr: &Expr) -> Value {
    match expr {
        Expr::Int(n) => Value::Int(*n),
        Expr::Float(n) => {},
        Expr::Binary(op, left, right) => {
            let left_value = eval(left);
            let right_value = eval(right);
            match (left_value, right_value) {
                (Value::Int(l), Value::Int(r)) => {
                    match op {
                        BinOp::Add => Value::Int(l + r),
                        BinOp::Subtract => Value::Int(l - r),
                        BinOp::Multiply => Value::Int(l * r),
                        BinOp::Divide => Value::Int(l / r),
                        BinOp::Modulo => Value::Int(l % r),
                    }
                },
                _ => panic!("type error"),
            }
        },
        Expr::Unary(op, operand) => {
            let operand_value = eval(operand);
            match operand_value {
                Value::Int(n) => {
                    match op {
                        UnaryOp::Negate => Value::Int(-n),
                    }
                },
                _ => panic!("type error"),
            }  
        },
    }
}