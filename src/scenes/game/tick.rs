use crate::input::MoveDir;
use crate::scenes::SceneSwitch;
use crate::systems::move_player;
use crate::world::GameWorld;
use tetra::Context;

pub fn tick(_ctx: &Context, world: &mut GameWorld, mdir: MoveDir) -> Option<SceneSwitch> {
    let mov = move_player(world, mdir);
    if mov.is_some() {
        return mov;
    }

    None
}
