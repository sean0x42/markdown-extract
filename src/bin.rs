mod commands;

use commands::{extract, view};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum MarkdownMagic {
    /// Previews a markdown file in the terminal
    #[structopt(visible_alias = "vw")]
    View,

    /// Extracts sections of raw markdown
    #[structopt(visible_alias = "ex")]
    Extract {
        #[structopt(short = "i", long)]
        case_insensitive: bool,

        /// Only return the first matching section
        #[structopt(short, long)]
        first: bool,

        /// Regular expression to match against section headings
        pattern: String,

        /// Path to markdown file
        #[structopt(parse(from_os_str))]
        path: PathBuf,
    },
}

fn main() {
    let opts = MarkdownMagic::from_args();
    println!("{:?}", opts);

    match opts {
        MarkdownMagic::View => view(),

        MarkdownMagic::Extract {
            case_insensitive,
            first,
            pattern,
            path,
        } => extract(pattern, path, case_insensitive, first),
    }
}
