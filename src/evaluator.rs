// Evaluator: walks an Expr tree and computes its Value. Takes the environment (read-only) to look up variables.

use crate::ast::{Stmt, Expr, BinOp, UnaryOp};
use crate::value::Value;
use crate::environment::{Environment, Variable};

// Recursion over the tree: leaves (Int/Float/Variable) produce values directly; Binary/Unary evaluate children then combine. 
// Tree shape = order of operations.
pub fn eval(expr: &Expr, env: &Environment) -> Value {
    match expr {
        Expr::Int(n) => Value::Int(*n),
        Expr::Float(f) => Value::Float(*f),

        // Look the name up in the environment and return its stored value.
        Expr::Variable(name) => env.get(name),

        Expr::Binary(op, left, right) => {
            let left_value = eval(left, env);
            let right_value = eval(right, env);
            // Both operands int -> integer arithmetic (note: integer division truncates). Otherwise -> promote both to f64 and do float arithmetic. 
            // This is the type-promotion rule.
            // Match on references so we don't move the values — lets us reuse them (e.g. in to_f64) without requiring Value: Copy.
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
            let operand_value = eval(operand, env);
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

pub fn eval_stmt(stmt: &Stmt, env: &mut Environment) {
    match stmt {
        Stmt::Declaration { is_mutable, declared_type, name, value } => {
            let evaluated_value = eval(value, env);
            env.define(name.to_string(), Variable::new(evaluated_value, *is_mutable, declared_type.clone()));
        },
        Stmt::Print(expr) => println!("{}", eval(expr, env)),
        Stmt::Expr(expr) => { eval(expr, env); },
    }
}

// Converts a Value to f64 for the float-promotion path: ints widen, floats pass through.
fn to_f64(v: &Value) -> f64 {
    match v {
        Value::Int(n) => *n as f64,
        Value::Float(f) => *f,
    }
}