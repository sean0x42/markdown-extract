/// A section within a markdown document
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Section {
    /// Section depth. In markdown, legal values range from 1 to 6, although this implementation
    /// technically allows more.
    pub level: u8,

    /// Title of section
    pub title: String,

    /// Raw markdown body.
    /// Does not include any child sections. See the `children` property for child content.
    pub body: String,

    /// An optional pointer to a parent section.
    /// This property should always be `Some`, unless the section is located at the root of the
    /// document.
    pub parent: Option<usize>,

    /// A vector of child sections
    pub children: Vec<usize>,
}

impl Section {
    /// Construct a new section
    pub fn new() -> Section {
        Section {
            level: 0,
            title: String::new(),
            body: String::new(),
            parent: None,
            children: Vec::new(),
        }
    }

    /// Appends the given line to the section's body
    pub fn append_to_body(&mut self, line: String) {
        self.body.push_str(&line);
        self.body.push('\n');
    }

    /// Add a child to this section.
    pub fn add_child(&mut self, child: usize) {
        self.children.push(child);
    }
}

/// A collection of sections from a markdown document
pub type Document = Vec<Section>;
