use regex::{Regex, RegexBuilder};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

struct Extractor {
    pattern: Regex,
    first: bool,
    ignore_matching_headings: bool,

    // State
    is_printing: bool,
    is_inside_pre: bool,
    print_depth: u8,
}

impl Extractor {
    pub fn new(
        pattern: &str,
        case_insensitive: bool,
        first: bool,
        ignore_matching_headings: bool,
    ) -> Self {
        let re = RegexBuilder::new(&pattern)
            .size_limit(1024 * 100) // 100 kb
            .case_insensitive(case_insensitive)
            .build()
            .unwrap();

        Extractor {
            pattern: re,
            first,
            ignore_matching_headings,
            is_printing: false,
            is_inside_pre: false,
            print_depth: 0,
        }
    }

    pub fn run(&mut self, path: PathBuf) {
        let file = File::open(path).expect("File not found!");
        let reader = BufReader::new(file);
        let mut section_count = 0;

        for line in reader.lines() {
            let line = line.unwrap();

            // Toggle pre formatted block
            if line.starts_with("```") {
                self.is_inside_pre = !self.is_inside_pre;
            }

            // Handle pre formatted block
            if self.is_inside_pre {
                self.print(&line);
                continue;
            }

            let heading_depth = self.is_heading_candidate(&line);

            // Not a heading or not a relevant heading
            if heading_depth.is_none() {
                self.print(&line);
                continue;
            }

            let is_match = self.does_heading_match(&line);

            // Cancel early if we only want the first match
            if self.first && section_count == 1 {
                return;
            }

            if is_match {
                section_count += 1;
                self.is_printing = true;
                self.print_depth = heading_depth.unwrap();

                if !self.ignore_matched_heading {
                    self.print(&line);
                }
            } else {
                self.is_printing = false;
            }
        }
    }

    fn is_heading_candidate(&self, line: &str) -> Option<u8> {
        // Quick exit if this is definitely not a heading
        if !line.starts_with("#") {
            return None;
        }

        // Count heading depth
        let mut chars = line.chars();
        let mut depth: u8 = 0;

        loop {
            // Check next char
            let ch = chars.next();
            if ch.is_none() {
                break;
            }

            // Continue only of char is a #
            match ch.unwrap() {
                '#' => depth += 1,
                _ => break,
            };
        }

        // Ignore headings deeper than the current print depth, since they are guaranteed to be
        // printed anyway
        if self.is_printing && depth > self.print_depth {
            return None;
        }

        return match depth {
            0 => None,
            _ => Some(depth),
        };
    }

    fn does_heading_match(&self, line: &str) -> bool {
        return self.pattern.is_match(line.trim_start_matches('#').trim());
    }

    /// Prints the provided line if the extractor is currently printing
    fn print(&self, line: &str) {
        if !self.is_printing {
            return;
        }

        println!("{}", line);
    }
}

pub fn extract(
    pattern: String,
    path: PathBuf,
    case_insensitive: bool,
    first: bool,
    ignore_matching_headings: bool,
) {
    Extractor::new(&pattern, case_insensitive, first, ignore_matching_headings).run(path);
}
