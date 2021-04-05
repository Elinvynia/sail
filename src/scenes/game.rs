use crate::components::Position;
use crate::entities::sea;
use crate::input::{key_to_dir, Dir};
use crate::scenes::{PauseScene, Scene, SceneSwitch, Scenes};
use crate::systems::{hover_system, render_system};
use crate::utils::position;
use crate::world::GameWorld;
use egui::{pos2, vec2, CtxRef, Window};
use hecs::EntityBuilder;
use tetra::graphics::{set_transform_matrix, Camera};
use tetra::input::{get_keys_down, Key};
use tetra::window::get_size;
use tetra::{Context, Event};

#[derive(Debug)]
pub struct GameScene {
    pause: bool,
    camera: Camera,
}

impl GameScene {
    pub fn new(world: &mut GameWorld, ctx: &mut Context) -> Self {
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

        let mut camera = Camera::with_window_size(ctx);
        let (width, height) = get_size(ctx).into();
        camera.position = [width as f32 / 3.0, height as f32 / 3.0].into();
        camera.update();

        GameScene { pause: false, camera }
    }
}

impl Scene for GameScene {
    fn update(&mut self, world: &mut GameWorld, ctx: &mut Context) -> tetra::Result<SceneSwitch> {
        if self.pause {
            self.pause = false;
            let scene = Scenes::Pause(PauseScene::new(world, ctx));
            return Ok(SceneSwitch::Push(scene));
        }

        for key in get_keys_down(ctx) {
            if let Some(dir) = key_to_dir(key) {
                match dir {
                    Dir::Up => self.camera.position.y -= 5.0,
                    Dir::Down => self.camera.position.y += 5.0,
                    Dir::Left => self.camera.position.x -= 5.0,
                    Dir::Right => self.camera.position.x += 5.0,
                }

                self.camera.update();
            }
        }

        Ok(SceneSwitch::None)
    }

    fn draw(&mut self, world: &mut GameWorld, ctx: &mut Context, ectx: &mut CtxRef) -> tetra::Result {
        set_transform_matrix(ctx, self.camera.as_matrix());
        render_system(ctx, world);
        hover_system(ctx, ectx, world);

        let rect = position(pos2(-100.0, 100.0), vec2(100.0, 150.0));
        Window::new("info")
            .resizable(false)
            .collapsible(false)
            .fixed_rect(rect)
            .show(ectx, |ui| {
                ui.label("Useful information here.");
            });

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
