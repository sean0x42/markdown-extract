use super::{Document, Section};
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

enum State {
    /// Ready to consume a heading
    Ready,

    /// Currently scanning a heading at the given depth
    Heading(u8),
}

pub struct Parser {
    state: State,
    document: Document,
    current: Option<Section>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            state: State::Ready,
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
        // Break down line into chars
        let chars = line.trim().chars();
        for ch in chars {
            println!("{}", ch);
            match self.state {
                State::Ready => self.handle_ready(ch),
                State::Heading(level) => self.handle_heading(level, ch),
            };
        }
    }

    fn handle_ready(&mut self, ch: char) {
        match ch {
            '#' => {
                self.state = State::Heading(1);

                // Add old section to document if one exists
                match &self.current {
                    Some(section) => self.document.push(section.clone()),
                    None => {}
                }
            }

            _ => {}
        };
    }

    fn handle_heading(&mut self, level: u8, ch: char) {
        match ch {
            '#' => self.state = State::Heading(level + 1),
            _ => {
                let mut section = Section::new();

                if let State::Heading(level) = self.state {
                    section.level(level);
                }

                self.current = Some(section);
                // Consume remaining characters and call that the title
            }
        }
    }
}

// /// Parse a markdown document from a file
// pub fn parse_from_file(file: File) -> Result<Document> {
//     let reader = BufReader::new(file);
//     let mut current: Option<Section> = None;
//     let mut depth = 0;

//     for line in reader.lines() {
//         let line = line.unwrap();
//         let chars = line.trim().chars();
//         for ch in chars {
//             match state {
//                 State::Ready => {
//                     match ch {
//                         '#' => {
//                             state = State::Heading;
//                             depth += 1;
//                         }
//                         _ => break,
//                     };
//                 }

//                 State::Heading => match ch {
//                     '#' => depth += 1,
//                     _ => {
//                         state = State::Ready;
//                         current = Some(Section::new(depth, chars.as_str().to_string()));
//                     }
//                 },
//             }
//         }
//     }

//     Ok(Document::new())
// }
