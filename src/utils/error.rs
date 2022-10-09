use std::io;

pub use io::ErrorKind;

#[derive(Debug)]
pub struct Error {
    kind: io::ErrorKind,
    message: String,
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error {
            kind: error.kind(),
            message: error.to_string(),
        }
    }
}

impl Error {
    pub fn new(kind: io::ErrorKind, message: String) -> Error {
        Error {
            kind,
            message,
        }
    }
}
