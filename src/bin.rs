pub mod document;
mod matchers;
mod parser;

use document::{Document, Section};
use matchers::{Matcher, RegexMatcher, SimpleMatcher};
use parser::Parser;
use std::convert::TryInto;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

/// Extract sections of a markdown file.
#[derive(StructOpt)]
#[structopt(name = "markdown-extract")]
pub struct Opts {
    /// Only return the first match
    #[structopt(short, long)]
    first: bool,

    /// Compile pattern as a regular expression.
    ///
    /// Documentation for the regex syntax can be found at
    /// <https://docs.rs/regex/1.3.3/regex/index.html#syntax>
    #[structopt(short, long)]
    regex: bool,

    /// Treat pattern as case sensitive
    #[structopt(short = "s", long)]
    case_sensitive: bool,

    /// Do not include the top level section heading
    #[structopt(short, long)]
    ignore_first_heading: bool,

    /// Pattern to match against section headings
    pattern: String,

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
    println!("{}", section.body.join("\n"));

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

    // Match
    let matches = if opts.regex {
        RegexMatcher::get_matches(&document, &opts)
    } else {
        SimpleMatcher::get_matches(&document, &opts)
    };

    // Handle no matches
    if matches.is_empty() {
        println!("No matches.");
        return Ok(());
    }

    // Only print the first match
    if opts.first {
        // It's okay to use `[0]` here since we check if the doc is empty above
        print_section(&document, &matches[0], opts.ignore_first_heading);
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
