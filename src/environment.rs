// Environment: runtime state — maps variable names to their stored Variable (value + mutability + declared type).

use crate::value::{Value, Type};
use std::collections::HashMap;

// A bound variable's storage: its current value, whether it's reassignable (let vs const), and its declared type (for type checking).
pub struct Variable {
    value: Value,
    is_mutable: bool,
    declared_type: Type,
}

impl Variable {
    pub fn new(value: Value, is_mutable: bool, declared_type: Type) -> Self {
        Self {
            value,
            is_mutable,
            declared_type,
        }
    }
}

pub struct Environment {
    scopes: Vec<HashMap<String, Variable>>,
}  

impl Environment {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    // get only reads (immutable borrow); define modifies the map (mutable borrow).
    pub fn get(&self, name: &str) -> Value {
        for scope in self.scopes.iter().rev() {
            if let Some(var) = scope.get(name) {
                // Clone because the caller gets an owned Value while the environment keeps its copy.
                return var.value.clone();
            }
        }
        // Undefined variable is a panic for now; becomes a graceful error once Result-based error handling is added.
        panic!("undefined variable: {}", name);
    }

    pub fn define(&mut self, name: String, var: Variable) {
        let current_scope = self.scopes.last_mut().unwrap();
        if current_scope.contains_key(&name) {
            panic!("variable {} is already declared in this scope", name);
        }
        current_scope.insert(name, var);
        
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }
}