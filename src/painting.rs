use layout::{AnonymousBlock, BlockNode, InlineNode, LayoutBox, Rect};
use css::{Value, Color};

pub struct Canvas {
    pub pixels: Vec<Color>,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug)]
pub enum DisplayCommand {
    SolidColor(Color, Rect),
}

pub type DisplayList = Vec<DisplayCommand>;

pub fn build_display_list(layout_root: &LayoutBox) -> DisplayList {
    let mut list = Vec::new();
    render_layout_box(&mut list, layout_root);
    list
}

fn render_layout_box(list: &mut DisplayList, layout_box: &LayoutBox) {
    render_background(list, layout_box);
    render_borders(list, layout_box);
    for child in &layout_box.children {
        render_layout_box(list, child);
    }
}

fn render_background(list: &mut DisplayList, layout_box: &LayoutBox) {
    get_color(layout_box, "background").map(|color|
        list.push(DisplayCommand::SolidColor(color, layout_box.dimensions.border_box())));
}

fn render_borders(list: &mut DisplayList, layout_box: &LayoutBox) {
    let color = match get_color(layout_box, "border-color") {
        Some(color) => color,
        _ => return
    };

    let d = &layout_box.dimensions;
    let border_box = d.border_box();

    // Left border
    list.push(DisplayCommand::SolidColor(color, Rect {
        x: border_box.x,
        y: border_box.y,
        width: d.border.left,
        height: border_box.height,
    }));

    // Right border
    list.push(DisplayCommand::SolidColor(color, Rect {
        x: border_box.x + border_box.width - d.border.right,
        y: border_box.y,
        width: d.border.right,
        height: border_box.height,
    }));

    // Top border
    list.push(DisplayCommand::SolidColor(color, Rect {
        x: border_box.x,
        y: border_box.y,
        width: border_box.width,
        height: d.border.top,
    }));

    // Bottom border
    list.push(DisplayCommand::SolidColor(color, Rect {
        x: border_box.x,
        y: border_box.y + border_box.height - d.border.bottom,
        width: border_box.width,
        height: d.border.bottom,
    }));
}

/// Return the specified color for CSS property `name`, or None if no color was specified.
fn get_color(layout_box: &LayoutBox, name: &str) -> Option<Color> {
    match layout_box.box_type {
        BlockNode(style) | InlineNode(style) => match style.value(name) {
            Some(Value::ColorValue(color)) => Some(color),
            _ => None
        },
        AnonymousBlock => None
    }
}

/// Paint a tree of LayoutBoxes to an array of pixels.
pub fn paint(layout_root: &LayoutBox, rect: &Rect) -> Canvas {
    let display_list = build_display_list(layout_root);
    let mut canvas = Canvas::new(rect.width as usize, rect.height as usize, None);
    for item in display_list {
        canvas.paint_item(&item);
    }
    canvas
}

impl Canvas {
    /// Create a blank canvas
    pub fn new(width: usize, height: usize, color: Option<Color>) -> Canvas {
        let fill_color: Color = match color {
            Some(value) => value,
            None => { Color { r: 255, g: 255, b: 255, a: 255 } }
        };
        
        let size: usize = width * height;
        Canvas {
            pixels: vec![fill_color; size],
            width: width,
            height: height,
        }
    }

    fn paint_item(&mut self, item: &DisplayCommand) {
        match *item {
            DisplayCommand::SolidColor(color, rect) => {
                // Clip the rectangle to the canvas boundaries.
                let x0 = rect.x.clamp(0, self.width as i32) as usize;
                let y0 = rect.y.clamp(0, self.height as i32) as usize;
                let x1 = (rect.x + rect.width).clamp(0, self.width as i32) as usize;
                let y1 = (rect.y + rect.height).clamp(0, self.height as i32) as usize;

                for y in y0 .. y1 {
                    for x in x0 .. x1 {
                        // TODO: alpha compositing with existing pixel
                        self.pixels[y * self.width + x] = color;
                    }
                }
            }
        }
    }
}
