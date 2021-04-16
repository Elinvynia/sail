use crate::components::{Inventory, Player};
use crate::world::GameWorld;
use hecs::Entity;

// Returns the player inventory, assumes there is one and just one.
pub fn get_player_inventory(world: &mut GameWorld) -> Inventory {
    if world.ecs.query::<(&Inventory, &Player)>().iter().count() > 1 {
        panic!("There are multiple players!")
    }

    #[allow(clippy::never_loop)]
    for (_id, (inv, _player)) in world.ecs.query::<(&Inventory, &Player)>().iter() {
        return inv.clone();
    }

    panic!("Player does not exist!")
}

// Returns the inventory of an island.
pub fn get_island_inventory(world: &mut GameWorld, entity: Entity) -> Inventory {
    (*world.ecs.get::<Inventory>(entity).unwrap()).clone()
}
