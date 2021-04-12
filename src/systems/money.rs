use crate::components::{Money, Player};
use crate::world::GameWorld;

// Returns the player money, assumes there is one and just one.
pub fn get_player_money(world: &mut GameWorld) -> u64 {
    if world.ecs.query::<(&Money, &Player)>().iter().count() > 1 {
        panic!("There are multiple player!")
    }

    #[allow(clippy::never_loop)]
    for (_id, (money, _player)) in world.ecs.query::<(&Money, &Player)>().iter() {
        return money.amount;
    }

    panic!("Player does not exist!")
}
