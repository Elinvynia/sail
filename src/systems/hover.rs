use crate::components::{Hoverable, Info, Position};
use crate::utils::TILE_SIZE;
use egui::{pos2, CtxRef, Id, Rect, Window};
use hecs::World;
use tetra::graphics::Camera;
use tetra::input::{is_key_down, Key};
use tetra::Context;

// Displays a popup with information when hovering over certain things.
pub fn hover_system(ctx: &mut Context, ectx: &mut CtxRef, ecs: &World, camera: &Camera) {
    let mouse_pos = camera.mouse_position(ctx);

    for (_id, (info, position, _h)) in ecs.query::<(&Info, &Position, &Hoverable)>().iter() {
        if !is_key_down(ctx, Key::LeftShift) {
            continue;
        }

        let pos = (position.x as f32, position.y as f32);
        let size = TILE_SIZE as f32;
        let rect = Rect::from_min_size(pos2(pos.0, pos.1), (size, size).into());
        if !rect.contains(pos2(mouse_pos.x, mouse_pos.y)) {
            continue;
        }

        let window_pos = pos2(position.x as f32, position.y as f32);
        Window::new(&info.name)
            .id(Id::new(position.x + position.y))
            .fixed_pos(window_pos)
            .resizable(false)
            .collapsible(false)
            .show(ectx, |ui| {
                ui.label(&info.description);
            });
    }
}
