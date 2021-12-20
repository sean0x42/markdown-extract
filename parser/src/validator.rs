use std::ops::Range;

enum Requirement {
    Space(Range<u8>),
    Char(char),
}

pub struct Validator {
    line: &'static mut str,
    requirements: Vec<Requirement>,
}

pub fn validate(line: &'static mut str) -> Validator {
    Validator {
        line,
        requirements: Vec::new(),
    }
}

impl Validator {
    pub fn consume_space(&mut self, range: Range<u8>) -> &mut Self {
        self.requirements.push(Requirement::Space(range));
        self
    }

    fn evaluate_and_consume_space(&mut self, range: Range<u8>) -> bool {
        let space_count = 0;

        for ch in self.line.chars() {
            match ch {
                ' ' => {
                    self.line = &mut self.line[1..];
                    space_count += 1;
                }
                _ => break,
            }
        }

        return space_count >= range.start && space_count < range.end;
    }

    pub fn consume_char(&mut self, ch: char) -> &mut Self {
        self.requirements.push(Requirement::Char(ch));
        self
    }

    fn evaluate_and_consume_char(&'static mut self, ch: char) -> bool {
        if self.line.chars().nth(0) == Some(ch) {
            self.line = &'static mut self.line[1..];
            return true;
        }

        return false;
    }

    pub fn evaluate(&mut self) -> bool {
        let chars = self.line.chars().peekable();

        for requirement in self.requirements {
            let is_match = match requirement {
                Requirement::Char(ch) => self.evaluate_and_consume_char(ch),
                Requirement::Space(range) => self.evaluate_and_consume_space(range),
            };

            if !is_match {
                return false;
            }
        }

        return true;
    }
}
