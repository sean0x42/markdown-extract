use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct NoMatchesError;

impl NoMatchesError {
    /// Constructs a new `NoMatchesError`
    pub fn new() -> NoMatchesError {
        NoMatchesError {}
    }
}

impl fmt::Display for NoMatchesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No matches.")
    }
}

impl Error for NoMatchesError {}
