use layout::{AnonymousBlock, BlockNode, InlineNode, LayoutBox, Rect};
use css::{ColorValue, Color};
use std::iter::range;

/// Paint a tree of LayoutBoxes to an array of pixels.
pub fn paint(layout_root: &LayoutBox, bounds: Rect) -> Canvas {
    let display_list = build_display_list(layout_root);
    let mut canvas = Canvas::new(bounds.width as uint, bounds.height as uint);
    for item in display_list.iter() {
        canvas.paint_item(item);
    }
    return canvas;
}

#[deriving(Show)]
enum DisplayItem {
    SolidColor(Color, Rect),
}

type DisplayList = Vec<DisplayItem>;

fn build_display_list(layout_root: &LayoutBox) -> DisplayList {
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
        list.push(SolidColor(color, layout_box.dimensions.border_box())));
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
    /// Create a blank canvas
    fn new(width: uint, height: uint) -> Canvas {
        let white = Color { r: 255, g: 255, b: 255, a: 255 };
        return Canvas {
            pixels: Vec::from_elem(width * height, white),
            width: width,
            height: height,
        }
    }

    fn paint_item(&mut self, item: &DisplayItem) {
        match item {
            &SolidColor(color, rect) => {
                // Clip the rectangle to the canvas boundaries.
                let x0 = rect.x.clamp(0.0, self.width as f32) as uint;
                let y0 = rect.y.clamp(0.0, self.height as f32) as uint;
                let x1 = (rect.x + rect.width).clamp(0.0, self.width as f32) as uint;
                let y1 = (rect.y + rect.height).clamp(0.0, self.height as f32) as uint;

                for y in range(y0, y1) {
                    for x in range(x0, x1) {
                        // TODO: alpha compositing with existing pixel
                        self.pixels[y * self.width + x] = color;
                    }
                }
            }
        }
    }
}

trait FloatClamp : FloatMath {
    fn clamp(self, lower: Self, upper: Self) -> Self {
        self.max(lower).min(upper)
    }
}
impl<T: FloatMath> FloatClamp for T {}
