use std::io;

#[derive(Debug)]
pub enum ErrorKind {
    CommandFail,
    Custom,
    Io,
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error {
            kind: ErrorKind::Io,
            message: error.to_string(),
        }
    }
}

impl Error {
    pub fn new(kind: ErrorKind, message: String) -> Error {
        Error {
            kind,
            message,
        }
    }
}
