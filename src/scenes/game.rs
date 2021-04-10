use crate::entities::generate_map;
use crate::input::{key_to_dir, Dir};
use crate::scenes::{PauseScene, Scene, SceneSwitch, Scenes};
use crate::systems::{hover_system, render_system};
use crate::utils::{position, TILE_SIZE};
use crate::world::GameWorld;
use egui::{pos2, vec2, CtxRef, Window};
use tetra::graphics::{set_transform_matrix, Camera};
use tetra::input::{get_keys_down, Key};
use tetra::window::get_size;
use tetra::{Context, Event};

#[derive(Debug)]
pub struct GameScene {
    pause: bool,
    camera: Camera,
    width: i32,
    height: i32,
}

impl GameScene {
    pub fn new(world: &mut GameWorld, ctx: &mut Context) -> Self {
        let (width, height) = get_size(ctx);
        let (map_width, map_height) = (58, 28);

        generate_map(map_width, map_height, world);

        let mut camera = Camera::with_window_size(ctx);
        camera.position = [width as f32 / 2.0, height as f32 / 2.0].into();
        camera.update();

        GameScene {
            pause: false,
            camera,
            width: (map_width * TILE_SIZE) as i32,
            height: (map_height * TILE_SIZE) as i32,
        }
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
                let (width, height) = get_size(ctx);
                match dir {
                    Dir::Up => {
                        let top = self.camera.position.y - (height / 2) as f32;
                        if top - 5.0 > 0.0 {
                            self.camera.position.y -= 5.0;
                        }
                    }
                    Dir::Down => {
                        let bottom = self.camera.position.y + (height / 2) as f32;
                        if bottom + 5.0 < self.height as f32 {
                            self.camera.position.y += 5.0;
                        }
                    }
                    Dir::Left => {
                        let left = self.camera.position.x - (width / 2) as f32;
                        if left - 5.0 > 0.0 {
                            self.camera.position.x -= 5.0;
                        }
                    }
                    Dir::Right => {
                        let right = self.camera.position.x + (width / 2) as f32;
                        if right + 5.0 < self.width as f32 {
                            self.camera.position.x += 5.0;
                        }
                    }
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

        let rect = position(pos2(100.0, 100.0), vec2(100.0, 150.0));
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
