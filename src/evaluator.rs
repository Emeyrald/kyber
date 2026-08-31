// Evaluator: walks the AST and executes it. Two entry points:
// - eval(expr, &Environment): computes an expression's Value (read-only - expressions don't mutate state).
// - eval_stmt(stmt, &mut Environment): executes a statement (mutates the environment - declarations, assignments, and blocks push/pop scopes).

use crate::ast::{Stmt, Expr, BinOp, UnaryOp};
use crate::value::{Value, Type};
use crate::environment::{Environment, Variable};

// Recursion over the tree: leaves (Int/Float/Variable) produce values directly; Binary/Unary evaluate children then combine. 
// Tree shape = order of operations.
pub fn eval(expr: &Expr, env: &Environment) -> Value {
    match expr {
        Expr::Int(n) => Value::Int(*n),
        Expr::Float(f) => Value::Float(*f),
        Expr::Bool(b) => Value::Bool(*b),

        // Look the name up in the environment and return its stored value.
        Expr::Variable(name) => env.get(name),

        Expr::Binary(op, left, right) => {
            let left_value = eval(left, env);
            let right_value = eval(right, env);

            match op {
                // Arithmetic -> existing (Int,Int)/promote logic, produces number
                BinOp::Add | BinOp::Subtract | BinOp::Multiply | BinOp::Divide | BinOp::Modulo => {
                    match (&left_value, &right_value) {
                        (Value::Int(l), Value::Int(r)) => {
                            match op {
                                BinOp::Add => Value::Int(l + r),
                                BinOp::Subtract => Value::Int(l - r),
                                BinOp::Multiply => Value::Int(l * r),
                                BinOp::Divide => Value::Int(l / r),
                                BinOp::Modulo => Value::Int(l % r),
                                _ => unreachable!(),
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
                                _ => unreachable!(),
                            }
                        }
                    }
                },
                // Ordering comparisons -> always compare as f64, produce Bool
                BinOp::Less | BinOp::Greater | BinOp::LessEqual | BinOp::GreaterEqual => {
                    let l = to_f64(&left_value);
                    let r = to_f64(&right_value);
                    match op {
                        BinOp::Less => Value::Bool(l < r),
                        BinOp::Greater => Value::Bool(l > r),
                        BinOp::LessEqual => Value::Bool(l <= r),
                        BinOp::GreaterEqual => Value::Bool(l >= r),
                        _ => unreachable!(),
                    }
                },
                // Equality -> works on numbers AND bools, produce Bool
                BinOp::EqualEqual | BinOp::NotEqual => {
                    match (&left_value, &right_value) {
                        (Value::Bool(l), Value::Bool(r)) => {
                            Value::Bool(match op { 
                                BinOp::EqualEqual => l == r, 
                                BinOp::NotEqual => l != r,
                                _ => unreachable!(),
                            })
                        },
                        (Value::Bool(_), _) | (_, Value::Bool(_)) => panic!("type error: can't compare bool with number"),
                        _ => {
                            let l = to_f64(&left_value);
                            let r = to_f64(&right_value);
                            Value::Bool(match op { 
                                BinOp::EqualEqual => l == r, 
                                BinOp::NotEqual => l != r,
                                _ => unreachable!(),
                            })
                        }
                    }
                },
            } 
        },
        Expr::Unary(op, operand) => {
            let operand_value = eval(operand, env);
            match (op, &operand_value) {
                (UnaryOp::Negate, Value::Int(n)) => Value::Int(-*n),
                (UnaryOp::Negate, Value::Float(f)) => Value::Float(-*f),
                (UnaryOp::Not, Value::Bool(b)) => Value::Bool(!*b),
                _ => panic!("type error: can't apply this operator to this type")
            }  
        },
    }
}

pub fn eval_stmt(stmt: &Stmt, env: &mut Environment) {
    match stmt {
        Stmt::Declaration { is_mutable, declared_type, name, value } => {
            let evaluated_value = eval(value, env);
            let checked_value = declared_type.check_and_convert(name, evaluated_value);
            env.define(name.to_string(), Variable::new(checked_value, *is_mutable, declared_type.clone()));
        },
        Stmt::Assignment { name, value } => {
            let evaluated_value = eval(value, env);
            env.assign(name.to_string(), evaluated_value);
        },
        Stmt::If { condition, then_branch, else_branch } => { 
            let condition_value = eval(condition, env);
            match condition_value {
                Value::Bool(b) => { 
                    if b {
                        eval_block(then_branch, env);
                    } else {
                        if let Some(stmt) = else_branch {
                            eval_stmt(stmt, env);
                        }
                    }
                },
                _ => panic!("expected boolean for condition"),
            }
        },
        Stmt::While { condition, body } => {
            loop {
                let condition_value = eval(condition, env);
                match condition_value {
                    Value::Bool(b) => {
                        if b {
                            eval_block(body, env);
                        } else {
                            break;
                        }
                    },
                    _ => panic!("expected boolean for condition"),
                }
            }
        },  
        Stmt::For { var, start, inclusive, end, step, body } => {
            let start_int = eval_int(start, env, "start");
            let end_int = eval_int(end, env, "end");
            let step_int = match step {
                Some(e) => eval_int(e, env, "step"),
                None => 1,
            };
            if step_int == 0 { panic!("step amount cannot be 0 in for loop"); }

            env.push_scope();

            let mut counter = start_int;
            env.define(var.clone(), Variable::new(Value::Int(start_int), true, Type::Int));
            while should_continue(counter, end_int, step_int, *inclusive) {
                env.assign(var.clone(), Value::Int(counter));
                eval_block(body, env);
                counter += step_int;
            }

            env.pop_scope();

        },
        Stmt::Block(statements) => {
            eval_block(statements, env);
        },
        Stmt::Print(expr) => println!("{}", eval(expr, env)),
        Stmt::Expr(expr) => { eval(expr, env); },
    }
}

fn eval_block(statements: &[Stmt], env: &mut Environment) {
    env.push_scope();
    for statement in statements {
        eval_stmt(statement, env);
    }
    env.pop_scope();
}

fn eval_int(expr: &Expr, env: &Environment, context: &str) -> i64 {
    match eval(expr, env) {
        Value::Int(n) => n,
        _ => panic!("for loop {} must be an integer", context),
    }
}

// Converts a Value to f64 for the float-promotion path: ints widen, floats pass through.
fn to_f64(v: &Value) -> f64 {
    match v {
        Value::Int(n) => *n as f64,
        Value::Float(f) => *f,
        _ => panic!("expected int or float"),
    }
}

fn should_continue(counter: i64, end: i64, step: i64, inclusive: bool) -> bool {
    if step > 0 {
        if inclusive { counter <= end } else { counter < end }
    } else {
        if inclusive { counter >= end } else { counter > end }
    }
}