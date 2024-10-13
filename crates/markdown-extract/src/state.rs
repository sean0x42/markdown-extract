use crate::heading::MarkdownHeading;
use crate::MarkdownSection;

pub struct State {
    pub matches: Vec<MarkdownSection>,
    pub is_within_matched_section: bool,
    pub is_inside_code_block: bool,
    pub depth: usize,
    pub current: Option<MarkdownSection>,
}

impl State {
    pub fn default() -> Self {
        State {
            matches: vec![],
            is_within_matched_section: false,
            is_inside_code_block: false,
            depth: 0,
            current: None,
        }
    }

    pub fn push_current(&mut self) {
        if let Some(current_match) = self.current.take() {
            self.matches.push(current_match);
        }
    }

    pub fn enter_matched_section(&mut self, heading: &MarkdownHeading) {
        self.is_within_matched_section = true;
        self.depth = heading.depth;

        self.push_current();
        self.current = Some(vec![]);
    }

    pub fn exit_matched_section(&mut self) {
        self.is_within_matched_section = false;
        self.push_current();
    }
}
