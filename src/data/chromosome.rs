use std::fmt::{Display, Formatter};
use crate::error::Error;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub(crate) enum Chromosome {
    Auto(u8),
    Allo(char),
}

impl Chromosome {
    pub(crate) fn parse(string: &str) -> Result<Chromosome, Error> {
        let stripped =
            if let Some(stripped) = string.strip_prefix("chr") {
                stripped
            } else if let Some(stripped) = string.strip_prefix('c') {
                stripped
            } else {
                string
            };
        if stripped == "X" {
            Ok(Chromosome::Allo('X'))
        } else if stripped == "Y" {
            Ok(Chromosome::Allo('Y'))
        } else {
            let number = stripped.parse::<u8>()?;
            Ok(Chromosome::Auto(number))
        }
    }
}

impl Display for Chromosome {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Chromosome::Auto(num) => { write!(f, "{}", num) }
            Chromosome::Allo(sym) => { write!(f, "{}", sym) }
        }
    }
}
