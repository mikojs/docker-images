use std::io;
use std::fmt;
use std::process;

use glob;
use regex;

#[derive(Debug)]
pub enum ErrorKind {
    CommandFail,
    Custom,
    Io,
    Glob,
    Regex,
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

impl From<glob::PatternError> for Error {
    fn from(error: glob::PatternError) -> Self {
        Error {
            kind: ErrorKind::Glob,
            message: error.to_string(),
        }
    }
}

impl From<regex::Error> for Error {
    fn from(error: regex::Error) -> Self {
        Error {
            kind: ErrorKind::Regex,
            message: error.to_string(),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::Io => write!(f, "[IO] {}", self.message),
            ErrorKind::Glob => write!(f, "[Glob] {}", self.message),
            ErrorKind::Regex => write!(f, "[Regex] {}", self.message),
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
