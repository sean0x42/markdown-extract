# markdown-extract

Extract sections of a markdown file with a regular expression.

## Usage

Given a document called `my-document.md`:

```markdown
# Welcome

This is my amazing markdown document.

## Extract me!

This section should be pulled out.
```

You can extract the second section with the following:

```rust
use markdown_extract::extract_from_path;
use regex::Regex;

fn main() {
    let regex = Regex::new(r"Extract me!").unwrap();
    let extracted = extract_from_path("my-document.md", &regex).unwrap();
    println!("{}", extracted);
}
```
