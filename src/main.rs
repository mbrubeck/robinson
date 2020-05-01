#![feature(clamp)]
#![feature(new_uninit)]

use std::default::Default;
use std::io::{Read, BufWriter, BufRead};
use std::fs::File;
pub mod css;
pub mod dom;
pub mod html;
pub mod layout;
pub mod style;
pub mod painting;
pub mod pdf;
pub mod platform;
use platform::window::*;

fn main() {
    //-----------------------------------
    // Parse command-line options
    //-----------------------------------
    let mut opts = getopts::Options::new();
    opts.optopt("h", "html", "HTML document", "FILENAME");
    opts.optopt("c", "css", "CSS stylesheet", "FILENAME");
    opts.optopt("o", "output", "Output file", "FILENAME");
    opts.optopt("f", "format", "Output file format", "png | pdf");

    let matches = opts.parse(std::env::args().skip(1)).unwrap();
    let str_arg = |flag: &str, default: &str| -> String {
        matches.opt_str(flag).unwrap_or(default.to_string())
    };

    // Choose a format:
    let png = match &str_arg("f", "png")[..] {
        "png" => true,
        "pdf" => false,
        x => panic!("Unknown output format: {}", x),
    };
    let filename = str_arg("o", if png { "output.png" } else { "output.pdf" });

    //---------------------------------------------------------
    // Parse and Rendering
    //---------------------------------------------------------
    
    // Read input files:
    let html = read_source(str_arg("h", "examples/test.html"));
    let css  = read_source(str_arg("c", "examples/test.css"));

    // Since we don't have an actual window, hard-code the "viewport" size.
    let mut viewport: layout::Dimensions = Default::default();
    viewport.content.width  = 800.0;
    viewport.content.height = 600.0;

    // Parsing:
    let root_node = html::parse(html);
    let stylesheet = css::parse(css);
    let style_root = style::style_tree(&root_node, &stylesheet);
    let layout_root = layout::layout_tree(&style_root, viewport);
    // Rendering:
    let mut canvas = painting::Canvas::new(viewport.content.width as usize, viewport.content.height as usize, None);
    canvas = painting::paint(&layout_root, canvas);
    
    //----------------------------------------------------------
    // Showing to the screen
    //----------------------------------------------------------
    let window_res = create_window("main window", "HTML viewer", &canvas);
    let window = match window_res {
        Ok(wnd) => wnd,
        Err(e) => {
            println!("Couldn't open a window: {}", e);
            println!("Press 'Y' to render to a file (any key to exit): ");
            let mut string = std::io::stdin().lock().lines().next().unwrap().unwrap();
            string = string.to_lowercase();
            if string.pop().unwrap() == 'y' {
                save_to_file(&canvas, &filename.as_str(), png);
            }
            std::process::exit(-1)
        }
    };

    // main loop
    loop {
        if !window.handle_message() {
            break;
        }
    }

    //-----------------------
    // Save image to file
    //-----------------------
    save_to_file(&canvas, &filename.as_str(), png);
    
    println!("Window: {}x{}", window.width, window.height);
}

fn read_source(filename: String) -> String {
    let mut str = String::new();
    File::open(filename).unwrap().read_to_string(&mut str).unwrap();
    str
}

fn save_to_file(canvas: &painting::Canvas, filename: &str, is_png: bool) {
    let mut file = BufWriter::new(File::create(&filename).unwrap());
    
    // Write to file:
    let ok = if is_png {
        let (w, h) = (canvas.width as u32, canvas.height as u32);
        let img = image::ImageBuffer::from_fn(w, h, move |x, y| {
            let color = canvas.pixels[(y * w + x) as usize];
            image::Pixel::from_channels(color.r, color.g, color.b, color.a)
        });
        image::ImageRgba8(img).save(&mut file, image::PNG).is_ok()
    } else {
        println!("Error saving output as {}: format not supported!", filename);
        false
    };
    if ok {
        println!("Saved output as {}", filename)
    } else {
        println!("Error saving output as {}", filename)
    }
}
