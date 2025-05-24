use super::Error;

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
}

impl Error for RuntimeError {
    fn scope_binding_not_found(ident: &str) -> Self {
        RuntimeError {
            message: format!("Binding for identifier \"{ident}\" not found in scope."),
        }
    }

    fn scope_binding_already_exists(ident: &str) -> Self {
        RuntimeError {
            message: format!("Scope binding already exists for identifier \"{ident}\"."),
        }
    }
}
