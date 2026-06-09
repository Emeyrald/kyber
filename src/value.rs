// Value: a runtime value in Kyber (the result of evaluation). Type: the declared kind of a variable. Value = actual contents, Type = the annotation.

use std::fmt;

// Clone (not Copy) so it survives gaining a String variant later, which can't be Copy.
#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
}

// Whole-number floats (5.0) print with a trailing .0 so the type is visible in output; ints never show a decimal. Fractional floats print as-is.
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(x) => {
                // Exact float equality is normally risky, but .fract() of a whole value is exactly 0.0, so this is safe here.
                if x.fract() == 0.0 {
                    write!(f, "{:.1}", x)
                } else {
                    write!(f, "{}", x)
                }             
            },
        }
    }
}