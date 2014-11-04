use layout::{AnonymousBlock, BlockNode, InlineNode, LayoutBox, Rect};
use css::{ColorValue, Color};
use std::iter::range;
use std::cmp::{max, min};

#[deriving(Show)]
enum DisplayItem {
    SolidColor(Color, Rect),
}

type DisplayList = Vec<DisplayItem>;

pub fn build_display_list(layout_root: &LayoutBox) -> DisplayList {
    let mut list = Vec::new();
    render_layout_box(&mut list, layout_root);
    return list;
}

fn render_layout_box(list: &mut DisplayList, layout_box: &LayoutBox) {
    render_background(list, layout_box);
    render_borders(list, layout_box);
    for child in layout_box.children.iter() {
        render_layout_box(list, child);
    }
}

fn render_background(list: &mut DisplayList, layout_box: &LayoutBox) {
    get_color(layout_box, "background").map(|color|
        list.push(SolidColor(color, layout_box.dimensions.padding_box())));
}

fn render_borders(list: &mut DisplayList, layout_box: &LayoutBox) {
    let color = match get_color(layout_box, "border-color") {
        Some(color) => color,
        _ => return
    };

    let d = &layout_box.dimensions;
    let border_box = d.border_box();

    // Left border
    list.push(SolidColor(color, Rect {
        x: border_box.x,
        y: border_box.y,
        width: d.border.left,
        height: border_box.height,
    }));

    // Right border
    list.push(SolidColor(color, Rect {
        x: border_box.x + border_box.width - d.border.right,
        y: border_box.y,
        width: d.border.right,
        height: border_box.height,
    }));

    // Top border
    list.push(SolidColor(color, Rect {
        x: border_box.x,
        y: border_box.y,
        width: border_box.width,
        height: d.border.top,
    }));

    // Bottom border
    list.push(SolidColor(color, Rect {
        x: border_box.x,
        y: border_box.y + border_box.height - d.border.bottom,
        width: border_box.width,
        height: d.border.bottom,
    }));
}

fn get_color(layout_box: &LayoutBox, name: &str) -> Option<Color> {
    match layout_box.box_type {
        BlockNode(style) | InlineNode(style) => match style.value(name) {
            Some(ColorValue(color)) => Some(color),
            _ => None
        },
        AnonymousBlock => None
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
            &SolidColor(color, rect) => {
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
