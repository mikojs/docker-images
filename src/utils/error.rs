use std::io;
use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    InvalidDockerName,
    NoDockerVersion,
    DockerVolumeNotExpected,
    CommandNotFound,
    CommandFail,
    DatabaseNotFound,
    DatabasePermissionDenied,
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            _ => write!(f, "{}", self.message),
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
