use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use markdown_extract::{extract_from_path, MarkdownSection};
use regex::RegexBuilder;
use std::{
    io::{self, Write},
    path::PathBuf,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Print all matching sections (don't quit after first match)
    #[arg(short, long)]
    all: bool,

    /// Treat pattern as case sensitive
    #[arg(short = 's', long)]
    case_sensitive: bool,

    /// Do not include the matched heading in the output
    #[arg(short, long)]
    no_print_matched_heading: bool,

    /// Pattern to match against headings
    #[arg(value_name = "PATTERN")]
    pattern: String,

    /// Path to markdown file
    #[arg(value_name = "FILE")]
    path: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let regex = RegexBuilder::new(&cli.pattern)
        .case_insensitive(!cli.case_sensitive)
        .size_limit(1024 * 100) // 100 kb
        .build()
        .unwrap();

    let matches = extract_from_path(&cli.path, &regex)
        .with_context(|| format!("Unable to extract at path: {}", cli.path.display()))?;

    if matches.len() == 0 {
        bail!("No matches found for pattern: {}", cli.pattern);
    }

    if !cli.all {
        return print_section(&matches[0], cli.no_print_matched_heading);
    }

    // match is a reserved keyword
    for m in matches {
        print_section(&m, cli.no_print_matched_heading)?;
    }

    Ok(())
}

fn print_section(section: &MarkdownSection, skip_printing_matched_heading: bool) -> Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for line in section
        .iter()
        .skip(if skip_printing_matched_heading { 1 } else { 0 })
    {
        writeln!(handle, "{}", line).with_context(|| format!("Failed to print line: {}", line))?;
    }

    handle.flush().map_err(|err| anyhow!(err))
}
