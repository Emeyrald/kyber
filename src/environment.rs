use crate::value::{Value, Type};
use std::collections::HashMap;

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
    globals: Vec<HashMap<String, Variable>>,
    call_stack: Vec<Frame>,
    functions: HashMap<String, Value>,
}  

struct Frame {
    scopes: Vec<HashMap<String, Variable>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            globals: vec![HashMap::new()],
            call_stack: Vec::new(),
            functions: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Value {
        if let Some(frame) = self.call_stack.last() {
            for scope in frame.scopes.iter().rev() {
                if let Some(var) = scope.get(name) {
                    return var.value.clone();
                }
            }
        }

        for scope in self.globals.iter().rev() {
            if let Some(var) = scope.get(name) {
                return var.value.clone();
            }
        }
        panic!("undefined variable: {}", name);
    }

    pub fn define(&mut self, name: String, var: Variable) {
        let current_scope = self.current_scopes_mut().last_mut().unwrap();
        if current_scope.contains_key(&name) {
            panic!("variable {} is already declared in this scope", name);
        }
        current_scope.insert(name, var);
        
    }

    pub fn assign(&mut self, name: String, new_value: Value) {
        if let Some(frame) = self.call_stack.last_mut() {
            for scope in frame.scopes.iter_mut().rev() {
                if let Some(var) = scope.get_mut(&name) {
                    if !var.is_mutable { panic!("variable {} is const and cannot be reassigned", name) }
                    let checked = var.declared_type.check_and_convert(&name, new_value);
                    var.value = checked;
                    return;
                }
            }
        }

        for scope in self.globals.iter_mut().rev() {
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
        self.current_scopes_mut().push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.current_scopes_mut().pop();
    }

    pub fn push_frame(&mut self) {
        self.call_stack.push(Frame { scopes: vec![HashMap::new()] });
    }

    pub fn pop_frame(&mut self) {
        self.call_stack.pop();
    }

    fn current_scopes_mut(&mut self) -> &mut Vec<HashMap<String, Variable>> {
        if let Some(frame) = self.call_stack.last_mut() {
            &mut frame.scopes
        } else {
            &mut self.globals
        }
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