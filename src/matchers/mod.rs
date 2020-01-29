mod regex;
mod simple;

pub use regex::RegexMatcher;
pub use simple::SimpleMatcher;
use crate::Opts;

pub trait Matcher {
    /// Find any sections within the document that match
    pub fn match(document: &Document, options: &Opts) -> Document;
}
