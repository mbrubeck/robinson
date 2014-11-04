use layout::{AnonymousBlock, BlockNode, InlineNode, LayoutBox, Rect};
use css::{ColorValue, Color};
use std::iter::range;
use std::cmp::{max, min};

#[deriving(Show)]
enum DisplayItem {
    SolidColor(Rect, Color),
}

type DisplayList = Vec<DisplayItem>;

pub fn build_display_list(layout_root: &LayoutBox) -> DisplayList {
    let mut list = Vec::new();
    render_layout_box(&mut list, layout_root);
    return list;
}

fn render_layout_box(list: &mut DisplayList, layout_box: &LayoutBox) {
    render_background(list, layout_box);
    // TODO: render borders
    // TODO: render text
    for child in layout_box.children.iter() {
        render_layout_box(list, child);
    }
}

fn render_background(list: &mut DisplayList, layout_box: &LayoutBox) {
    let transparent = Color { r: 0, g: 0, b: 0, a: 0 };
    let background_style = match layout_box.box_type {
        BlockNode(style) | InlineNode(style) => {
            Some(style.lookup("background-color", "background", &ColorValue(transparent)))
        }
        AnonymousBlock => None
    };

    match background_style {
        Some(ColorValue(color)) if color != transparent => {
            list.push(SolidColor(layout_box.dimensions.padding_box(), color));
        }
        _ => {} // other values not supported yet
    }
}

pub struct Canvas {
    pub pixels: Vec<Color>,
    pub width: uint,
    pub height: uint,
}

impl Canvas {
    fn new(width: uint, height: uint) -> Canvas {
        let white = Color { r: 255, g: 255, b: 255, a: 255 };
        return Canvas {
            pixels: Vec::from_elem(width * height, white),
            width: width,
            height: height,
        }
    }
}

pub fn paint(list: &DisplayList, bounds: Rect) -> Canvas {
    let mut canvas = Canvas::new(bounds.width as uint, bounds.height as uint);
    for item in list.iter() {
        item.paint(&mut canvas);
    }
    return canvas;
}

impl DisplayItem {
    fn paint(&self, canvas: &mut Canvas) {
        match self {
            &SolidColor(rect, color) => {
                let x0 = max(0, rect.x as uint);
                let y0 = max(0, rect.y as uint);
                let x1 = min(canvas.width, (rect.x + rect.width) as uint);
                let y1 = min(canvas.height, (rect.y + rect.height) as uint);

                for x in range(x0, x1) {
                    for y in range(y0, y1) {
                        // TODO: alpha compositing with existing pixel
                        canvas.pixels[x + y * canvas.width] = color;
                    }
                }
            }
        }
    }
}
