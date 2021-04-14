use crate::input::MoveDir;
use crate::systems::move_player;
use crate::world::GameWorld;
use tetra::Context;

pub fn tick(_ctx: &Context, world: &mut GameWorld, mdir: MoveDir) {
    move_player(world, mdir);
}
