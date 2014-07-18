//! A simple parser for a tiny subset of CSS.
//!
//! To support more CSS syntax, it would probably be easiest to replace this
//! hand-rolled parser with one based on a library or parser generator.

use std::from_str::FromStr;
use std::ascii::OwnedStrAsciiExt;

// Data structures:

#[deriving(Show)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

#[deriving(Show)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[deriving(Show)]
pub enum Selector {
    Simple(SimpleSelector),
}

#[deriving(Show)]
pub struct SimpleSelector {
    pub local_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

#[deriving(Show)]
pub struct Declaration {
    pub name: String,
    pub value: Value,
}

#[deriving(Show)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
}

#[deriving(Show)]
pub enum Unit {
    Px,
}

// Parsing:

pub fn parse(source: String) -> Stylesheet {
    let mut parser = Parser {
        pos: 0u,
        len: source.len(),
        input: source,
    };
    parser.parse_stylesheet()
}

struct Parser {
    pos: uint,
    len: uint,
    input: String,
}

impl Parser {
    fn parse_stylesheet(&mut self) -> Stylesheet {
        let mut rules = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() {
                break;
            }
            rules.push(self.parse_rule());
        }
        Stylesheet { rules: rules }
    }

    fn parse_rule(&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations(),
        }
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        loop {
            selectors.push(Simple(self.parse_simple_selector()));
            self.consume_whitespace();
            match self.curr_char() {
                Some(',') => {
                    self.consume_char();
                    self.consume_whitespace();
                    continue;
                }
                Some('{') => break,
                _ => fail!("Unexpected end of selector list")
            }
        }
        selectors
    }

    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut result = SimpleSelector { local_name: None, id: None, class: Vec::new() };
        loop {
            match self.curr_char().unwrap() {
                '#' => {
                    self.consume_char();
                    result.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.consume_char();
                    result.class.push(self.parse_identifier());
                }
                '*' => {
                    // universal selector
                    self.consume_char();
                }
                c if valid_identifier_char(c) => {
                    result.local_name = Some(self.parse_identifier());
                }
                _ => break
            }
        }
        result
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        self.consume_whitespace();
        assert!(self.consume_char() == '{');
        let mut declarations = Vec::new();
        loop {
            self.consume_whitespace();
            if self.curr_char() == Some('}') {
                self.consume_char();
                break;
            }
            declarations.push(self.parse_declaration());
        }
        declarations
    }

    fn parse_declaration(&mut self) -> Declaration {
        let property_name = self.parse_identifier();
        self.consume_whitespace();
        assert!(self.consume_char() == ':')
        self.consume_whitespace();
        let value = self.parse_value();
        self.consume_whitespace();
        assert!(self.consume_char() == ';')

        Declaration {
            name: property_name,
            value: value,
        }
    }

    fn parse_value(&mut self) -> Value {
        match self.curr_char().unwrap() {
            '0'..'9' => self.parse_length(),
            _ => Keyword(self.parse_identifier())
        }
    }

    fn parse_length(&mut self) -> Value {
        Length(self.parse_float(), self.parse_unit())
    }

    fn parse_identifier(&mut self) -> String {
        let mut name = String::new();
        // TODO: Identifiers must not start with a digit or "--" or "-" and a digit.
        loop {
            let c = self.curr_char();
            if c.is_none() {
                break;
            }
            let c = c.unwrap();
            if !valid_identifier_char(c) {
                break;
            }
            name.push_char(self.consume_char());
        }
        name
    }

    fn parse_float(&mut self) -> f32 {
        let mut s = String::new();
        loop {
            match self.curr_char().unwrap() {
                '0'..'9' | '.' => s.push_char(self.consume_char()),
                _ => break
            }
        }
        let f: Option<f32> = FromStr::from_str(s.as_slice());
        f.unwrap()
    }

    fn parse_unit(&mut self) -> Unit {
        match self.parse_identifier().into_ascii_lower().as_slice() {
            "px" => Px,
            _ => fail!("unrecognized unit")
        }
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

    fn curr_char(&self) -> Option<char> { self.char_at(self.pos)     }

    /// Return true if all input is consumed.
    fn eof(&self) -> bool {
        self.pos >= self.len
    }
}

fn valid_identifier_char(c: char) -> bool {
    match c {
        'a'..'z' | 'A'..'Z' | '0'..'9' | '-' | '_' => true, // TODO: Include U+00A0 and higher.
        _ => false,
    }
}
