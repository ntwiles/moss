pub mod runtime_error;
pub mod type_error;

pub trait Error {
    fn scope_binding_not_found(name: &str) -> Self;
}
