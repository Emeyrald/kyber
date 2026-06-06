use crate::ast::{Expr, BinOp, UnaryOp};
use crate::value::Value;

pub fn eval(expr: &Expr) -> Value {
    match expr {
        Expr::Int(n) => Value::Int(*n),
        Expr::Float(f) => Value::Float(*f),
        Expr::Binary(op, left, right) => {
            let left_value = eval(left);
            let right_value = eval(right);
            match (&left_value, &right_value) {
                (Value::Int(l), Value::Int(r)) => {
                    match op {
                        BinOp::Add => Value::Int(l + r),
                        BinOp::Subtract => Value::Int(l - r),
                        BinOp::Multiply => Value::Int(l * r),
                        BinOp::Divide => Value::Int(l / r),
                        BinOp::Modulo => Value::Int(l % r),
                    }
                },
                _ => {
                    let l = to_f64(&left_value);
                    let r = to_f64(&right_value);
                    match op {
                        BinOp::Add => Value::Float(l + r),
                        BinOp::Subtract => Value::Float(l - r),
                        BinOp::Multiply => Value::Float(l * r),
                        BinOp::Divide => Value::Float(l / r),
                        BinOp::Modulo => Value::Float(l % r),
                    }
                }
            }
        },
        Expr::Unary(op, operand) => {
            let operand_value = eval(operand);
            match &operand_value {
                Value::Int(n) => {
                    match op {
                        UnaryOp::Negate => Value::Int(-*n),
                    }
                },
                Value::Float(f) => {
                    match op {
                        UnaryOp::Negate => Value::Float(-*f),
                    }
                }
            }  
        },
    }
}

fn to_f64(v: &Value) -> f64 {
    match v {
        Value::Int(n) => *n as f64,
        Value::Float(f) => *f,
    }
}