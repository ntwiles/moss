use core::fmt;
use core::fmt::Debug;
use std::fmt::Formatter;

use super::scope::Scope;
use crate::errors::Error;

pub struct ScopeStack<T> {
    current: Vec<Scope<T>>,
    previous: Option<Vec<Scope<T>>>,
}

impl<T: Debug> ScopeStack<T> {
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

    pub fn lookup<E: Error>(&self, ident: &str) -> Result<&T, E> {
        for scope in self.current.iter().rev() {
            if let Some(value) = scope.get(ident) {
                return Ok(value);
            }
        }

        Err(E::scope_binding_not_found(ident))
    }

    pub fn insert(&mut self, name: String, value: T) {
        self.current.last_mut().unwrap().insert(name, value);
    }
}

impl<T: Debug> Debug for ScopeStack<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "ScopeStack {{")?;

        writeln!(f, "  current: [")?;
        for (i, scope) in self.current.iter().enumerate() {
            writeln!(f, "    Scope {}: {:?}", i, scope)?;
        }
        writeln!(f, "  ]")?;

        match &self.previous {
            Some(prev_scopes) => {
                writeln!(f, "  previous: [")?;
                for (i, scope) in prev_scopes.iter().enumerate() {
                    writeln!(f, "    Scope {}: {:?}", i, scope)?;
                }
                writeln!(f, "  ]")?;
            }
            None => {
                writeln!(f, "  previous: None")?;
            }
        }

        writeln!(f, "}}")
    }
}
