use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Node<T> {
    pub idx: usize,
    pub val: T,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl<T> Node<T>
where
    T: Serialize,
{
    pub fn new(idx: usize, val: T) -> Self {
        Node {
            idx,
            val,
            parent: None,
            children: Vec::new(),
        }
    }

    // fn serialize<S>(&self, doc: &Tree<T>, serializer: S) -> Result<S::Ok, S::Error>
    // where
    //     S: Serializer,
    // {
    //     let mut map = serializer.serialize_map(Some(self.children.len()))?;

    //     for child_id in self.children {
    //         let child = doc.by_id(child_id).unwrap();
    //         map.serialize_entry(&child.val, &child.serialize(doc, serializer)?)?;
    //     }

    //     map.end()
    // }
}
