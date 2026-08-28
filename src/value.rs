// Value: a runtime value in Kyber (the result of evaluation). Type: the declared kind of a variable. Value = actual contents, Type = the annotation.

use std::fmt;

// Clone (not Copy) so it survives gaining a String variant later, which can't be Copy.
#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
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
            Value::Bool(b) => write!(f, "{}", b),
        }
    }
}

impl Type {
    // Checks a value against a declared type, returning the value to store. Widens int->float; 
    // errors on narrowing (float->int) or type mismatch. Used by both declaration and reassignment.
    pub(crate) fn check_and_convert(&self, name: &str, value: Value) -> Value {
        match (self, &value) {
            (Type::Int, Value::Int(_)) | (Type::Float, Value::Float(_)) | (Type::Bool, Value::Bool(_)) => value,
            (Type::Float, Value::Int(n)) => Value::Float(*n as f64),
            (Type::Int, Value::Float(_)) => panic!("cannot assign float to int variable {} (use a cast to truncate)", name),
            _ => panic!("type mismatch: cannot assign to {:?} variable {}", self, name),
        }
    }
}