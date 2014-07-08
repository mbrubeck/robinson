use std::fmt;

pub enum Node {
    Element(ElementData),
    Text(TextData),
}

pub type NodeList = Vec<Box<Node>>;

pub struct NodeData {
    pub children: Option<NodeList>,
}

pub struct ElementData {
    pub node: NodeData,
    pub local_name: String,
}

pub struct TextData {
    pub node: NodeData,
    pub data: String,
}

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

impl fmt::Show for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Text(ref text) => {
                write!(f, "{}", text.data)
            }
            Element(ref elem) => {
                try!(write!(f, "<{}>", elem.local_name));
                match elem.node.children {
                    Some(ref children) => {
                        for child in children.iter() {
                            try!(write!(f, "{}", *child));
                        }
                    }
                    None => {}
                }
                write!(f, "</{}>", elem.local_name)
            }
        }
    }
}
