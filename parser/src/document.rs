// use serde::ser::{SerializeMap, Serializer};
use super::node::Node;
use super::{Block, BlockKind};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct Document {
    #[serde(flatten)]
    nodes: HashMap<usize, Node<Block>>,

    #[serde(skip)]
    pub next_id: usize,
}

type NodeBlock = Node<Block>;

impl Document {
    pub fn new() -> Self {
        let mut nodes = HashMap::new();
        nodes.insert(0, Node::new(0, Block::Document));

        Document { nodes, next_id: 1 }
    }

    pub fn by_id(&self, id: usize) -> Option<&Node<Block>> {
        self.nodes.get(&id)
    }

    pub fn insert_node(&mut self, parent_id: usize, node: NodeBlock) {
        self.nodes
            .get_mut(&parent_id)
            .unwrap()
            .children
            .push(node.idx);
        self.nodes.insert(node.idx, node);
        self.next_id += 1;
    }

    pub fn open_blocks(&self) -> Vec<&NodeBlock> {
        let root = self.by_id(0).unwrap();
        return self.find_open_children(root);
    }

    fn find_open_children<'a>(&'a self, block: &'a NodeBlock) -> Vec<&'a NodeBlock> {
        let mut open_children = vec![block];

        for child_id in &block.children {
            let child = self.by_id(*child_id).unwrap();
            open_children.append(&mut self.find_open_children(child));
        }

        return open_children;
    }
}
