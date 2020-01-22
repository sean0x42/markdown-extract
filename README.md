# Markdown Extract

Extract sections of a markdown file. This project mostly exists to help me learn
Rust, and to fill a niche requirement for extracting patch notes from
a `CHANGELOG.md`.


## Usage 

Start by installing `markdown_extract`. Requires Cargo.

```
cargo install markdown_extract 
```

View the help guide if you like.

```console
$ markdown-extract -h
markdown-extract 0.1.0-alpha
Parses a markdown document into a tree of sections. Then returns any sections which match the given pattern

USAGE:
    markdown-extract [FLAGS] <pattern> <path>

FLAGS:
    -s, --case-sensitive    Regular expression should be treated as case sensitive
    -h, --help              Prints help information
        --no-children       Do not include any child sections in the output
        --no-heading        Do not include the top level section heading
    -V, --version           Prints version information

ARGS:
    <title>      A title to find in section headings
    <path>       Path to markdown file
```

Then extract matching sections in a markdown file.

```console
$ markdown-extract v0.1.0-alpha CHANGELOG.md
## v0.1.0-alpha

...
```


## Use Cases

There aren't many. I created this tool to extract patch notes from a
`CHANGELOG.md` by version. If you have another use for this binary, please let
me know!
