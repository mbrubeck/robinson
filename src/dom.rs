//! Basic DOM data structures.

use std::collections::hashmap::{HashMap, HashSet};

pub type AttrMap = HashMap<String, String>;

#[deriving(Show)]
pub struct Node {
    // data common to all nodes:
    pub children: Vec<Node>,

    // data specific to each node type:
    pub node_type: NodeType,
}

#[deriving(Show)]
pub enum NodeType {
    Element(ElementData),
    Text(String),
}

#[deriving(Show)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

// Constructor functions for convenience:

pub fn text(data: String) -> Node {
    Node::new(Text(data))
}

pub fn elem(name: String, attrs: AttrMap) -> Node {
    Node::new(Element(ElementData {
        tag_name: name,
        attributes: attrs,
    }))
}

// Node methods

impl Node {
    fn new(node_type: NodeType) -> Node {
        Node {
            children: vec!(),
            node_type: node_type
        }
    }
}

// Element methods

impl ElementData {
    pub fn get_attribute<'a>(&'a self, key: &str) -> Option<&'a String> {
        self.attributes.find_equiv(&key)
    }

    pub fn id<'a>(&'a self) -> Option<&'a String> {
        self.get_attribute("id")
    }

    pub fn classes(&self) -> HashSet<String> {
        self.get_attribute("class").iter().flat_map(|classlist| {
            classlist.as_slice().split(' ').map(|s| s.to_string())
        }).collect()
    }
}
