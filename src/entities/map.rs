use crate::components::{Hoverable, Position};
use crate::entities::{island, sea};
use crate::utils::{Layer, TILE_SIZE};
use crate::world::GameWorld;
use hecs::EntityBuilder;
use rand::prelude::*;

const MAX_ISLANDS: u32 = 10;

// Generates a random map, sprinkling islands into the sea.
pub fn generate_map(width: u32, height: u32, world: &mut GameWorld) {
    let mut rng = rand::thread_rng();

    let mut x;
    let mut y = 0;

    let total_tiles = width * height;
    let mut island_prob = 0;
    let mut current_islands = 0;
    let mut current_step = 0;

    for _ in 0..height {
        x = 0;
        for _ in 0..width {
            let mut builder = EntityBuilder::new();
            let pos = Position::new(x, y, Layer::Background as u32);

            if rng.gen_ratio(island_prob, 100) && current_islands < MAX_ISLANDS {
                let island = island();
                builder.add(Hoverable);
                builder.add_bundle(island);
                current_islands += 1;
                island_prob = 0;
                current_step = 0;
                current_step += (total_tiles / MAX_ISLANDS) / 10
            } else {
                let sea = sea();
                builder.add_bundle(sea);
                current_step += 1;
                island_prob += current_step / (total_tiles / MAX_ISLANDS);
            }

            builder.add(pos);
            let entity = builder.build();
            world.ecs.spawn(entity);

            x += TILE_SIZE as u32;
        }
        y += TILE_SIZE as u32;
    }
}
