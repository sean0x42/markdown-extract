pub mod block;
pub mod document;
pub mod node;
mod validator;

use block::Block;
pub use document::Document;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use validator::validate;

pub fn parse_from_str(string: String) -> Document {
    let mut parser = Parser::new();

    string.split("\n").for_each(|line| parser.parse_line(line));

    return parser.document;
}

pub fn parse_from_path(path: PathBuf) -> Document {
    let file = File::open(path).expect("file not found");
    let mut reader = BufReader::new(file);
    return parse_from_reader(&mut reader);
}

pub fn parse_from_reader<R: Read>(reader: &mut BufReader<R>) -> Document {
    let mut parser = Parser::new();

    for line in reader.lines() {
        let l = line.unwrap();
        parser.parse_line(&l);
    }

    return parser.document;
}

struct Parser {
    document: Document,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            document: Document::new(),
        }
    }

    pub fn parse_line(&mut self, line: &str) {
        let open_blocks = self.document.open_blocks();

        let mut processed_line = line;

        let candidate_blocks = open_blocks.iter().filter(|node| match node.val {
            Block::Document => true,

            Block::BlockQuote => validate(&mut processed_line)
                .consume_space(0..3)
                .consume_char('>')
                .consume_space(0..1)
                .evaluate(),

            Block::Paragraph(_content) => !line.is_empty(),
        });
    }
}
