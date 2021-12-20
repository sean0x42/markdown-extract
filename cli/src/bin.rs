use markdown_magic_parser::parse_from_path;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum MarkdownMagic {
    /// Extracts sections of raw markdown from a file, given a regular
    /// expression to match against section headings.
    #[structopt(visible_alias = "ex")]
    Extract {
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
        MarkdownMagic::Extract { pattern: _, path } => {
            parse_from_path(path);
        }
    }
}
