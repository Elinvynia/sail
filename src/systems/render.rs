use crate::components::{Position, Sprite};
use crate::world::GameWorld;
use tetra::graphics::{DrawParams, Texture as TetraTexture};
use tetra::Context;

// Renders sprites, from the back to the front, respecting layer and texture order.
pub fn render_system(ctx: &mut Context, world: &mut GameWorld) {
    let mut existing_z = vec![];

    for (_id, (position, _sprite)) in world.ecs.query::<(&Position, &Sprite)>().iter() {
        if !existing_z.contains(&position.z) {
            existing_z.push(position.z)
        }
    }

    existing_z.sort_unstable();

    for z in existing_z {
        for (_id, (position, sprite)) in world.ecs.query::<(&Position, &Sprite)>().iter() {
            if position.z != z {
                continue;
            }

            for texture in sprite.textures.iter() {
                if world.textures.get(&texture).is_none() {
                    let tex = TetraTexture::new(ctx, &texture.to_string()).expect("Failed to create sprite.");
                    world.textures.insert(*texture, tex);
                };

                let tex = world.textures.get(&texture).unwrap().clone();
                let pos = (position.x as f32, position.y as f32).into();
                tex.draw(ctx, DrawParams::default().position(pos));
            }
        }
    }
}
