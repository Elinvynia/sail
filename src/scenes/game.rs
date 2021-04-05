use crate::components::Position;
use crate::entities::sea;
use crate::scenes::{PauseScene, Scene, SceneSwitch, Scenes};
use crate::systems::{hover_system, render_system};
use crate::world::GameWorld;
use egui::CtxRef;
use hecs::EntityBuilder;
use tetra::input::*;
use tetra::{Context, Event};

#[derive(Debug)]
pub struct GameScene {
    pause: bool,
}

impl GameScene {
    pub fn new(world: &mut GameWorld, _ctx: &mut Context) -> Self {
        let mut x = 0;
        let mut y = 0;

        for _ in 0..10 {
            for _ in 0..10 {
                let mut builder = EntityBuilder::new();
                let pos = Position::new(x, y, 0);
                let sea = sea();

                builder.add(pos);
                builder.add_bundle(sea);
                let entity = builder.build();
                world.ecs.spawn(entity);
                x += 32;
            }
            x = 0;
            y += 32;
        }

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
