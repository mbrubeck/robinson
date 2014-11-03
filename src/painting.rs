use layout::{AnonymousBlock, BlockNode, InlineNode, LayoutBox, Rect};
use css::{ColorValue, Color};
use std::default::Default;

#[deriving(Show)]
pub enum DisplayItem {
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
    let background_style = match layout_box.box_type {
        BlockNode(style) | InlineNode(style) => {
            let default = ColorValue(Default::default()); // transparent
            Some(style.lookup("background-color", "background", &default))
        }
        AnonymousBlock => None
    };

    match background_style {
        Some(ColorValue(color)) => {
            list.push(SolidColor(layout_box.dimensions.padding_box(), color))
        }
        _ => {} // other values not supported yet
    }
}
