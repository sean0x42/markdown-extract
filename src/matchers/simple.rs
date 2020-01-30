use super::Matcher;
use crate::document::Document;
use crate::Opts;

pub struct SimpleMatcher;

impl Matcher for SimpleMatcher {
    /// Performs a simple pattern == title match
    fn get_matches(document: &Document, opts: &Opts) -> Document {
        document
            .iter()
            .filter(|section| {
                if opts.case_sensitive {
                    section.title.trim() == opts.pattern.trim()
                } else {
                    section.title.to_lowercase().trim() == opts.pattern.to_lowercase().trim()
                }
            })
            .cloned()
            .collect()
    }
}
