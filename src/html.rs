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
        input: source,
    };
    parser.parse_root()
}

struct Parser {
    pos: uint,
    input: String,
}

impl Parser {
    /// Parse a document and return the root element.
    fn parse_root(&mut self) -> dom::Node {
        let mut nodes = vec!();
        self.parse_nodes(&mut nodes);

        // If the document contains a root `<html>` element, just return it.
        let has_root = nodes.len() == 1 && match nodes[0].node_type {
            dom::Element(ref elem) if elem.tag_name.as_slice() == "html" => true,
            _ => false
        };
        if has_root {
            nodes.swap_remove(0).unwrap()
        } else {
            // If the root `<html>` element is missing, create one.
            let mut root = dom::elem("html".to_string(), HashMap::new());
            root.children.push_all_move(nodes);
            root
        }
    }

    /// Parse a sequence of sibling nodes.
    fn parse_nodes(&mut self, nodes: &mut Vec<dom::Node>) {
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
    }

    /// Parse a single node. Returns None if there is no node at the current position.
    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _   => self.parse_text()
        }
    }

    /// Parse a single element, including its open tag, contents, and closing tag.
    fn parse_element(&mut self) -> dom::Node {
        let mut elem = self.parse_open_tag();
        self.parse_nodes(&mut elem.children);
        self.consume_close_tag();
        elem
    }

    // Helper functions for parse_element:

    fn parse_open_tag(&mut self) -> dom::Node {
        assert!(self.consume_char() == '<');
        let name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert!(self.consume_char() == '>');

        dom::elem(name, attrs)
    }

    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..'z' | 'A'..'Z' | '0'..'9' => true,
            _ => false
        })
    }

    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            match self.next_char() {
                '>' => {
                    break;
                }
                _ => {
                    let (name, value) = self.parse_attr();
                    attributes.insert(name, value);
                }
            }
        }
        attributes
    }

    // name="value"
    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert!(self.consume_char() == '=');
        let value = self.parse_attr_value();
        (name, value)
    }

    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let mut value = self.consume_while(|c| c != open_quote);
        assert!(!self.eof() && self.consume_char() == open_quote);
        value
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
    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|c| c != '<'))
    }

    /// Consume and discard zero or more whitespace characters.
    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    /// Consume characters until `test` returns false.
    fn consume_while(&mut self, test: |char| -> bool) -> String {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push_char(self.consume_char());
        }
        result
    }

    /// Return the current character, and advance self.pos to the next character.
    fn consume_char(&mut self) -> char {
        let range = self.input.as_slice().char_range_at(self.pos);
        self.pos = range.next;
        range.ch
    }

    /// Read the current character without consuming it.
    fn next_char(&self) -> char {
        self.input.as_slice().char_at(self.pos)
    }

    /// Does the current input start with the given string?
    fn starts_with(&self, s: &str) -> bool {
        self.input.as_slice().slice_from(self.pos).starts_with(s)
    }

    /// Return true if all input is consumed.
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}
