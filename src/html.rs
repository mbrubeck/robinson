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

use dom;
use std::collections::hashmap::HashMap;

pub fn parse(source: String) -> dom::Node {
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
    fn parse_root(&mut self) -> dom::Node {
        // TODO: Don't create a root <html> element if the input already contains one.
        let mut root = dom::elem("html".to_string(), HashMap::new());
        self.parse_nodes(&mut root);
        root
    }

    /// Parse a sequence of sibling nodes.
    fn parse_nodes(&mut self, parent: &mut dom::Node) {
        loop {
            match self.parse_node() {
                Some(node) => parent.children.push(node),
                None => break
            }
        }
    }

    /// Parse a single node. Returns None if there is no node at the current position.
    fn parse_node(&mut self) -> Option<dom::Node> {
        self.consume_whitespace();
        match (self.curr_char(), self.next_char()) {
            (Some('<'), Some('/')) => None, // Unexpected end tag. Stop parsing nodes.
            (Some('<'), Some(_))   => self.parse_element(),
            (_, _)                 => self.parse_text()
        }
    }

    /// Parse a single element, including its open tag, contents, and closing tag.
    fn parse_element(&mut self) -> Option<dom::Node> {
        let mut elem = self.parse_open_tag();
        self.parse_nodes(&mut elem);
        self.consume_close_tag();
        Some(elem)
    }

    // Helper functions for parse_element:

    fn parse_open_tag(&mut self) -> dom::Node {
        assert!(self.consume_char() == '<');
        let name = self.parse_tag_name();
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            match self.curr_char().unwrap() {
                '>' => {
                    self.consume_char();
                    break;
                }
                _   => {
                    let (name, value) = self.parse_attr();
                    attributes.insert(name, value);
                }
            }
        }
        dom::elem(name, attributes)
    }

    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        self.consume_whitespace();
        assert!(self.consume_char() == '=');
        self.consume_whitespace();
        let value = self.parse_attr_value();
        (name, value)
    }

    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let mut value = String::new();
        loop {
            let c = self.consume_char();
            if c == open_quote {
                break;
            }
            value.push_char(c);
        }
        value
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
    fn parse_text(&mut self) -> Option<dom::Node> {
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
        Some(dom::text(data))
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
