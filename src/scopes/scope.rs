use std::{
    collections::HashMap,
    fmt::Formatter,
    ops::{Deref, DerefMut},
};

use core::fmt;
use core::fmt::Debug;

pub struct Scope<T>(HashMap<String, T>);

impl<T> Scope<T> {
    pub fn new() -> Self {
        Scope(HashMap::new())
    }
}

impl<T> Deref for Scope<T> {
    type Target = HashMap<String, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Scope<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Debug> Debug for Scope<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{")?;

        let len = self.len();
        for (i, (k, v)) in self.iter().enumerate() {
            // Optional: abbreviate known verbose variants like function definitions
            // TODO: what is this? Functions don't even use this syntax anymore.
            let v_display = match format!("{:?}", v) {
                s if s.starts_with("TypedFunc") => "Function(...)".to_string(),
                s => s,
            };

            let comma = if i + 1 == len { "" } else { "," };
            writeln!(f, "      \"{}\": {}{}", k, v_display, comma)?;
        }

        writeln!(f, "    }}")
    }
}
