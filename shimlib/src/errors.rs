use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct TagError {
    message: String,
}

impl TagError {
    pub fn new(msg: &str) -> TagError {
        TagError {
            message: format!("Invalid Tag: {}", msg),
        }
    }
}

impl fmt::Display for TagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for TagError {
    fn description(&self) -> &str {
        &self.message
    }
}
