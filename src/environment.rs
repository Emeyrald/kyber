// Environment: runtime variable storage as a stack of scopes. Each scope is a name→Variable map; 
// entering a block pushes a scope, exiting pops it. Lookups and assignments search innermost-outward for lexical scoping.

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
    functions: HashMap<String, Value>,
}  

impl Environment {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            functions: HashMap::new(),
        }
    }

    // get/assign search scopes innermost-outward; define inserts into the innermost scope.
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

    // Reassignment: find the nearest scope with the name, enforce mutability and type, update in place. 
    // Errors if const or undeclared.
    pub fn assign(&mut self, name: String, new_value: Value) {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(var) = scope.get_mut(&name) {
                if !var.is_mutable { panic!("variable {} is const and cannot be reassigned", name) }
                let checked = var.declared_type.check_and_convert(&name, new_value);
                var.value = checked;
                return;
            }
        }
        panic!("cannot assign to undeclared variable {}", name)
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn define_function(&mut self, name: String, func: Value) {
        if self.functions.contains_key(&name) { panic!("function {} is already declared in this scope", name); }
        self.functions.insert(name, func);
    }

    pub fn get_function(&mut self, name: &str) -> Value {
        if let Some(func) = self.functions.get(name) {
            return func.clone();
        }
        panic!("undefined function: {}", name);
        
    }
}