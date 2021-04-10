use crate::components::{Money, Player};
use crate::world::GameWorld;

pub fn get_player_money(world: &mut GameWorld) -> u64 {
    #[allow(clippy::never_loop)]
    for (_id, (money, _player)) in world.ecs.query::<(&Money, &Player)>().iter().take(1) {
        return money.amount;
    }

    panic!("Player does not exist.")
}
