extern crate debug;

mod dom;
mod html;
mod css;

fn main() {
    let e = html::parse("<div>Hello <em>there</em></div>".to_string());
    let c = css::parse("div { display: block; height: 1px; }".to_string());
    println!("{}", e);
    println!("{}", c);
}
