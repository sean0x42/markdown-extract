pub struct MarkdownHeading {
    pub depth: usize,
    pub content: String,
}

pub fn try_parse_heading(line: &str) -> Option<MarkdownHeading> {
    let mut depth = 0;

    for ch in line.chars() {
        match ch {
            '#' => depth += 1,
            _ => break,
        }
    }

    if depth == 0 {
        return None;
    }

    Some(MarkdownHeading {
        depth,
        content: line[depth..].trim().to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_headings() {
        // When
        let value = try_parse_heading("### The quick ## brown fox #");

        // Then
        let heading = value.unwrap();
        assert_eq!(heading.depth, 3);
        assert_eq!(heading.content, "The quick ## brown fox #");
    }

    #[test]
    fn should_parse_non_headings() {
        // When
        let value = try_parse_heading("T#he quick brown fox ## jumped over the lazy dog");

        // Then
        assert_eq!(value.is_none(), true);
    }
}
