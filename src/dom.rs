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
    Node { children: vec![], node_type: Text(data) }
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: Element(ElementData {
            tag_name: name,
            attributes: attrs,
        })
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

    pub fn classes<'a>(&'a self) -> HashSet<&'a str> {
        self.get_attribute("class").iter().flat_map(|classlist| {
            classlist.as_slice().split(' ')
        }).collect()
    }
}
