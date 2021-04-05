use crate::components::{Position, Texture};
use crate::world::GameWorld;
use tetra::graphics::{DrawParams, Texture as TetraTexture};
use tetra::Context;

pub fn render_system(ctx: &mut Context, world: &mut GameWorld) {
    let mut existing_z = vec![];

    for (_id, (position, _texture)) in world.ecs.query::<(&Position, &Texture)>().iter() {
        if !existing_z.contains(&position.z) {
            existing_z.push(position.z)
        }
    }

    existing_z.sort_unstable();

    for z in existing_z {
        for (_id, (position, texture)) in world.ecs.query::<(&Position, &Texture)>().iter() {
            if position.z != z {
                continue;
            }

            if world.textures.get(&texture.texture).is_none() {
                let tex = TetraTexture::new(ctx, &texture.texture).expect("Failed to create texture.");
                world.textures.insert(texture.texture.clone(), tex);
            };

            let tex = world.textures.get(&texture.texture).unwrap().clone();
            let pos = (position.x as f32, position.y as f32).into();
            tex.draw(ctx, DrawParams::default().position(pos))
        }
    }
}
