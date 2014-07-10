//! A simple parser for a tiny subset of HTML.
//!
//! Can parse basic opening and closing tags, and text nodes.
//!
//! Not yet supported:
//!
//! * Attributes
//! * Comments
//! * Doctypes and processing instructions
//! * Self-closing tags
//! * Non-well-formed markup
//! * Character entities

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
    /// Parse a document and return the root element.
    fn parse_root(&mut self) -> Node {
        // TODO: Don't create a root <html> element if the input already contains one.
        Node::new_elem("html".to_string(), self.parse_nodes())
    }

    /// Parse a sequence of sibling nodes.
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

    /// Parse a single node. Returns None if there is no node at the current position.
    fn parse_node(&mut self) -> Option<Node> {
        self.consume_whitespace();
        match (self.curr_char(), self.next_char()) {
            (Some('<'), Some('/')) => None, // Unexpected end tag. Stop parsing nodes.
            (Some('<'), Some(_))   => self.parse_element(),
            (_, _)                 => self.parse_text()
        }
    }

    /// Parse a single element, including its open tag, contents, and closing tag.
    fn parse_element(&mut self) -> Option<Node> {
        let name = self.parse_open_tag();
        let children = self.parse_nodes();
        self.consume_close_tag();
        Some(Node::new_elem(name, children))
    }

    // Helper functions for parse_element:

    fn parse_open_tag(&mut self) -> String {
        assert!(self.consume_char() == '<');
        let name = self.parse_tag_name();
        loop {
            match self.consume_char() {
                '>' => break,
                _   => continue, // TODO: Parse attributes
            }
        }
        name
    }

    fn parse_tag_name(&mut self) -> String {
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

    /// Parse a text node.
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

    /// Consume and discard zero or more whitespace characters.
    fn consume_whitespace(&mut self) {
        loop {
            match self.curr_char() {
                Some(c) if c.is_whitespace() => self.consume_char(),
                _ => break,
            };
        }
    }

    /// Return the current character, and advance self.pos to the next character.
    fn consume_char(&mut self) -> char {
        let range = self.input.as_slice().char_range_at(self.pos);
        self.pos = range.next;
        range.ch
    }

    /// Read a character without consuming it.
    fn char_at(&self, i: uint) -> Option<char> {
        if i < self.len {
            Some(self.input.as_slice().char_at(i))
        } else {
            None
        }
    }

    // Convenience functions for reading ahead 0 or 1 characters.
    fn curr_char(&self) -> Option<char> { self.char_at(self.pos)     }
    fn next_char(&self) -> Option<char> { self.char_at(self.pos + 1) }

    /// Return true if all input is consumed.
    fn eof(&self) -> bool {
        self.pos >= self.len
    }
}
