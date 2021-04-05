use egui::{pos2, Pos2, Rect, Vec2};
use tetra::window::{get_height, get_width};
use tetra::Context;

// Centers the element given the size.
pub fn center(ctx: &Context, size: Vec2) -> Rect {
    let window_width = (get_width(ctx) / 2) as f32;
    let window_height = (get_height(ctx) / 2) as f32;

    let top_left = pos2(window_width - (size.x / 2.0), window_height - (size.y / 2.0));

    Rect::from_min_size(top_left, size)
}

// Give it the center position and size of the element.
pub fn position(center_pos: Pos2, size: Vec2) -> Rect {
    let top_left = pos2(center_pos.x - (size.x / 2.0), center_pos.y - (size.y / 2.0));

    Rect::from_min_size(top_left, size)
}
