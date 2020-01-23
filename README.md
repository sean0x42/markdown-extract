# Markdown Extract

Extract sections of a markdown file. This project mostly exists to help me learn
Rust, and to fill a niche requirement for extracting patch notes from
a `CHANGELOG.md`.


## Usage 

Start by installing `markdown-extract`. Requires Cargo.

```
cargo install markdown-extract 
```

View the help guide if you like.

```console
$ markdown-extract -h
markdown-extract 0.1.1
Extracts sections of a markdown file

USAGE:
    markdown-extract [FLAGS] <title> <path>

FLAGS:
    -s, --case-sensitive          Title is case sensitive
    -h, --help                    Prints help information
    -i, --ignore-first-heading    Do not include the top level section heading
    -V, --version                 Prints version information

ARGS:
    <title>    A title to find in section headings
    <path>     Path to markdown file
```

Then extract matching sections in a markdown file.

```console
$ markdown-extract v0.1.1 CHANGELOG.md
## v0.1.1

...
```


## Use Cases

There aren't many. I created this tool to extract patch notes from a
`CHANGELOG.md` by version. If you have another use for this binary, please let
me know!
