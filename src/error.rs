//! Errors due to injection resolution failure
//!
//! When injection fails due to a provider not being available, or a downcast has gone awry.
//!
//! Most probably, the encountered error will be `InjectError::MissingProvider`.
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum InjectError {
    /// Returned when down-casting has failed for the installed provider. Very rare.
    FailedCast,
    /// Returned when a provider for the type is not present within
    /// the [`Container`](../struct.Container.html)
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
