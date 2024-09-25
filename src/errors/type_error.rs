use super::Error;

#[derive(Debug)]
pub struct TypeError {
    pub message: String,
}

impl Error for TypeError {
    fn new(message: String) -> Self {
        TypeError { message }
    }
}
