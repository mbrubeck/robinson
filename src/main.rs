extern crate getopts;
extern crate image;

use getopts::{optopt,getopts};
use std::default::Default;
use std::io::fs::File;
use std::os::args;

mod css;
mod dom;
mod html;
mod layout;
mod style;
mod painting;

#[allow(unstable)]
fn main() {
    // Parse command-line options:
    let opts = [
        optopt("h", "html", "HTML document", "FILENAME"),
        optopt("c", "css", "CSS stylesheet", "FILENAME"),
        optopt("o", "output", "Output file", "FILENAME"),
    ];
    let matches = match getopts(args().tail(), &opts) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string())
    };

    // Read input files:
    let read_source = |&: arg_filename: Option<String>, default_filename: &str| {
        let path = match arg_filename {
            Some(ref filename) => &**filename,
            None => default_filename,
        };
        File::open(&Path::new(path)).read_to_string().unwrap()
    };
    let html = read_source(matches.opt_str("h"), "examples/test.html");
    let css  = read_source(matches.opt_str("c"), "examples/test.css");

    // Since we don't have an actual window, hard-code the "viewport" size.
    let initial_containing_block = layout::Dimensions {
        content: layout::Rect { x: 0.0, y: 0.0, width: 800.0, height: 600.0 },
        padding: Default::default(),
        border: Default::default(),
        margin: Default::default(),
    };

    // Parsing and rendering:
    let root_node = html::parse(html);
    let stylesheet = css::parse(css);
    let style_root = style::style_tree(&root_node, &stylesheet);
    let layout_root = layout::layout_tree(&style_root, initial_containing_block);
    let canvas = painting::paint(&layout_root, initial_containing_block.content);

    // Create the output file:
    let filename = matches.opt_str("o").unwrap_or("output.png".to_string());
    let file = File::create(&Path::new(&*filename)).unwrap();

    // Save an image:
    let (w, h) = (canvas.width as u32, canvas.height as u32);
    let buffer: Vec<image::Rgba<u8>> = unsafe { std::mem::transmute(canvas.pixels) };
    let img = image::ImageBuffer::from_fn(w, h, Box::new(|&: x: u32, y: u32| buffer[(y * w + x) as usize]));

    let result = image::ImageRgba8(img).save(file, image::PNG);
    match result {
        Ok(_) => println!("Saved output as {}", filename),
        Err(_) => println!("Error saving output as {}", filename)
    }

    // Debug output:
    // println!("{}", layout_root.dimensions);
    // println!("{}", display_list);
}
