mod css;
mod dom;
mod html;
mod style;

fn main() {
    let root_node = html::parse("<div id='a' class='b'>Hello <em>there</em></div>".to_string());
    println!("{}\n", root_node);

    let stylesheet = css::parse("div, *, span#foo.bar { display: block; height: 1px; }".to_string());
    println!("{}\n", stylesheet);

    let declarations = match root_node.node_type {
        dom::Element(ref elem) => style::specified_values(elem, &stylesheet),
        _ => fail!("not an element")
    };
    println!("{}\n", declarations);
}
