# Changelog

## v2.0.0 (January 2021)

In this release, `markdown-extract` has been dramatically simplified, and comes
with a more sensible API out of the box. There are a number of breaking changes.

- Fixed matching headings inside code blocks.
- Remove `--regex` flag. All inputs will be treated as regular expressions.
- Remove `--first` flag. This is now the default behaviour.
- Add `--all` flag. When setting this flag, all matches will be printed (not
  just the first).
- Renamed `--ignore-first-heading` to `--no-print-matched-heading`. Behaviour is
  the same.
- Clarified help text for the `--no-print-matched-heading` flag.

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
