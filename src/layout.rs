///! Basic CSS block layout.

use style::StyledNode;
use css::{Value, Keyword, Length, Px, Color};
use std::default::Default;
use std::iter::AdditiveIterator; // for `sum`

/// A tree of nodes with associated layout data.
pub struct LayoutNode<'a> {
    pub style_node: &'a StyledNode<'a>,
    pub dimensions: Dimensions,
    pub children: Vec<LayoutNode<'a>>,
}

// CSS box model. All sizes are in px.

#[deriving(Default, Show)]
pub struct Dimensions {
    // Content area:
    pub width: f32,
    pub height: f32,

    // Position of the content area relative to the document origin:
    pub x: f32,
    pub y: f32,

    // Surrounding edges:
    pub padding: EdgeSizes,
    pub border: EdgeSizes,
    pub margin: EdgeSizes,
}

#[deriving(Default, Show)]
struct EdgeSizes { left: f32, right: f32, top: f32, bottom: f32 }

pub fn layout<'a>(node: &'a StyledNode<'a>, containing_block: Dimensions) -> LayoutNode<'a> {
    let mut layout_node = LayoutNode {
        style_node: node,
        dimensions: Default::default(),
        children: Vec::new(),
    };
    calculate_width(&mut layout_node, containing_block);
    for child in node.children.iter() {
        layout_node.children.push(layout(child, layout_node.dimensions))
    }
    calculate_height(&mut layout_node);
    layout_node
}

/// Calculate the width of a block-level non-replaced element in normal flow.
///
/// http://www.w3.org/TR/CSS2/visudet.html#blockwidth
fn calculate_width(node: &mut LayoutNode, containing_block: Dimensions) {
    let style = node.style_node;

    // `width` has initial value `auto`.
    let auto = Keyword("auto".to_string());
    let mut width = style.value("width").unwrap_or(auto.clone());

    // margin, border, and padding have initial value 0.
    let zero = Length(0.0, Px);

    let mut margin_left = style.lookup("margin-left", "margin", &zero);
    let mut margin_right = style.lookup("margin-right", "margin", &zero);

    let border_left = style.lookup("border-left-width", "border-width", &zero);
    let border_right = style.lookup("border-right-width", "border-width", &zero);

    let padding_left = style.lookup("padding-left", "padding", &zero);
    let padding_right = style.lookup("padding-right", "padding", &zero);

    let total_width = sum_lengths([&margin_left, &margin_right, &border_left, &border_right,
                                   &padding_left, &padding_right, &width]);

    // If width is not auto and the total is wider than the container, treat auto margins as 0.
    if width != auto && total_width > containing_block.width {
        if margin_left == auto {
            margin_left = Length(0.0, Px);
        }
        if margin_right == auto {
            margin_right = Length(0.0, Px);
        }
    }

    // Adjust used values so that the above sum equals `containing_block.width`.
    // Each arm of the `match` should increase the total width by exactly `underflow`,
    // and afterward all values should be absolute lengths in px.
    let underflow = containing_block.width - total_width;
    match (width == auto, margin_left == auto, margin_right == auto) {
        // If the values are overconstrained, calculate margin_right.
        (false, false, false) => {
            margin_right = Length(px(margin_right) + underflow, Px);
        }
        // If exactly one value is auto, its used value follows from the equality.
        (false, false, true) => {
            margin_right = Length(underflow, Px);
        }
        (false, true, false) => {
            margin_left = Length(underflow, Px);
        }
        // If width is set to auto, any other auto values become 0.
        (true, _, _) => {
            if margin_left == auto {
                margin_left = Length(0.0, Px);
            }
            if margin_right == auto {
                margin_right = Length(0.0, Px);
            }
            width = Length(underflow, Px);
        }
        (false, true, true) => {
            // If margin-left and margin-right are both auto, their used values are equal.
            margin_left = Length(underflow / 2.0, Px);
            margin_right = Length(underflow / 2.0, Px);
        }
    }

    let d = &mut node.dimensions;
    d.width = px(width);

    d.padding.left = px(padding_left);
    d.padding.right = px(padding_right);

    d.border.left = px(border_left);
    d.border.right = px(border_right);

    d.margin.left = px(margin_left);
    d.margin.right = px(margin_right);

    d.x = containing_block.x + d.margin.left + d.border.left + d.padding.left;
}

/// Height of a block-level non-replaced element in normal flow with overflow visible.
///
/// http://www.w3.org/TR/CSS2/visudet.html#normal-block
fn calculate_height(node: &mut LayoutNode) {
    let style = node.style_node;

    // `height` has initial value `auto`.
    let auto = Keyword("auto".to_string());
    let mut height = style.value("height").unwrap_or(auto.clone());

    // margin, border, and padding have initial value 0.
    let zero = Length(0.0, Px);

    let mut margin_top = style.lookup("margin-top", "margin", &zero);
    let mut margin_bottom = style.lookup("margin-bottom", "margin", &zero);

    let border_top = style.lookup("border-top-width", "border-width", &zero);
    let border_bottom = style.lookup("border-bottom-width", "border-width", &zero);

    let padding_top = style.lookup("padding-top", "padding", &zero);
    let padding_bottom = style.lookup("padding-bottom", "padding", &zero);

    // If margin-top or margin-bottom is `auto`, the used value is 0.
    if margin_top == auto {
        margin_top = Length(0.0, Px);
    }
    if margin_bottom == auto {
        margin_bottom = Length(0.0, Px);
    }

    // TODO: Calculate child `y` coordinates.

    // If height is `auto` the used value depends on the element's children.
    if height == auto {
        // TODO: margin collapsing
        let content_height = node.children.iter().map(total_height).sum();
        height = Length(content_height, Px);
    }

    let dimensions = &mut node.dimensions;
    dimensions.height = px(height);

    dimensions.padding.top = px(padding_top);
    dimensions.padding.bottom = px(padding_bottom);

    dimensions.border.top = px(border_top);
    dimensions.border.bottom = px(border_bottom);

    dimensions.margin.top = px(margin_top);
    dimensions.margin.bottom = px(margin_bottom);
}

/// Add together all the non-`auto` lengths.
fn sum_lengths(values: &[&Value]) -> f32 {
    values.iter().map(|value| match **value {
        Length(f, Px) => f,
        _ => 0.0 // ignore 'auto' or invalid widths
    }).sum()
}

fn total_height(node: &LayoutNode) -> f32 {
    let d = &node.dimensions;
    d.height + d.padding.top + d.padding.bottom
             + d.border.top + d.border.bottom
             + d.margin.top + d.margin.bottom
}

/// Return the size of a Length in px.
fn px(value: Value) -> f32 {
    match value {
        Length(f, Px) => f,
        Color(..) | Keyword(..) => fail!("not a length")
    }
}
