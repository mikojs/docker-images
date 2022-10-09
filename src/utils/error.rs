use std::fmt;
use std::io;
use std::process;
use std::string;

use glob;
use regex;
use shellwords;

#[derive(Debug)]
pub enum ErrorKind {
    CommandFail,
    Custom,
    Io,
    String,
    Glob,
    Regex,
    Shellwords,
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

impl From<string::FromUtf8Error> for Error {
    fn from(error: string::FromUtf8Error) -> Self {
        Error {
            kind: ErrorKind::String,
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

impl From<shellwords::MismatchedQuotes> for Error {
    fn from(error: shellwords::MismatchedQuotes) -> Self {
        Error {
            kind: ErrorKind::Shellwords,
            message: error.to_string(),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::Io => write!(f, "[IO] {}", self.message),
            ErrorKind::String => write!(f, "[String] {}", self.message),
            ErrorKind::Glob => write!(f, "[Glob] {}", self.message),
            ErrorKind::Regex => write!(f, "[Regex] {}", self.message),
            ErrorKind::Shellwords => write!(f, "[Shellwords] {}", self.message),
            _ => write!(f, "{}", self.message),
        }
    }
}

impl Error {
    pub fn new(kind: ErrorKind, message: String) -> Error {
        match kind {
            ErrorKind::CommandFail => process::exit(1),
            _ => Error { kind, message },
        }
    }
}
