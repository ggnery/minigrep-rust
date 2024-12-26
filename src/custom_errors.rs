use std::{error::Error, fmt::{self, Debug, Display}};

pub enum GreepError {
    NotEnoughtArgs,
    ManyArgs,
    GenericError(&'static str)
}

impl fmt::Display for GreepError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::GenericError(e) => { write!(f, "{e}") },
            Self::NotEnoughtArgs => write!(f, "NotEnoughtArgs error: Passing less than 2 arguments"),
            Self::ManyArgs => write!(f, "ManyArgs: To many arguments"),
        }
    }
}

impl Debug for GreepError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { Display::fmt(&self, f) }
}

impl Error for GreepError {}