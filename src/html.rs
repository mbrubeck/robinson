use dom::Node;

pub fn parse(source: String) -> Node {
    let mut parser = Parser {
        pos: 0u,
        len: source.len(),
        input: source,
    };
    parser.parse_root()
}

struct Parser {
    pos: uint,
    len: uint,
    input: String,
}

impl Parser {
    fn parse_root(&mut self) -> Node {
        // TODO: Don't create a root <html> element if the input already contains one.
        Node::new_elem("html".to_string(), self.parse_nodes())
    }

    fn parse_nodes(&mut self) -> Vec<Box<Node>> {
        let mut nodes = Vec::new();
        loop {
            match self.parse_node() {
                Some(node) => nodes.push(box node),
                None => break
            }
        }
        nodes
    }

    fn parse_node(&mut self) -> Option<Node> {
        self.consume_whitespace();
        match (self.curr_char(), self.next_char()) {
            (Some('<'), Some('/')) => None, // Unexpected end tag. Stop parsing nodes.
            (Some('<'), Some(_))   => self.parse_elem(),
            (_, _)                 => self.parse_text()
        }
    }

    fn parse_elem(&mut self) -> Option<Node> {
        let name = self.parse_open_tag();
        let children = self.parse_nodes();
        self.consume_close_tag();
        Some(Node::new_elem(name, children))
    }

    fn parse_open_tag(&mut self) -> String {
        assert!(self.consume_char() == '<');
        let name = self.consume_tag_name();
        loop {
            match self.consume_char() {
                '>' => break,
                _   => continue, // TODO: Parse attributes
            }
        }
        name
    }

    fn consume_tag_name(&mut self) -> String {
        let mut name = String::new();
        loop {
            let c = self.curr_char();
            if c.is_none() {
                break;
            }
            match c.unwrap() {
                'a'..'z' | 'A'..'Z' | '0'..'9' => name.push_char(self.consume_char()),
                _ => break,
            }
        }
        name
    }

    fn consume_close_tag(&mut self) {
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        loop {
            match self.consume_char() {
                '>' => break,
                _   => continue,
            }
        }
    }

    fn parse_text(&mut self) -> Option<Node> {
        if self.eof() {
            return None;
        }
        let mut data = String::new();
        loop {
            match self.curr_char() {
                None      => break,
                Some('<') => break,
                _         => data.push_char(self.consume_char())
            }
        }
        Some(Node::new_text(data))
    }

    fn consume_whitespace(&mut self) {
        loop {
            match self.curr_char() {
                Some(c) if c.is_whitespace() => self.consume_char(),
                _                            => break,
            };
        }
    }

    fn consume_char(&mut self) -> char {
        let range = self.input.as_slice().char_range_at(self.pos);
        self.pos = range.next;
        range.ch
    }

    fn curr_char(&self) -> Option<char> { self.char_at(self.pos)     }
    fn next_char(&self) -> Option<char> { self.char_at(self.pos + 1) }

    fn char_at(&self, i: uint) -> Option<char> {
        if i < self.len {
            Some(self.input.as_slice().char_at(i))
        } else {
            None
        }
    }

    fn eof(&self) -> bool { self.pos >= self.len }
}
