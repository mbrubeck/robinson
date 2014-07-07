pub enum Node {
    Element(ElementData),
    Text(TextData),
}

pub type NodeList = Option<Vec<Box<Node>>>;

pub struct NodeData {
    pub children: NodeList,
}

pub struct ElementData {
    pub node: NodeData,
    pub local_name: String,
}

pub struct TextData {
    pub node: NodeData,
    pub data: String,
}


// Constructor functions

pub fn text(data: &'static str) -> Node {
    Text(TextData {
        node: NodeData {
          children: None
        },
        data: data.to_string(),
    })
}

pub fn elem(local_name: &'static str, children: NodeList) -> Node {
    Element(ElementData {
        node: NodeData {
          children: children,
        },
        local_name: local_name.to_string(),
    })
}
