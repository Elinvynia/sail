use crate::components::TextureFile;
use crate::utils::CustomTexture;
use egui::paint::ClippedShape;
use egui::{ClippedMesh, CtxRef, Event, Modifiers, Pos2, RawInput};
use egui::{PointerButton, Texture as ETexture, TextureId, Vec2 as EVec2};
use std::convert::TryFrom;
use tetra::graphics::mesh::{IndexBuffer, Vertex, VertexBuffer, VertexWinding};
use tetra::graphics::{Color, DrawParams, Texture};
use tetra::input::*;
use tetra::math::Vec2;
use tetra::{Context, Event as TEvent};

// Paint the frame.
pub fn render_ui(ctx: &mut Context, ectx: &mut CtxRef, shapes: Vec<ClippedShape>) {
    let texture = ectx.texture();
    let clipped_meshes = ectx.tessellate(shapes);
    paint(ctx, clipped_meshes, texture.as_ref());
}

// Process tetra Events into egui Events
pub fn handle_event(ctx: &mut Context, ri: &mut RawInput, event: &TEvent, pos_override: Option<Pos2>) {
    let pos = match pos_override {
        Some(p) => p,
        None => Pos2::new(get_mouse_x(ctx), get_mouse_y(ctx)),
    };
    let mut modifiers = Modifiers::default();

    if is_key_modifier_down(ctx, KeyModifier::Ctrl) {
        modifiers.ctrl = true;
        modifiers.command = true;
    }

    if is_key_modifier_down(ctx, KeyModifier::Shift) {
        modifiers.shift = true;
    }

    if is_key_modifier_down(ctx, KeyModifier::Alt) {
        modifiers.alt = true;
    }

    match event {
        TEvent::MouseButtonPressed { button } => {
            let ebutton = match button {
                MouseButton::Left => PointerButton::Primary,
                MouseButton::Right => PointerButton::Secondary,
                MouseButton::Middle => PointerButton::Middle,
                _ => PointerButton::Primary,
            };

            ri.events.push(Event::PointerButton {
                pos,
                button: ebutton,
                pressed: true,
                modifiers,
            })
        }
        TEvent::MouseButtonReleased { button } => {
            let ebutton = match button {
                MouseButton::Left => PointerButton::Primary,
                MouseButton::Right => PointerButton::Secondary,
                MouseButton::Middle => PointerButton::Middle,
                _ => PointerButton::Primary,
            };

            ri.events.push(Event::PointerButton {
                pos,
                button: ebutton,
                pressed: false,
                modifiers,
            })
        }
        TEvent::KeyPressed { key } => {
            if let Some(k) = convert_key(key) {
                ri.events.push(Event::Key {
                    key: k,
                    pressed: true,
                    modifiers,
                })
            }
        }
        TEvent::KeyReleased { key } => {
            if let Some(k) = convert_key(key) {
                ri.events.push(Event::Key {
                    key: k,
                    pressed: false,
                    modifiers,
                })
            }
        }
        TEvent::MouseMoved { position, .. } => {
            let p = match pos_override {
                Some(p) => p,
                None => Pos2::new(position.x, position.y),
            };
            ri.events.push(Event::PointerMoved(p));
        }
        TEvent::MouseWheelMoved { amount } => {
            let am = EVec2 {
                x: amount.x as f32,
                y: amount.y as f32,
            };
            ri.scroll_delta = am;
        }
        TEvent::TextInput { text } => ri.events.push(Event::Text(text.to_owned())),
        TEvent::FocusLost => ri.events.push(Event::PointerGone),
        _ => {}
    }
}

