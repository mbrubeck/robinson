///! Basic CSS block layout.

use style;
use css::{Keyword, Length, Px};
use std::default::Default;
use std::iter::AdditiveIterator; // for `sum`

/// A tree of nodes with associated layout data.
pub struct LayoutNode<'a> {
    pub style_node: &'a style::StyledNode<'a>,
    pub dimensions: Dimensions,
    pub children: Vec<LayoutNode<'a>>,
}

// CSS box model. All sizes are in px.

#[deriving(Default, Show)]
pub struct Dimensions {
    // Position of the content area relative to the document origin:
    pub x: f32,
    pub y: f32,

    // Content area size:
    pub width: f32,
    pub height: f32,

    // Surrounding edges:
    pub padding: EdgeSizes,
    pub border: EdgeSizes,
    pub margin: EdgeSizes,
}

#[deriving(Default, Show)]
struct EdgeSizes { left: f32, right: f32, top: f32, bottom: f32 }

pub fn layout<'a>(node: &'a style::StyledNode<'a>, containing_block: Dimensions) -> LayoutNode<'a> {
    let mut layout_node = LayoutNode {
        style_node: node,
        dimensions: Default::default(),
        children: Vec::new(),
    };

    // Child width can depend on parent width, so we need to calculate this node's width before
    // laying out its children.
    calculate_width(&mut layout_node, containing_block);

    // Recursively lay out the children of this node within its content area.
    layout_content(&mut layout_node, containing_block);

    // Parent height can depend on child height, so `calculate_height` must be called after the
    // content layout is finished.
    calculate_height(&mut layout_node);

    return layout_node;
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

    let total = [&margin_left, &margin_right, &border_left, &border_right,
                 &padding_left, &padding_right, &width].iter().map(|v| v.to_px()).sum();

    // If width is not auto and the total is wider than the container, treat auto margins as 0.
    if width != auto && total > containing_block.width {
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
    let underflow = containing_block.width - total;
    match (width == auto, margin_left == auto, margin_right == auto) {
        // If the values are overconstrained, calculate margin_right.
        (false, false, false) => {
            margin_right = Length(margin_right.to_px() + underflow, Px);
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
    d.width = width.to_px();

    d.padding.left = padding_left.to_px();
    d.padding.right = padding_right.to_px();

    d.border.left = border_left.to_px();
    d.border.right = border_right.to_px();

    d.margin.left = margin_left.to_px();
    d.margin.right = margin_right.to_px();

    d.x = containing_block.x + d.margin.left + d.border.left + d.padding.left;
}

/// Lay out the node's children within its content area.
fn layout_content(node: &mut LayoutNode, containing_block: Dimensions) {
    // First we need to find the position of the content area...
    let style = node.style_node;
    let d = &mut node.dimensions;

    // margin, border, and padding have initial value 0.
    let zero = Length(0.0, Px);

    // If margin-top or margin-bottom is `auto`, the used value is zero.
    d.margin.top = style.lookup("margin-top", "margin", &zero).to_px();
    d.margin.bottom = style.lookup("margin-bottom", "margin", &zero).to_px();

    d.border.top = style.lookup("border-top-width", "border-width", &zero).to_px();
    d.border.bottom = style.lookup("border-bottom-width", "border-width", &zero).to_px();

    d.padding.top = style.lookup("padding-top", "padding", &zero).to_px();
    d.padding.bottom = style.lookup("padding-bottom", "padding", &zero).to_px();

    d.y = containing_block.y + d.margin.top + d.border.top + d.padding.top;

    // Now we can lay out the children within the content area.
    let mut content_height = 0.0;
    for child_style in node.style_node.children.iter() {
        // Skip nodes with `display` set to `None`.
        if child_style.display() != style::None {
            let mut child_layout = layout(child_style, *d);

            // Position each child below the previous one. TODO: margin collapsing
            child_layout.dimensions.y = d.y + content_height;
            content_height = content_height + child_layout.dimensions.margin_box_height();

            node.children.push(child_layout);
        }
    }
    // Record the total height of the children.
    d.height = content_height;
}

/// Height of a block-level non-replaced element in normal flow with overflow visible.
///
/// http://www.w3.org/TR/CSS2/visudet.html#normal-block
fn calculate_height(node: &mut LayoutNode) {
    // If the height is an absolute length, it overrides the content height.
    match node.style_node.value("height") {
        Some(Length(h, Px)) => { node.dimensions.height = h; }
        _ => {}
    }
}

impl Dimensions {
    /// Total height of a box including its margins, border, and padding.
    fn margin_box_height(&self) -> f32 {
        self.height + self.padding.top + self.padding.bottom
                    + self.border.top + self.border.bottom
                    + self.margin.top + self.margin.bottom
    }
}
