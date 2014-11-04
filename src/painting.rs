use image::{GenericImage, ImageBuf, Rgba};
use layout::{AnonymousBlock, BlockNode, InlineNode, LayoutBox, Rect};
use css::ColorValue;
use std::iter::range;
use std::cmp::{max, min};

#[deriving(Show)]
pub enum DisplayItem {
    SolidColor(Rect, Rgba<u8>),
}

type DisplayList = Vec<DisplayItem>;

pub fn paint(list: &DisplayList, bounds: Rect) -> ImageBuf<Rgba<u8>> {
    let white = Rgba(255, 255, 255, 255);
    let mut img = ImageBuf::from_pixel(bounds.width as u32, bounds.height as u32, white);
    for item in list.iter() {
        item.paint(&mut img);
    }
    return img;
}

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
    let transparent = Rgba(0,0,0,0);
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

impl DisplayItem {
    fn paint(&self, img: &mut ImageBuf<Rgba<u8>>) {
        let (width, height) = img.dimensions();
        match self {
            &SolidColor(rect, color) => {
                let xmin = max(0, rect.x as u32);
                let ymin = max(0, rect.y as u32);
                let xmax = min(width, (rect.x + rect.width) as u32);
                let ymax = min(height, (rect.y + rect.height) as u32);

                for x in range(xmin, xmax) {
                for y in range(ymin, ymax) {
                    // TODO: alpha compositing with existing pixel
                    img.put_pixel(x, y, color);
                }}
            }
        }
    }
}
