//! Basic DOM data structures.

use std::collections::hashmap::HashMap;

pub type AttrMap = HashMap<String, String>;

#[deriving(Show)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

#[deriving(Show)]
pub enum NodeType {
    Element {
        local_name: String,
        attributes: AttrMap,
    },
    Text {
        data: String,
    },
}

// Constructor functions for convenience:

pub fn text(data: String) -> Node {
    Node::new(Text { data: data })
}

pub fn elem(name: String, attrs: AttrMap) -> Node {
    Node::new(Element {
        local_name: name,
        attributes: attrs,
    })
}

impl Node {
    fn new(node_type: NodeType) -> Node {
        Node {
            children: vec!(),
            node_type: node_type
        }
    }
}
