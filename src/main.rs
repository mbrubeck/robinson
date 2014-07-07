extern crate debug;

mod dom;

fn main() {
    let t = box dom::text("Hello");
    let e = box dom::elem("div", Some(vec!(t)));
    println!("{:?}", e);
}
