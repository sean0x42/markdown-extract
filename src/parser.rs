use super::{Document, Section};
use log::debug;
use regex::Regex;
use std::convert::TryInto;
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

pub struct Parser {
    re: Regex,
    document: Document,
    current: Option<usize>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            re: Regex::new(r"^(#{1,6}) (.*)$").unwrap(),
            document: Document::new(),
            current: None,
        }
    }

    pub fn parse_file(&mut self, file: File) -> Result<Document> {
        let reader = BufReader::new(file);

        // Parse line by line
        for line in reader.lines() {
            let line = line.unwrap();
            self.parse_line(line);
        }

        Ok(self.document.clone())
    }

    fn parse_line(&mut self, line: String) {
        // Get captures
        let caps = match self.re.captures(&line) {
            Some(caps) => caps,

            // Handle Regex not matching (e.g. normal line)
            None => {
                // Append to current section, if one has been defined
                if let Some(current) = self.current {
                    self.document[current].append_to_body(line);
                }

                return;
            }
        };

        // Create new section object
        let mut new_section = Section::new();
        new_section.level = caps[1].len().try_into().unwrap();
        new_section.title = (&caps[2]).to_string();

        // Find this section a home in the document
        let index = self.document.len();
        self.find_home(&mut new_section, index);
        self.document.push(new_section);
    }

    /// Find the given section a home within the document tree
    fn find_home(&mut self, section: &mut Section, index: usize) {
        // Get current section
        let current_index = match self.current {
            Some(current) => current,
            None => {
                debug!("Found initial section");
                self.current = Some(index);
                return;
            }
        };
        let current = &mut self.document[current_index];

        // Determine whether to add as child, sibling, or climb the tree
        if current.level < section.level {
            // New section should be child
            debug!("Found child section");
            section.parent = Some(current_index);
            current.add_child(index);
            self.current = Some(index);
        } else if current.level == section.level {
            // New section should be adjacent
            debug!("Found adjacent section");
            self.current = Some(index);
            section.parent = current.parent;

            // If parent exists, add new section as a child
            if let Some(parent) = section.parent {
                self.document[parent].add_child(index);
            }
        } else {
            // Compare against parent
            debug!("Found ancestor section. Climbing...");
            self.current = current.parent;
            self.find_home(section, index);
        }
    }
}
