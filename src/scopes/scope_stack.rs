use core::fmt;
use core::fmt::Debug;
use std::collections::hash_map::Entry;
use std::fmt::Formatter;

use super::scope::Scope;
use crate::errors::Error;

#[derive(Debug)]
pub struct ScopeEntry<T> {
    pub is_mutable: bool,
    pub value: T,
}

pub struct ScopeStack<T> {
    current: Vec<Scope<ScopeEntry<T>>>,
    previous: Option<Vec<Scope<ScopeEntry<T>>>>,
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

    pub fn lookup<E: Error>(&self, ident: &str) -> Result<&ScopeEntry<T>, E> {
        for scope in self.current.iter().rev() {
            if let Some(entry) = scope.get(ident) {
                return Ok(entry);
            }
        }

        Err(E::scope_binding_not_found(ident))
    }

    pub fn insert<E: Error>(&mut self, ident: String, is_mutable: bool, value: T) -> Result<(), E> {
        let curr_scope = self.current.last_mut().unwrap();
        match curr_scope.entry(ident.clone()) {
            Entry::Vacant(v) => {
                v.insert(ScopeEntry { value, is_mutable });
                Ok(())
            }
            Entry::Occupied(_) => Err(E::scope_binding_already_exists(&ident)),
        }
    }

    pub fn mutate<E: Error>(&mut self, ident: &str, value: T) -> Result<(), E> {
        for scope in self.current.iter_mut().rev() {
            if let Some(entry) = scope.get_mut(ident) {
                entry.value = value;
                return Ok(());
            }
        }

        Err(E::scope_binding_not_found(ident))
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
