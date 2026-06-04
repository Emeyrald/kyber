#[derive(Debug)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Binary(BinOp, Box<Expr>, Box<Expr>),
}