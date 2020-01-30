mod regex;
mod simple;

pub use self::regex::RegexMatcher;
use crate::document::Document;
use crate::Opts;
pub use simple::SimpleMatcher;

pub trait Matcher {
    /// Find any sections within the document that match
    fn get_matches(document: &Document, opts: &Opts) -> Document;
}
