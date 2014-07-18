use dom::{ElementData};
use css::{Selector, Simple, SimpleSelector};

fn matches_selector(elem: ElementData, selector: Selector) -> bool {
    match selector {
        Simple(simple_selector) => matches_simple_selector(elem, simple_selector)
    }
}

fn matches_simple_selector(elem: ElementData, selector: SimpleSelector) -> bool {
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
