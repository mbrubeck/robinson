extern crate getopts;

use getopts::{optopt,getopts};
use std::os::args;
use std::io::fs::File;

mod css;
mod dom;
mod html;
mod layout;
mod style;

fn main() {
    // Parse command-line options:
    let opts = [
        optopt("h", "html", "HTML document", "FILENAME"),
        optopt("c", "css", "CSS stylesheet", "FILENAME"),
    ];
    let matches = match getopts(args().tail(), opts) {
        Ok(m) => m,
        Err(f) => fail!(f.to_string())
    };

    // Read input files:
    let read_source = |arg_filename: Option<String>, default_filename: &str| {
        let path = match arg_filename {
            Some(ref filename) => filename.as_slice(),
            None => default_filename,
        };
        File::open(&Path::new(path)).read_to_string().unwrap()
    };
    let html = read_source(matches.opt_str("h"), "examples/test.html");
    let css  = read_source(matches.opt_str("c"), "examples/test.css");

    // Parsing and rendering:
    let root_node = html::parse(html);
    let stylesheet = css::parse(css);
    let style_root = style::style_tree(&root_node, &stylesheet);
    let dimensions = layout::calculate_dimensions(&style_root);

    // Debug output:
    println!("{}\n", dimensions);
}
