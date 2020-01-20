/// A section within a markdown document
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Section {
    level: u8,
    title: String,
    body: String,
}

impl Section {
    /// Construct a new section
    pub fn new() -> Section {
        Section {
            level: 0,
            title: String::new(),
            body: String::new(),
        }
    }

    /// Set section level
    pub fn level(&mut self, level: u8) {
        self.level = level;
    }

    /// Set the section's title
    pub fn title(&mut self, title: String) {
        self.title = title;
    }

    /// Appends the given line to the section's body
    ///
    /// Note: This function always leaves a trailing new line.
    ///
    /// # Examples
    ///
    /// ```
    /// let section = Section::new(1, String::from("Hello, world!"));
    /// section.append_to_body(String::from(""));
    /// section.append_to_body(String::from("The quick brown fox jumps over the lazy dog"));
    ///
    /// assert_eq!(section.body, "\nThe quick brown fox jumps over the lazy dog\n");
    /// ```
    pub fn append_to_body(&mut self, line: String) {
        self.body.push_str(&line);
        self.body.push('\n');
    }
}

/// A collection of sections from a markdown document
pub type Document = Vec<Section>;
