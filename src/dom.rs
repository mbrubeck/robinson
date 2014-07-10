//! Basic DOM data structures:

#[deriving(Show)]
pub enum Node {
    Element(ElementData),
    Text(TextData),
}

pub type NodeList = Vec<Box<Node>>;

#[deriving(Show)]
pub struct NodeData {
    pub children: Option<NodeList>,
}

#[deriving(Show)]
pub struct ElementData {
    pub node: NodeData,
    pub local_name: String,
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
              children: None
            },
            data: data,
        })
    }

    pub fn new_elem(local_name: String, children: NodeList) -> Node {
        Element(ElementData {
            node: NodeData {
              children: Some(children),
            },
            local_name: local_name,
        })
    }
}
