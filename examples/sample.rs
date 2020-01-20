use markdown_sections::{Document, Parser};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let file = File::open("examples/sample.md")?;
    let mut parser = Parser::new();
    let document = parser.parse_file(file)?;

    println!("{:?}", document);
    Ok(())
}
