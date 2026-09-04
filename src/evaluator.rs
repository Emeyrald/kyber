// Evaluator: walks the AST and executes it. Two entry points:
// - eval(expr, &Environment): computes an expression's Value (read-only - expressions don't mutate state).
// - eval_stmt(stmt, &mut Environment): executes a statement (mutates the environment - declarations, assignments, and blocks push/pop scopes).

use crate::ast::{Stmt, Expr, BinOp, UnaryOp};
use crate::value::{Value, Type};
use crate::environment::{Environment, Variable};

pub enum Flow {
    Normal,
    Return(Value),
    Break,
    Continue,
}

// Recursion over the tree: leaves (Int/Float/Variable) produce values directly; Binary/Unary evaluate children then combine. 
// Tree shape = order of operations.
pub fn eval(expr: &Expr, env: &mut Environment) -> Value {
    match expr {
        Expr::Call { name, arguments } => {
            let function = env.get_function(name).clone();
            let arg_values: Vec<Value> = arguments.iter().map(|arg| eval(arg, env)).collect();

            match function {
                Value::Function { parameters, return_type, body } => {
                    let num_params = parameters.len();
                    let num_args = arg_values.len();
                    if num_params != num_args { panic!("wrong number of arguments for {name}. expected {num_params}, got {num_args}"); }

                    env.push_frame();

                    for (param, arg_value) in parameters.iter().zip(arg_values) {
                        let checked_value = param.param_type.check_and_convert(&param.name, arg_value);
                        env.define(param.name.clone(), Variable::new(checked_value, true, param.param_type.clone()));
                    }

                    let body_flow = eval_block(&body, env);
                    let return_value: Value = match body_flow {
                        Flow::Return(v) => v,
                        Flow::Normal => Value::Void,
                        Flow::Break | Flow::Continue => panic!("break/continue outside of loop"),
                    };

                    env.pop_frame();
                    let checked_value = return_type.check_and_convert("return", return_value);
                    checked_value
                },
                _ => panic!("{} is not a function", name),
            }
        },

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

pub fn eval_stmt(stmt: &Stmt, env: &mut Environment) -> Flow {
    match stmt {
        Stmt::Declaration { is_mutable, declared_type, name, value } => {
            let evaluated_value = eval(value, env);
            let checked_value = declared_type.check_and_convert(name, evaluated_value);
            env.define(name.to_string(), Variable::new(checked_value, *is_mutable, declared_type.clone()));
            Flow::Normal
        },
        Stmt::Assignment { name, value } => {
            let evaluated_value = eval(value, env);
            env.assign(name.to_string(), evaluated_value);
            Flow::Normal
        },
        Stmt::If { condition, then_branch, else_branch } => { 
            let condition_value = eval(condition, env);
            match condition_value {
                Value::Bool(b) => { 
                    if b {
                        eval_block(then_branch, env)
                    } else if let Some(stmt) = else_branch {
                        eval_stmt(stmt, env)
                    } else {
                        Flow::Normal
                    }
                },
                _ => panic!("expected boolean for condition"),
            }
        },
        Stmt::While { condition, body } => {
            let result = loop {
                let condition_value = eval(condition, env);
                match condition_value {
                    Value::Bool(b) => {
                        if b {
                            let flow = eval_block(body, env);
                            match flow {
                                Flow::Break => break Flow::Normal,
                                Flow::Continue => continue,
                                Flow::Return(v) => break Flow::Return(v),
                                Flow::Normal => (),
                            }
                        } else {
                            break Flow::Normal;
                        }
                    },
                    _ => panic!("expected boolean for condition"),
                }
            };
            result
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
            let result = loop {
                if !should_continue(counter, end_int, step_int, *inclusive) { break Flow::Normal; }

                env.assign(var.clone(), Value::Int(counter));
                let flow = eval_block(body, env);
                counter += step_int;
                match flow {
                    Flow::Break => break Flow::Normal,
                    Flow::Return(v) => break Flow::Return(v),
                    Flow::Normal | Flow::Continue => (),
                } 
            };
            env.pop_scope();
            result
        },
        Stmt::FunctionDef { name, parameters, return_type, body } => {
            let function = Value::Function {
                parameters: parameters.clone(),
                return_type: return_type.clone(),
                body: body.clone(),
            };
            env.define_function(name.clone(), function);
            Flow::Normal
        },
        Stmt::Block(statements) => { eval_block(statements, env) },
        Stmt::Print(expr) => {
            println!("{}", eval(expr, env));
            Flow::Normal
        },
        Stmt::Expr(expr) => { 
            eval(expr, env); 
            Flow::Normal
        },
        Stmt::Return(expr) => {
            match expr {
                Some(e) => {
                    let value = eval(e, env);
                    Flow::Return(value)
                },
                None => Flow::Return(Value::Void),
            }
        },
        Stmt::Break => Flow::Break,
        Stmt::Continue => Flow::Continue,
    }
}

fn eval_block(statements: &[Stmt], env: &mut Environment) -> Flow {
    env.push_scope();
    for statement in statements {
        let flow = eval_stmt(statement, env);
        if !matches!(flow, Flow::Normal) {
            env.pop_scope();
            return flow;
        }
    }
    env.pop_scope();
    Flow::Normal
}

fn eval_int(expr: &Expr, env: &mut Environment, context: &str) -> i64 {
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