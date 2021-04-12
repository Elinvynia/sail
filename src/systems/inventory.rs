use crate::components::{Inventory, Player};
use crate::world::GameWorld;

pub fn get_player_inventory(world: &mut GameWorld) -> Inventory {
    #[allow(clippy::never_loop)]
    for (_id, (inv, _player)) in world.ecs.query::<(&Inventory, &Player)>().iter() {
        return inv.clone();
    }

    panic!("Player does not exist.")
}
