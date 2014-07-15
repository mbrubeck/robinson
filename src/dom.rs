//! Basic DOM data structures:

use std::collections::hashmap::HashMap;

#[deriving(Show)]
pub enum Node {
    Element(ElementData),
    Text(TextData),
}

pub type NodeList = Vec<Box<Node>>;

#[deriving(Show)]
pub struct NodeData {
    pub children: NodeList,
}

pub type AttrMap = HashMap<String, String>;

#[deriving(Show)]
pub struct ElementData {
    pub node: NodeData,
    pub local_name: String,
    pub attrs: AttrMap,
}

#[deriving(Show)]
pub struct TextData {
    pub node: NodeData,
    pub data: String,
}

// Constructor functions for convenience:

impl Node {
    pub fn new_text(data: String) -> Node {
        Text(TextData {
            node: NodeData {
              children: Vec::new()
            },
            data: data,
        })
    }

    pub fn new_elem(local_name: String, attrs: AttrMap, children: NodeList) -> Node {
        Element(ElementData {
            node: NodeData {
              children: children,
            },
            local_name: local_name,
            attrs: attrs,
        })
    }
}
