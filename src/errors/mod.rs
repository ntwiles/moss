pub mod runtime_error;
pub mod type_error;

// TODO: This is very hacky, this trait probably shouldn't exist.
pub trait Error {
    fn scope_binding_not_found(name: &str) -> Self;
    fn scope_binding_already_exists(name: &str) -> Self;
}
