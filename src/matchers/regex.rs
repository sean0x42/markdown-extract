use super::Matcher;
use crate::document::Document;
use crate::Opts;
use regex::RegexBuilder;

pub struct RegexMatcher;

impl Matcher for RegexMatcher {
    /// Compile the pattern as a regular expression
    fn get_matches(document: &Document, opts: &Opts) -> Document {
        // Compile regex for provided pattern
        let re = RegexBuilder::new(&opts.pattern)
            .case_insensitive(!opts.case_sensitive)
            .size_limit(1024 * 100) // 100 kb
            .build()
            .unwrap();

        document
            .iter()
            .filter(|section| re.is_match(&section.title))
            .cloned()
            .collect()
    }
}
