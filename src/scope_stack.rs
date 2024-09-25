use std::collections::HashMap;

type Scope<T> = HashMap<String, T>;

pub struct ScopeStack<T> {
    current: Vec<Scope<T>>,
    previous: Option<Vec<Scope<T>>>,
}

impl<T> ScopeStack<T> {
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

    pub fn create_new_stack(&mut self) {
        self.previous = Some(std::mem::take(&mut self.current));
        self.current = vec![Scope::new()]; // New isolated scope stack
    }

    pub fn restore_previous_stack(&mut self) {
        if let Some(prev) = self.previous.take() {
            self.current = prev;
        }
    }

    pub fn lookup(&self, name: &str) -> &T {
        for scope in self.current.iter().rev() {
            if let Some(value) = scope.get(name) {
                return value;
            }
        }

        unreachable!()
    }

    pub fn insert(&mut self, name: String, value: T) {
        self.current.last_mut().unwrap().insert(name, value);
    }
}
