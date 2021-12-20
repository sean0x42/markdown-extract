use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum Block {
    Document,
    Paragraph(String),
    List,
    ListItem,
    BlockQuote,
}
