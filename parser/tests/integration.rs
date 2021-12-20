use markdown_magic_parser::parse_from_str;

#[test]
fn test_block_quotes() {
    // When
    let doc = parse_from_str("> This is a block quote".to_owned());

    // Then
    insta::assert_yaml_snapshot!(doc, @r###"
    ---
    1:
      idx: 1
      val:
        Paragraph: "> This is a block quote"
      parent: 0
      children: []
    0:
      idx: 0
      val: Document
      parent: ~
      children:
        - 1
    "###);
}
