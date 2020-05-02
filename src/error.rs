use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum InjectError {
    FailedCast,
    MissingProvider,
}

impl Display for InjectError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            InjectError::FailedCast => write!(f, "failed cast"),
            InjectError::MissingProvider => write!(f, "no provider available"),
        }
    }
}

impl Error for InjectError {}
