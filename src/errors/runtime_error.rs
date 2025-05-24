use super::Error;

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
}

impl Error for RuntimeError {
    fn scope_binding_not_found(_ident: &str) -> Self {
        todo!()
    }

    fn scope_binding_already_exists(_ident: &str) -> Self {
        todo!()
    }
}
