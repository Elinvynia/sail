use crate::components::{Name, Position};
use crate::world::GameWorld;
use egui::{pos2, CtxRef, Id, Rect, Window};
use tetra::input::{get_mouse_position, is_key_down, Key};
use tetra::Context;

pub fn hover_system(ctx: &mut Context, ectx: &mut CtxRef, world: &mut GameWorld) {
    let mouse_pos = get_mouse_position(ctx);

    for (_id, (name, position)) in world.ecs.query::<(&Name, &Position)>().iter() {
        if !is_key_down(ctx, Key::LeftShift) {
            continue;
        }

        let rect = Rect::from_min_size(pos2(position.x as f32, position.y as f32), (32.0, 32.0).into());
        if !rect.contains(pos2(mouse_pos.x, mouse_pos.y)) {
            continue;
        }

        let window_pos = pos2(position.x as f32, position.y as f32);
        Window::new(&name.name)
            .id(Id::new(position.x + position.y))
            .fixed_pos(window_pos)
            .resizable(false)
            .collapsible(false)
            .show(ectx, |ui| {
                ui.label(&name.description);
            });
    }
}
