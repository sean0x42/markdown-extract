<div align="center">

# Markdown Extract

[![Crates.io](https://img.shields.io/crates/v/markdown-extract)](https://crates.io/crates/markdown-extract)
[![Docker Pulls](https://img.shields.io/docker/pulls/sean0x42/markdown-extract)](https://hub.docker.com/r/sean0x42/markdown-extract)

</div>

Extract sections of a markdown file with a regular expression! Great for changelogs ;)

## Usage

Given a document called `my-document.md`:

```markdown
# Welcome

This is my amazing markdown document.

## Extract me!

This section should be pulled out.
```

You can extract the second section with the following command:

```console
$ markdown-extract "Extract me!" my-document.md
## Extract me!

This section should be pulled out.
```

## Installation

If you've got Rust installed on your system, you can simply install
`markdown-extract` with Cargo.

```console
$ cargo install markdown-extract
```

### Docker

A Docker container is also available, and can be installed with the following
command:

```console
$ docker pull sean0x42/markdown-extract:v2
```

You can then run the container with the following command:

```console
$ docker run -it sean0x42/markdown-extract:v2 --help
```

## Use Cases

There aren't many, to be honest. 

1. Extract patch notes from a `CHANGELOG.md` by version.
2. The talented folks at HashiCorp are using `markdown-extract` to extract API
   documentation, and inject it into OpenAPI schemas.

If you have another use for this tool, please let me know!
