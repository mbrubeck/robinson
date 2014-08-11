//! Code for applying CSS styles to the DOM.
//!
//! This is not very interesting at the moment.  It will get much more
//! complicated if I add support for compound selectors.

use dom::{Node, Element, ElementData};
use css::{Stylesheet, Rule, Selector, Simple, SimpleSelector, Value};
use std::collections::hashmap::HashMap;

/// A node with associated style data.
pub struct StyledNode<'a> {
    pub node: &'a Node,
    pub specified_values: PropertyMap,
    pub children: Vec<StyledNode<'a>>,
}

impl<'a> StyledNode<'a> {
    pub fn value(&self, name: &str) -> Option<Value> {
        self.specified_values.find_equiv(&name).map(|v| v.clone())
    }

    pub fn lookup(&self, name: &str, fallback_name: &str, default: &Value) -> Value {
        self.value(name).unwrap_or_else(|| self.value(fallback_name).unwrap_or_else(|| default.clone()))
    }
}

/// Map from CSS property names to values.
pub type PropertyMap =  HashMap<String, Value>;

/// Apply a stylesheet to an entire DOM tree, returning a StyledNode tree.
///
/// This finds only the specified values at the moment. Eventually it should be extended to find the
/// computed values too, including inherited values.
pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
    StyledNode {
        node: root,
        specified_values: match root.node_type {
            Element(ref elem) => specified_values(elem, stylesheet),
            _ => HashMap::new(),
        },
        children: root.children.iter().map(|child| style_tree(child, stylesheet)).collect(),
    }
}

/// Apply styles to a single element, returning the specified styles.
///
/// To do: Allow multiple UA/author/user stylesheets, and implement the cascade.
fn specified_values(elem: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    // Go through the rules from lowest to highest specificity.
    rules.sort_by(|&(a, _), &(b, _)| a.specificity().cmp(&b.specificity()));
    for &(_, rule) in rules.iter() {
        for declaration in rule.declarations.iter() {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }
    values
}

/// A single CSS rule and the highest-specificity selector that resulted in a given match.
type MatchedRule<'a> = (&'a Selector, &'a Rule);

/// Find all CSS rules that match the given element.
fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    // For now, we just do a linear scan of all the rules.  For large
    // documents, it would be more efficient to store the rules in hash tables
    // based on tag name, id, class, etc.
    stylesheet.rules.iter()
        .filter_map(|rule| {
            // Find the first (highest-specificity) matching selector.
            rule.selectors.iter().find(|selector| matches(elem, *selector))
                .map(|selector| (selector, rule))
        }).collect()
}

/// Selector matching:
fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match *selector {
        Simple(ref simple_selector) => matches_simple_selector(elem, simple_selector)
    }
}

fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
    // Check type selector
    if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
        return false;
    }

    // Check ID selector
    if selector.id.iter().any(|id| {
        elem.id().iter().any(|elem_id| {
            *elem_id != id
    })}) {
        return false;
    }

    // Check class selectors
    let elem_classes = elem.classes();
    if selector.class.iter().any(|class| !elem_classes.contains(&class.as_slice())) {
        return false;
    }

    // We didn't find any non-matching selector components.
    return true;
}
