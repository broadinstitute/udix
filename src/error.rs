use std::fmt::{Debug, Display, Formatter};

#[derive(Clone)]
pub struct Error {
    message: String,
    source: Option<Box<Error>>,
}

impl Error {
    pub fn new(message: String, source: Option<Box<Error>>) -> Error {
        Error { message, source }
    }
    pub fn report(&self) -> String {
        match &self.source {
            None => { self.message.clone() }
            Some(source) => { format!("{}: {}", self.message, source) }
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
        let source: Option<Box<Error>> = None;
        Error::new(message, source)
    }
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