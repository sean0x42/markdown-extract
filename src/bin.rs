use markdown_extract::{Document, Parser, Section};
use std::convert::TryInto;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

/// Extracts sections of a markdown file.
#[derive(StructOpt)]
#[structopt(name = "markdown-extract")]
struct Opts {
    /// Title is case sensitive
    #[structopt(short = "s", long)]
    case_sensitive: bool,

    /// Do not include the top level section heading
    #[structopt(short, long)]
    ignore_first_heading: bool,

    /// A title to find in section headings
    title: String,

    /// Path to markdown file
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

/// Print a section to stdout
fn print_section(document: &Document, section: &Section, ignore_first_heading: bool) {
    // Only print first heading if needed
    if !ignore_first_heading {
        println!(
            "{} {}",
            "#".repeat(section.level.try_into().unwrap()),
            section.title
        );
    }
    println!("{}", section.body);

    // Print children
    for child in &section.children {
        print_section(&document, &document[*child], false);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    // Get opts
    let opts = Opts::from_args();

    // Create parser and get file
    let mut parser = Parser::new();
    let file = File::open(&opts.path)?;
    let document = parser.parse_file(file)?;

    let matches: Document = document
        .iter()
        .filter(|section| {
            if opts.case_sensitive {
                section.title.trim() == opts.title.trim()
            } else {
                section.title.to_lowercase().trim() == opts.title.to_lowercase().trim()
            }
        })
        .cloned()
        .collect();

    // Handle no matches
    if matches.is_empty() {
        println!("No matches.");
        return Ok(());
    }

    // Print matching sections
    for section in matches {
        print_section(&document, &section, opts.ignore_first_heading);
    }

    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(error) => {
            println!("Error: {}", error);
            1
        }
    });
}
