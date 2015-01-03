extern crate getopts;
extern crate image;

use getopts::{optopt,getopts};
use std::default::Default;
use std::io::fs::File;
use std::os::args;
use dom::{Element, Text};

mod css;
mod dom;
mod html;
mod layout;
mod style;
mod painting;

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
    fn read_source(arg_filename: Option<&String>) -> Option<String> {
        let path = match arg_filename {
            Some(ref filename) => filename.as_slice(),
            None => return None,
        };
        File::open(&Path::new(path)).read_to_string().ok()
    }

    let html = match read_source(matches.opt_str("h").as_ref()) {
        Some(s) => s,
        None => {
            println!("You have to give the html file!")
            return
        }
    };

    // Since we don't have an actual window, hard-code the "viewport" size.
    let initial_containing_block = layout::Dimensions {
        content: layout::Rect { x: 0.0, y: 0.0, width: 800.0, height: 600.0 },
        padding: Default::default(),
        border: Default::default(),
        margin: Default::default(),
    };

    // Parsing and rendering:
    let root_node = html::parse(html);

    // Find css links.
    let links = root_node.get_elements_by_tag_name("link".as_slice());
    let css = links.iter().map(|link| {
        let href = match link.node_type {
            Element(ref element_data) => element_data.get_attribute("href"),
            Text(_) => panic!("Method get_elements_by_tag_name shouldn't return Text node, ever."),
        };
        match read_source(href) {
            Some(s) => s,
            None => {
                println!("Stylesheet {} not found.", href);
                "".to_string()
            },
        }
    }).fold("".to_string(), |acc, s| acc+s);

    let stylesheet = css::parse(css);
    let style_root = style::style_tree(&root_node, &stylesheet);
    let layout_root = layout::layout_tree(&style_root, initial_containing_block);
    let canvas = painting::paint(&layout_root, initial_containing_block.content);

    // Create the output file:
    let filename = matches.opt_str("o").unwrap_or("output.png".to_string());
    let file = File::create(&Path::new(filename.as_slice())).unwrap();

    // Save an image:
    let (w, h) = (canvas.width as u32, canvas.height as u32);
    let buffer: Vec<image::Rgba<u8>> = unsafe { std::mem::transmute(canvas.pixels) };
    let img = image::ImageBuffer::from_fn(w, h, |x, y| buffer[(y * w + x) as uint]);

    let result = image::ImageRgba8(img).save(file, image::PNG);
    match result {
        Ok(_) => println!("Saved output as {}", filename),
        Err(_) => println!("Error saving output as {}", filename)
    }

    // Debug output:
    // println!("{}", layout_root.dimensions);
    // println!("{}", display_list);
}
