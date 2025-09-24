#[derive(Debug)]
pub enum Errors {
    StreamClosed,
    StreamSendError(String),
}

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Errors::StreamClosed => write!(f, "Stream is closed"),
            Errors::StreamSendError(msg) => write!(f, "Failed to send to stream: {}", msg),
        }
    }
}

pub type Result<T> = std::result::Result<T, Errors>;
