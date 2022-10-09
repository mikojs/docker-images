use std::io;
use std::fmt;

pub enum ErrorKind {
    InvalidDockerName,
    NoDockerVersion,
    DockerVolumeNotExpected,
    CommandNotFound,
    CommandFail,
    DatabaseNotFound,
    DatabasePermissionDenied,
}

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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            _ => write!(f, "{}", self.message),
        }
    }
}

impl Error {
    pub fn new(error_kind: ErrorKind, message: String) -> Error {
        let kind = match error_kind {
            ErrorKind::InvalidDockerName => io::ErrorKind::InvalidInput,
            ErrorKind::NoDockerVersion => io::ErrorKind::InvalidInput,
            ErrorKind::DockerVolumeNotExpected => io::ErrorKind::InvalidData,
            ErrorKind::CommandNotFound => io::ErrorKind::NotFound,
            ErrorKind::CommandFail => io::ErrorKind::Interrupted,
            ErrorKind::DatabaseNotFound => io::ErrorKind::NotFound,
            ErrorKind::DatabasePermissionDenied => io::ErrorKind::PermissionDenied,
        };

        Error {
            kind,
            message,
        }
    }
}
