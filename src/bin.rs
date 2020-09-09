mod commands;

use commands::{extract, view};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum MarkdownMagic {
    /// Previews a markdown file in the terminal
    #[structopt(visible_alias = "vw")]
    View,

    /// Extracts sections of raw markdown from a file, given a regular
    /// expression to match against section headings.
    #[structopt(visible_alias = "ex")]
    Extract {
        /// Ignores case
        #[structopt(short = "i", long)]
        case_insensitive: bool,

        /// Prints only the first matching section
        #[structopt(short, long)]
        first: bool,

        /// Avoids printing headings that match the given pattern
        #[structopt(short = "g", long)]
        ignore_matching_headings: bool,

        /// Regular expression to match against section headings
        pattern: String,

        /// Path to markdown file
        #[structopt(parse(from_os_str))]
        path: PathBuf,
    },
}

fn main() {
    let opts = MarkdownMagic::from_args();

    match opts {
        MarkdownMagic::View => view(),

        MarkdownMagic::Extract {
            case_insensitive,
            first,
            ignore_matching_headings,
            pattern,
            path,
        } => extract(
            pattern,
            path,
            case_insensitive,
            first,
            ignore_matching_headings,
        ),
    }
}
