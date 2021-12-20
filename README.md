# Markdown Extract

Extract sections of a markdown file according to a regular expression. Given a
document called `my-document.md`:

```markdown
# Welcome!

This is my amazing markdown document.

## Extract me!

This section should be pulled out.
```

You could then extract the second section with the following command:

```console
$ markdown-extract "Extract me!" my-document.md
## Extract me!

This section should be pulled out.
```

For more help and configuration, use the following command:

```console
$ markdown-extract --help
```

## Use Cases

There aren't many, to be honest. 

1. I created this tool to extract patch notes from a `CHANGELOG.md` by version.
2. The talented folks at HashiCorp are using `markdown-extract` to extract API
   documentation, and inject it into OpenAPI schemas.

If you have another use for this tool, please let me know!

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
$ docker pull sean0x42/markdown-extract
```

You can then run the container with the following command:

```console
$ docker run -it sean0x42/markdown-extract --help
```
