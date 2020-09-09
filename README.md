<div align="center">

![Markdown Magic](./logo.svg)

</div>

---

A magical tool belt for working with markdown. You can:

- [**Preview**](#preview) markdown files in your terminal (Coming soon) ✨
- [**Build**](#build) markdown to HTML (Coming soon) ✨
- [**Format**](#format) a markdown file (Coming soon) ✨
- [**Extract**](#extract) sections according to a regular expression ✨

## Installation 🧙

> TODO

You probably also want to add an alias to your `.bashrc` or `.zshrc`.

```bash
alias md="markdown-magic"
```

## Usage

### Preview

Coming soon!

### Build

Coming soon!

### Format

Coming soon!

### Extract

The `extract` command lets you pull out sections of markdown according to
a regular expression. Given a document called `my-document.md`:

```markdown
# Welcome!

This is my amazing markdown document.

## Extract me!

This section should be pulled out.
```

We could then extract the second section with the following command:

```console
$ markdown-magic extract "Extract me!" my-document.md
## Extract me!

This section should be pulled out.
```

For more help and configuration, use the following command:

```console
$ markdown-magic help extract
```

## Version 1

If you used the old `markdown-extract` tool, the [old docs are still
available][old-readme].

[old-readme]: https://github.com/sean0x42/markdown-extract/blob/401d385bc68f9048a19e5a3dbe2c8d7bbb5292c0/README.md
