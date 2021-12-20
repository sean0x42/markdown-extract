use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

pub type MarkdownSection = Vec<String>;

struct MarkdownHeading {
    depth: usize,
    content: String,
}

fn try_parse_heading(line: &str) -> Option<MarkdownHeading> {
    let mut depth = 0;

    for ch in line.chars() {
        match ch {
            '#' => depth += 1,
            _ => break,
        }
    }

    if depth == 0 {
        return None;
    }

    Some(MarkdownHeading {
        depth,
        content: line[depth..].trim().to_owned(),
    })
}

pub struct MarkdownExtract {
    matches: Vec<MarkdownSection>,
    is_within_matched_section: bool,
    is_inside_code_block: bool,
    depth: usize,
    current: Option<MarkdownSection>,
}

impl MarkdownExtract {
    fn default() -> Self {
        MarkdownExtract {
            matches: vec![],
            is_within_matched_section: false,
            is_inside_code_block: false,
            depth: 0,
            current: None,
        }
    }

    fn push_current(&mut self) {
        if let Some(current_match) = self.current.take() {
            self.matches.push(current_match);
        }
    }

    fn enter_matched_section(&mut self, heading: &MarkdownHeading) {
        self.is_within_matched_section = true;
        self.depth = heading.depth;

        self.push_current();
        self.current = Some(vec![]);
    }

    fn exit_matched_section(&mut self) {
        self.is_within_matched_section = false;
        self.push_current();
    }

    fn process_candidate_line(&mut self, line: &str, regex: &Regex) {
        let heading = try_parse_heading(line);

        if let Some(heading) = heading {
            if heading.depth <= self.depth {
                self.exit_matched_section();
            }

            if !self.is_within_matched_section && regex.is_match(&heading.content) {
                self.enter_matched_section(&heading);
            }
        }
    }

    pub fn extract_from_path(
        path: &PathBuf,
        regex: &Regex,
    ) -> Result<Vec<MarkdownSection>, std::io::Error> {
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        return Ok(MarkdownExtract::extract_from_reader(&mut reader, &regex));
    }

    pub fn extract_from_reader<R: Read>(
        reader: &mut BufReader<R>,
        regex: &Regex,
    ) -> Vec<MarkdownSection> {
        let mut state = MarkdownExtract::default();

        for raw_line in reader.lines() {
            let line = raw_line.unwrap();

            if line.starts_with("```") {
                state.is_inside_code_block = !state.is_inside_code_block;
            }

            if !state.is_inside_code_block {
                state.process_candidate_line(&line, &regex);
            }

            if state.is_within_matched_section {
                state.current.as_mut().unwrap().push(line);
            }
        }

        state.push_current();

        return state.matches;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_headings() {
        // When
        let value = try_parse_heading("### The quick ## brown fox #");

        // Then
        let heading = value.unwrap();
        assert_eq!(heading.depth, 3);
        assert_eq!(heading.content, "The quick ## brown fox #");
    }

    #[test]
    fn should_parse_non_headings() {
        // When
        let value = try_parse_heading("T#he quick brown fox ## jumped over the lazy dog");

        // Then
        assert_eq!(value.is_none(), true);
    }
}
