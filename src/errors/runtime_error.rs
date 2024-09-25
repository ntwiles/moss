use super::Error;

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
}

impl Error for RuntimeError {
    fn new(message: String) -> Self {
        RuntimeError { message }
    }
}
