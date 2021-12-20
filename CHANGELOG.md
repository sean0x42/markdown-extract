# Changelog

Patch notes are automatically extracted from this changelog whenever a tag is
pushed to the GitHub repository. The tag name must match a heading exactly.

## v2.0.0

Published on x.x.x

## v1.1.0

- Exit with code 1 if no matches are found (#3, thanks @brennerm)
- Publish `markdown-extract` as a Docker image (#2, thanks @brennerm)

## v1.0.0

The first proper release of `markdown-extract`! :tada:

- Add `--regex` flag, which enables the use of regular expressions to search
  for section titles.
- Add `--first` flag, which only prints the first matching section.
- Fix an issue where extra newlines where inserted into the final output.

## v0.1.1

- Publish as a binary instead of a library

## v0.1.0-alpha

This version is the initial release of `markdown_extract`! It features the
following:

- Extract sections from a markdown document
- Run from the command line
- Use as a Rust library
