mod heading;
mod state;

use heading::try_parse_heading;
use regex::Regex;
use state::State;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::PathBuf,
};

pub type MarkdownSection = Vec<String>;

pub fn extract_from_path(
    path: &PathBuf,
    regex: &Regex,
) -> Result<Vec<MarkdownSection>, std::io::Error> {
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    return Ok(extract_from_reader(&mut reader, &regex));
}

pub fn extract_from_reader<R: Read>(
    reader: &mut BufReader<R>,
    regex: &Regex,
) -> Vec<MarkdownSection> {
    let mut state = State::default();

    for line in reader.lines() {
        let line = line.unwrap();

        if line.starts_with("```") {
            state.is_inside_code_block = !state.is_inside_code_block;
        }

        if !state.is_inside_code_block {
            let heading = try_parse_heading(&line);

            if let Some(heading) = heading {
                if heading.depth <= state.depth {
                    state.exit_matched_section();
                }

                if !state.is_within_matched_section && regex.is_match(&heading.content) {
                    state.enter_matched_section(&heading);
                }
            }
        }

        if state.is_within_matched_section {
            state.current.as_mut().unwrap().push(line);
        }
    }

    state.push_current();

    return state.matches;
}
