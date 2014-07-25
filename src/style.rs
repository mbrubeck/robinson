//! Code for applying CSS styles to the DOM.
//!
//! This is not very interesting at the moment.  It will get much more
//! complicated if I add support for compound selectors.

use dom::{ElementData};
use css::{Stylesheet, Rule, Selector, Simple, SimpleSelector, Value};
use std::collections::hashmap::HashMap;

pub type PropertyMap<'a> =  HashMap<&'a String, &'a Value>;

pub fn specified_values<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> PropertyMap<'a> {
    let mut values = HashMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    // Go through the rules from lowest to highest specificity.
    rules.sort_by(|&(a, _), &(b, _)| a.specificity().cmp(&b.specificity()));
    for &(_, rule) in rules.iter() {
        for declaration in rule.declarations.iter() {
            values.insert(&declaration.name, &declaration.value);
        }
    }
    values
}

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

fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match *selector {
        Simple(ref simple_selector) => matches_simple_selector(elem, simple_selector)
    }
}

fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
    // Check type selector
    if selector.local_name.iter().any(|name| elem.local_name != *name) {
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
    if selector.class.iter().any(|class| !elem_classes.contains(class)) {
        return false;
    }

    // We didn't find any non-matching selector components.
    return true;
}
