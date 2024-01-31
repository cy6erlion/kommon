//! 🚨 kommon errors

use std::fmt;

/// kommon Errors
#[derive(Debug)]
pub enum Error {
    /// Gender
    Gender,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gender => write!(f, "Gender"),
        }
    }
}
