///! Basic CSS block layout.

use style::{PropertyMap};
use css::{Value, Keyword, Length, Px};
use std::iter::AdditiveIterator; // for `sum`

/// Calculate the width of a block-level non-replaced element in normal flow.
pub fn calculate_block_width(specified_values: &PropertyMap, containing_block_width: f32) {
    // http://www.w3.org/TR/CSS2/visudet.html#blockwidth
    let val = |name| specified_values.find_equiv(&name).map(|v| v.clone());

    // `width` has initial value `auto`.
    let auto = Keyword("auto".to_string());
    let mut width = val("width").unwrap_or(auto.clone());

    // margin, border, and padding have initial value 0.
    let get_length = |name, fallback| {
        val(name).unwrap_or_else(|| val(fallback).unwrap_or(Length(0.0, Px)))
    };
    let mut margin_left = get_length("margin-left", "margin");
    let mut margin_right = get_length("margin-right", "margin");

    let border_left = get_length("border-left-width", "border-width");
    let border_right = get_length("border-right-width", "border-width");

    let padding_left = get_length("padding-left", "padding");
    let padding_right = get_length("padding-right", "padding");

    let total = sum_lengths([&margin_left, &margin_right, &border_left, &border_right,
                             &padding_left, &padding_right, &width]);

    // Adjust used values so that the above sum equals containing_block_width.
    // Each arm of the `match` should increase the above sum by exactly `underflow`.
    let underflow = containing_block_width - total;
    match (width == auto, margin_left == auto, margin_right == auto) {
        // If the values are overconstrained, calculate margin_right.
        (false, false, false) => {
            margin_right = Length(px(margin_right) + underflow, Px);
        }
        // If exactly one value is `auto`, its used value follows from the equality.
        (false, false, true) => {
            margin_right = Length(underflow, Px);
        }
        (false, true, false) => {
            margin_left = Length(underflow, Px);
        }
        // If 'width' is set to 'auto', any other 'auto' values become '0'.
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
}

/// Add together all the non-`auto` lengths.
fn sum_lengths(values: &[&Value]) -> f32 {
    values.iter().map(|value| match **value {
        Length(f, Px) => f,
        _ => 0.0 // ignore 'auto' or invalid widths
    }).sum()
}

/// Return the size of a Length in px.
fn px(value: Value) -> f32 {
    match value {
        Length(f, Px) => f,
        Keyword(_) => fail!("not a length")
    }
}
