mod css;
mod dom;
mod html;
mod style;

fn main() {
    let e = html::parse("<div id='a' class='b'>Hello <em>there</em></div>".to_string());
    let c = css::parse("div, *, span#foo.bar { display: block; height: 1px; }".to_string());
    println!("{}", e);
    println!("{}", c);
}
