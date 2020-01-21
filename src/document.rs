/// A section within a markdown document
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Section {
    pub level: u8,
    pub title: String,
    body: String,
    children: Vec<Section>,
}

impl Section {
    /// Construct a new section
    pub fn new() -> Section {
        Section {
            level: 0,
            title: String::new(),
            body: String::new(),
            children: Vec::new(),
        }
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

    /// Add a child to this section.
    pub fn add_child(&mut self, child: Section) {
        self.children.push(child);
    }
}

/// A collection of sections from a markdown document
pub type Document = Vec<Section>;
