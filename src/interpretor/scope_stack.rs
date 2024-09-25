use std::collections::HashMap;

use super::resolved_value::ResolvedValue;

pub type Scope = HashMap<String, ResolvedValue>;

pub struct ScopeStack {
    current: Vec<Scope>,
    previous: Option<Vec<Scope>>,
}

impl ScopeStack {
    pub fn new() -> Self {
        Self {
            current: vec![Scope::new()],
            previous: None,
        }
    }

    pub fn push_scope(&mut self) {
        self.current.push(Scope::new());
    }

    pub fn pop_scope(&mut self) {
        self.current.pop();
    }

    // Create a new isolated scope stack for non-closures
    pub fn create_new_stack(&mut self) {
        // Store the current stack in `previous` before creating a new one
        self.previous = Some(std::mem::take(&mut self.current));
        self.current = vec![Scope::new()]; // New isolated scope stack
    }

    // Restore the previous scope stack after a non-closure function call
    pub fn restore_previous_stack(&mut self) {
        if let Some(prev) = self.previous.take() {
            self.current = prev;
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&ResolvedValue> {
        for scope in self.current.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value);
            }
        }
        None
    }

    pub fn insert(&mut self, name: String, value: ResolvedValue) {
        self.current.last_mut().unwrap().insert(name, value);
    }
}
