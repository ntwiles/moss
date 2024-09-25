pub mod runtime_error;
pub mod type_error;

pub trait Error {
    fn new(message: String) -> Self;
}
