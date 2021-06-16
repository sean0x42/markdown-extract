# Markdown Extract

Extract sections of a markdown file. This project mostly exists to help me learn
Rust, and to fill a niche requirement for extracting patch notes from
a `CHANGELOG.md`.

## Use Cases

There aren't many, to be honest. 

1. I created this tool to extract patch notes from a `CHANGELOG.md` by version.
2. The talented folks at HashiCorp are using `markdown-extract` to extract API
   documentation, and inject it into OpenAPI schemas.

If you have another use for this tool, please let me know!

## Installation

If you've got Rust installed on your system, you can simple install
`markdown-extract` with Cargo.

```console
$ cargo install markdown-extract
```

### Docker

A Docker container is also available, and can be installed with the following
command:

```console
$ docker pull sean0x42/markdown-extract
```

You can then run the container with the following command:

```console
$ docker run -it sean0x42/markdown-extract --help
```

## Usage

View the help guide if you like.

```console
$ markdown-extract --help
markdown-extract 1.1.0
Extract sections of a markdown file

USAGE:
    markdown-extract [FLAGS] <pattern> <path>

FLAGS:
    -s, --case-sensitive          Treat pattern as case sensitive
    -f, --first                   Only return the first match
    -h, --help                    Prints help information
    -i, --ignore-first-heading    Do not include the top level section heading
    -r, --regex                   Compile pattern as a regular expression
    -V, --version                 Prints version information

ARGS:
    <pattern>    Pattern to match against section headings
    <path>       Path to markdown file
```

Then extract matching sections in a markdown file.

```console
$ markdown-extract --fr "^v1" CHANGELOG.md
## v1.1.0

...
```
