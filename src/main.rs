extern crate debug;

mod dom;
mod html;

fn main() {
    let e = html::parse("<div>Hello <em>there</em></div>".to_string());
    println!("{}", e);
}
