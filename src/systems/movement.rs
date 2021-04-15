use crate::components::{Inventory, ItemName, Player, Position, Sea};
use crate::input::MoveDir;
use crate::scenes::{EndScene, SceneSwitch, Scenes};
use crate::utils::TILE_SIZE;
use crate::world::GameWorld;

pub fn move_player(world: &mut GameWorld, mdir: MoveDir) -> Option<SceneSwitch> {
    // Find out where we want to move the player.
    let mut target_pos = (0, 0);
    for (_id, (position, _player)) in world.ecs.query::<(&Position, &Player)>().iter() {
        use MoveDir::*;
        target_pos = match mdir {
            Up => (position.x, position.y - TILE_SIZE),
            Down => (position.x, position.y + TILE_SIZE),
            Left => (position.x - TILE_SIZE, position.y),
            Right => (position.x + TILE_SIZE, position.y),
        };
    }

    for (_id, (position, inventory, _player)) in world.ecs.query::<(&mut Position, &mut Inventory, &Player)>().iter() {
        // Find out if sea is at the desired position.
        let mut ok = false;
        for (_id, (pos, _sea)) in world.ecs.query::<(&Position, &Sea)>().iter() {
            if pos.x == target_pos.0 && pos.y == target_pos.1 {
                ok = true;
            }
        }

        // If so, move us there.
        if ok {
            position.x = target_pos.0;
            position.y = target_pos.1;
        } else {
            break;
        }

        // Remove one unit of water per move.
        for item in inventory.items.iter_mut() {
            if item.name == ItemName::Water {
                item.amount -= 1;

                if item.amount == 0 {
                    let scene = Scenes::End(EndScene::new());
                    return Some(SceneSwitch::ReplaceAll(scene));
                }

                break;
            }
        }
    }

    None
}
