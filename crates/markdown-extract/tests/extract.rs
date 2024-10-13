use markdown_extract::extract_from_path;
use regex::{Regex, RegexBuilder};
use std::path::PathBuf;

fn create_regex(pattern: &str) -> Regex {
    RegexBuilder::new(pattern)
        .case_insensitive(true)
        .size_limit(1024 * 100) // 100 kb
        .build()
        .unwrap()
}

#[test]
fn should_handle_multiple_matching_sections() {
    // Given
    let path = PathBuf::from(r"tests/markdown/multiple_matches.md");
    let regex = create_regex("^%%");

    // When
    let matches = extract_from_path(&path, &regex).unwrap();

    // Then
    assert_eq!(matches.len(), 2);
    assert_eq!(matches[0][0], "## %% (Heading 1)");
    assert_eq!(matches[1][0], "# %% (Heading 2)");
}

#[test]
fn should_not_match_headings_in_code_blocks() {
    // Given
    let path = PathBuf::from(r"tests/markdown/heading_in_code_block.md");
    let regex = create_regex("^%%");

    // When
    let matches = extract_from_path(&path, &regex).unwrap();

    // Then
    assert_eq!(matches.len(), 0);
}
