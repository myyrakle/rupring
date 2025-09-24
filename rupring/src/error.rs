#[derive(Debug)]
pub enum Errors {}

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error occurred")
    }
}

pub type Result<T> = std::result::Result<T, Errors>;
