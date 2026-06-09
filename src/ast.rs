// AST: the parsed structure of Kyber expressions. Represents syntax (what was written), distinct from runtime values.

use crate::value::Type;

#[derive(Debug)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

#[derive(Debug)]
pub enum UnaryOp {
    Negate,
}

// An Expr is anything that produces a value. 
// Binary/Unary children are Boxed because a type can't contain itself directly (infinite size) — Box is a pointer, fixed size.
#[derive(Debug)]
pub enum Expr {
    Int(i64),
    Float(f64),

    // Variable holds only the variable's name (syntax). 
    // Resolving it to a value happens at eval time via the environment — the AST never holds runtime data.
    Variable(String),

    // Which operation; kept as separate enums so the evaluator dispatches uniformly and the compiler checks all operators are handled.
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
}

#[derive(Debug)]
pub enum Stmt {
    Declaration {
        is_mutable: bool,
        declared_type: Type,
        name: String,
        value: Expr,
    },
    Print(Expr),
    Expr(Expr),
}