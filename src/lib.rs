pub mod cli_helper;
pub mod fs;

#[derive(Debug)]
pub enum CustomError {
    InvalidInput(&'static str),
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::InvalidInput(detail) => write!(f, "Invalid input: {}", detail),
        }
    }
}

impl std::error::Error for CustomError {}
