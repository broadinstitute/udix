use std::env::VarError;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use std::time::SystemTimeError;

#[derive(Copy, Clone, Debug)]
pub enum ErrorKind { Udix, VarError, Io, Toml, Utf8, ParseInt, SystemTime }

#[derive(Clone)]
pub struct Error {
    kind: ErrorKind,
    message: String,
    source: Option<Box<Error>>,
}

impl Error {
    pub fn new(kind: ErrorKind, message: String, source: Option<Box<Error>>) -> Error {
        Error { kind, message, source }
    }
    pub fn report(&self) -> String {
        match &self.source {
            None => { format!("{} ({})", self.message, self.kind) }
            Some(source) => { format!("{} ({}): {}", self.message, self.kind, source) }
        }
    }
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Error::from(message.to_string())
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        let kind = ErrorKind::Udix;
        let source: Option<Box<Error>> = None;
        Error::new(kind, message, source)
    }
}

impl From<VarError> for Error {
    fn from(var_error: VarError) -> Self {
        from_error(ErrorKind::VarError, &var_error)
    }
}

impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Self {
        from_error(ErrorKind::Io, &io_error)
    }
}

impl From<toml::de::Error> for Error {
    fn from(toml_error: toml::de::Error) -> Self {
        from_error(ErrorKind::Toml, &toml_error)
    }
}

impl From<Utf8Error> for Error {
    fn from(utf8_error: Utf8Error) -> Self {
        from_error(ErrorKind::Utf8, &utf8_error)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(utf8_error: FromUtf8Error) -> Self {
        from_error(ErrorKind::Utf8, &utf8_error)
    }
}

impl From<ParseIntError> for Error {
    fn from(parse_int_error: ParseIntError) -> Self {
        from_error(ErrorKind::ParseInt, &parse_int_error)
    }
}

impl From<SystemTimeError> for Error {
    fn from(system_time_error: SystemTimeError) -> Self {
        from_error(ErrorKind::SystemTime, &system_time_error)
    }
}

fn from_error(kind: ErrorKind, error: &dyn std::error::Error) -> Error {
    let message = error.to_string();
    let source: Option<Box<Error>> = None;
    Error::new(kind, message, source)
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", self) }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.report())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.report())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|source| source.as_ref() as &dyn std::error::Error)
    }
}