use super::{Document, Section};
use log::debug;
use regex::Regex;
use std::convert::TryInto;
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

pub struct Parser {
    re: Regex,
    document: Document,
    current: Option<Section>,
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
                if let Some(current) = &mut self.current {
                    current.append_to_body(line);
                }
                return;
            }
        };

        // Create new section object
        let mut new_section = Section::new();
        new_section.level = caps[1].len().try_into().unwrap();
        new_section.title = (&caps[2]).to_string();
        debug!("\n{:#?}", new_section);

        // Determine where to place new section in tree
        match &mut self.current {
            Some(current) => {
                if new_section.level < current.level {
                    // New section should be child
                    current.add_child(new_section);
                    debug!("Woof {:?}", current);
                } else if new_section.level == current.level {
                    // New section should be adjacent
                    self.document.push(current.clone());
                    self.current = Some(new_section);
                } else {
                    // Compare against parent
                    todo!("Climb tree");
                }
            }

            // No current section
            None => {
                self.current = Some(new_section);
            }
        }
    }
}
