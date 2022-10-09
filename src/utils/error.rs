use std::io;
use std::fmt;
use std::process;

#[derive(Debug)]
pub enum ErrorKind {
    CommandFail,
    Custom,
    Io,
}

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

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::Io => write!(f, "[IO] {}", self.message),
            _ => write!(f, "{}", self.message),
        }
    }
}

impl Error {
    pub fn new(kind: ErrorKind, message: String) -> Error {
        match kind {
            ErrorKind::CommandFail => process::exit(1),
            _ => Error {
                kind,
                message,
            }
        }
    }
}
