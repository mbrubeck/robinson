mod css;
mod dom;
mod html;
mod style;

fn main() {
    let root_node = html::parse("<div id='a' class='b'>Hello <em>there</em></div>".to_string());
    let stylesheet = css::parse("div, *, span#foo.bar { display: block; height: 1px; }".to_string());
    let declarations = match root_node.node_type {
        dom::Element(ref elem) => style::matching_rules(elem, &stylesheet),
        _ => fail!("not an element")
    };
    println!("{}", root_node);
    println!("{}", stylesheet);
    println!("{}", declarations);
}
