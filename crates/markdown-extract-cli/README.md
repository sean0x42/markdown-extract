<div align="center">

# Markdown Extract CLI

[![Crates.io](https://img.shields.io/crates/v/markdown-extract)](https://crates.io/crates/markdown-extract)
[![Docker Pulls](https://img.shields.io/docker/pulls/sean0x42/markdown-extract)](https://hub.docker.com/r/sean0x42/markdown-extract)
[![Build & Test](https://github.com/sean0x42/markdown-extract/actions/workflows/build_and_test.yml/badge.svg)](https://github.com/sean0x42/markdown-extract/actions/workflows/build_and_test.yml)

</div>

Extract sections of a markdown file with a regular expression.

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
$ cargo install markdown-extract-cli
```
