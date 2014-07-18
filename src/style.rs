//! Code for applying CSS styles to the DOM.
//!
//! This is not very interesting at the moment.  It will get much more
//! complicated if I add support for compound selectors.

use dom::{ElementData};
use css::{Stylesheet, Rule, Selector, Simple, SimpleSelector};

/// Find all CSS rules that match the given element.
pub fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> Vec<&'a Rule> {
    // For now, we just do a linear scan of all the rules.  For large
    // documents, it would be more efficient to store the rules in hash tables
    // based on tag name, id, class, etc.
    stylesheet.rules.iter()
        .filter(|rule| rule.selectors.iter().any(|s| matches_selector(elem, s)))
        .collect()
}

fn matches_selector(elem: &ElementData, selector: &Selector) -> bool {
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
