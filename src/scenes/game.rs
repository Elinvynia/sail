use crate::scenes::{PauseScene, Scene, SceneSwitch, Scenes};
use crate::systems::{hover_system, render_system};
use crate::world::GameWorld;
use egui::CtxRef;
use tetra::input::*;
use tetra::{Context, Event};

#[derive(Debug)]
pub struct GameScene {
    pub pause: bool,
}

impl GameScene {
    pub fn new(_world: &mut GameWorld, _ctx: &mut Context) -> Self {
        GameScene { pause: false }
    }
}

impl Scene for GameScene {
    fn update(&mut self, world: &mut GameWorld, ctx: &mut Context) -> tetra::Result<SceneSwitch> {
        if self.pause {
            self.pause = false;
            let scene = Scenes::Pause(PauseScene::new(world, ctx));
            return Ok(SceneSwitch::Push(scene));
        }

        Ok(SceneSwitch::None)
    }

    fn draw(&mut self, world: &mut GameWorld, ctx: &mut Context, ectx: &mut CtxRef) -> tetra::Result {
        render_system(ctx, world);
        hover_system(ctx, ectx, world);

        Ok(())
    }

    fn event(&mut self, _world: &mut GameWorld, _ctx: &mut Context, event: Event) -> tetra::Result {
        if let Event::KeyPressed { key } = event {
            if key == Key::Escape {
                self.pause = true;
            }
        }

        Ok(())
    }
}
