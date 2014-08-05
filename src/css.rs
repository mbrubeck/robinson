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
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

#[deriving(Show)]
pub struct Declaration {
    pub name: String,
    pub value: Value,
}

#[deriving(Show, Clone, PartialEq)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
}

#[deriving(Show, Clone, PartialEq)]
pub enum Unit {
    Px,
}

pub type Specificity = (uint, uint, uint);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        // http://www.w3.org/TR/selectors/#specificity
        let Simple(ref simple) = *self;
        let a = simple.id.iter().len();
        let b = simple.class.len();
        let c = simple.tag_name.iter().len();
        (a, b, c)
    }
}

// Parsing:

pub fn parse(source: String) -> Stylesheet {
    let mut parser = Parser {
        pos: 0u,
        input: source,
    };
    parser.parse_stylesheet()
}

struct Parser {
    pos: uint,
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
            self.consume_whitespace();
            selectors.push(Simple(self.parse_simple_selector()));
            self.consume_whitespace();
            match self.curr_char() {
                ',' => { self.consume_char(); }
                '{' => break,
                c   => fail!("Unexpected character {} in selector list", c),
            }
        }
        // Return selectors with highest specificity first, for use in matching.
        selectors.sort_by(|a,b| b.specificity().cmp(&a.specificity()));
        selectors
    }

    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut result = SimpleSelector { tag_name: None, id: None, class: Vec::new() };
        while !self.eof() {
            match self.curr_char() {
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
                    result.tag_name = Some(self.parse_identifier());
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
            if self.curr_char() == '}' {
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
        match self.curr_char() {
            '0'..'9' => self.parse_length(),
            _ => Keyword(self.parse_identifier())
        }
    }

    fn parse_length(&mut self) -> Value {
        Length(self.parse_float(), self.parse_unit())
    }

    fn parse_identifier(&mut self) -> String {
        self.consume_while(valid_identifier_char)
    }

    fn parse_float(&mut self) -> f32 {
        let mut s = self.consume_while(|c| match c {
            '0'..'9' | '.' => true,
            _ => false
        });
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
        self.consume_while(|c| c.is_whitespace());
    }

    /// Consume characters until `test` returns false.
    fn consume_while(&mut self, test: |char| -> bool) -> String {
        let mut result = String::new();
        while !self.eof() && test(self.curr_char()) {
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
    fn curr_char(&self) -> char {
        self.input.as_slice().char_at(self.pos)
    }

    /// Return true if all input is consumed.
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}

fn valid_identifier_char(c: char) -> bool {
    match c {
        'a'..'z' | 'A'..'Z' | '0'..'9' | '-' | '_' => true, // TODO: Include U+00A0 and higher.
        _ => false,
    }
}