// Paint the GUI using tetra.
pub fn paint(ctx: &mut Context, meshes: Vec<ClippedMesh>, texture: &ETexture) {
    for cm in meshes.into_iter() {
        let mut verts = vec![];

        // Convert egui::Vertex into tetra::Vertex
        for v in cm.1.vertices.iter() {
            let c = v.color.to_tuple();
            let vert = Vertex {
                position: Vec2::new(v.pos.x, v.pos.y),
                uv: Vec2::new(v.uv.x, v.uv.y),
                color: Color::rgba8(c.0, c.1, c.2, c.3),
            };
            verts.push(vert);
        }

        // Indices
        let index = IndexBuffer::new(ctx, &cm.1.indices).unwrap();
        // Vertices
        let buffer = VertexBuffer::new(ctx, &verts).unwrap();

        // Egui uses premultiplied alpha with white pixels.
        let alphas = &texture.pixels;
        let mut fixed = vec![];
        for x in alphas {
            fixed.push(255);
            fixed.push(255);
            fixed.push(255);
            fixed.push(*x);
        }

        let tex = if let TextureId::User(x) = cm.1.texture_id {
            let texture: TextureFile = match CustomTexture::try_from(x) {
                Ok(ct) => ct.into(),
                Err(_) => TextureFile::Unimplemented,
            };
            Texture::new(ctx, &texture.to_string()).unwrap()
        } else {
            Texture::from_rgba(ctx, texture.width as i32, texture.height as i32, &fixed).unwrap()
        };
        let mut mesh = buffer.into_mesh();
        mesh.set_index_buffer(index);
        // This should most likely stay disabled.
        mesh.set_backface_culling(false);
        // This may change in the future (egui doesn't guarantee it). If something looks broken, look here first.
        mesh.set_front_face_winding(VertexWinding::Clockwise);
        mesh.set_texture(tex);
        mesh.draw(ctx, DrawParams::default());
    }
}

// This is neccesary since egui has less Key types than tetra.
fn convert_key(key: &Key) -> Option<egui::Key> {
    match key {
        Key::A => Some(egui::Key::A),
        Key::B => Some(egui::Key::B),
        Key::C => Some(egui::Key::C),
        Key::D => Some(egui::Key::D),
        Key::E => Some(egui::Key::E),
        Key::F => Some(egui::Key::F),
        Key::G => Some(egui::Key::G),
        Key::H => Some(egui::Key::H),
        Key::I => Some(egui::Key::I),
        Key::J => Some(egui::Key::J),
        Key::K => Some(egui::Key::K),
        Key::L => Some(egui::Key::L),
        Key::M => Some(egui::Key::M),
        Key::N => Some(egui::Key::N),
        Key::O => Some(egui::Key::O),
        Key::P => Some(egui::Key::P),
        Key::Q => Some(egui::Key::Q),
        Key::R => Some(egui::Key::R),
        Key::S => Some(egui::Key::S),
        Key::T => Some(egui::Key::T),
        Key::U => Some(egui::Key::U),
        Key::V => Some(egui::Key::V),
        Key::W => Some(egui::Key::W),
        Key::X => Some(egui::Key::X),
        Key::Y => Some(egui::Key::Y),
        Key::Z => Some(egui::Key::Z),
        Key::Num0 => Some(egui::Key::Num0),
        Key::Num1 => Some(egui::Key::Num1),
        Key::Num2 => Some(egui::Key::Num2),
        Key::Num3 => Some(egui::Key::Num3),
        Key::Num4 => Some(egui::Key::Num4),
        Key::Num5 => Some(egui::Key::Num5),
        Key::Num6 => Some(egui::Key::Num6),
        Key::Num7 => Some(egui::Key::Num7),
        Key::Num8 => Some(egui::Key::Num8),
        Key::Num9 => Some(egui::Key::Num9),
        Key::Escape => Some(egui::Key::Escape),
        Key::Tab => Some(egui::Key::Tab),
        Key::Backspace => Some(egui::Key::Backspace),
        Key::Enter => Some(egui::Key::Enter),
        Key::Space => Some(egui::Key::Space),
        Key::Insert => Some(egui::Key::Insert),
        Key::Delete => Some(egui::Key::Delete),
        Key::Home => Some(egui::Key::Home),
        Key::End => Some(egui::Key::End),
        Key::PageDown => Some(egui::Key::PageDown),
        Key::PageUp => Some(egui::Key::PageUp),
        Key::Up => Some(egui::Key::ArrowUp),
        Key::Down => Some(egui::Key::ArrowDown),
        Key::Left => Some(egui::Key::ArrowLeft),
        Key::Right => Some(egui::Key::ArrowRight),
        _ => None,
    }
}
