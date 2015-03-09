#![feature(core, std_misc, collections)]

extern crate getopts;
extern crate image;

use std::default::Default;
use std::io::Read;
use std::fs::File;

mod css;
mod dom;
mod html;
mod layout;
mod style;
mod painting;
mod pdf;

fn main() {
    // Parse command-line options:
    let mut opts = getopts::Options::new();
    opts.optopt("h", "html", "HTML document", "FILENAME");
    opts.optopt("c", "css", "CSS stylesheet", "FILENAME");
    opts.optopt("o", "output", "Output file", "FILENAME");
    opts.optopt("f", "format", "Output file format", "png | pdf");

    let matches = match opts.parse(std::env::args().skip(1)) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string())
    };

    let png = match matches.opt_str("f") {
        None => true,
        Some(format) => match &*format {
            "png" => true,
            "pdf" => false,
            _ => panic!("Unknown output format: {}", format),
        }
    };

    // Read input files:
    let read_source = |arg_filename: Option<String>, default_filename: &str| {
        let path = match arg_filename {
            Some(ref filename) => &**filename,
            None => default_filename,
        };
        let mut str = String::new();
        File::open(&Path::new(path)).unwrap().read_to_string(&mut str);
        str
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

    // Create the output file:
    let default_filename = if png { "output.png" } else { "output.pdf" };
    let filename = matches.opt_str("o").unwrap_or(default_filename.to_string());
    let mut file = File::create(&Path::new(&*filename)).unwrap();

    let result_ok;
    if png {
        let canvas = painting::paint(&layout_root, initial_containing_block.content);

        // Save an image:
        let (w, h) = (canvas.width as u32, canvas.height as u32);
        let img = image::ImageBuffer::from_fn(w, h, move |x, y| {
            let color = canvas.pixels[(y * w + x) as usize];
            image::Pixel::from_channels(color.r, color.g, color.b, color.a)
        });

        result_ok = image::ImageRgba8(img).save(&mut file, image::PNG).is_ok();
    } else {
        result_ok = pdf::render(&layout_root, initial_containing_block.content, &mut file).is_ok();
    }

    if result_ok {
        println!("Saved output as {}", filename)
    } else {
        println!("Error saving output as {}", filename)
    }
}
