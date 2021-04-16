use crate::components::Position;
use crate::entities::{generate_map, player};
use crate::input::{key_to_cameradir, key_to_movedir};
use crate::scenes::{PauseScene, Scene, SceneSwitch, Scenes};
use crate::systems::{get_player_inventory, get_player_money, hover_system, render_system};
use crate::utils::{position, CustomTexture, Layer, TILE_SIZE};
use crate::world::GameWorld;
use egui::{pos2, vec2, CtxRef, TextureId, Window};
use hecs::EntityBuilder;
use tetra::graphics::set_transform_matrix;
use tetra::input::{get_keys_down, get_keys_pressed, Key};
use tetra::window::get_size;
use tetra::{Context, Event};

mod camera;
use camera::move_camera;
mod tick;
use tick::tick;

#[derive(Debug)]
pub struct GameScene {
    width: i32,
    height: i32,
    pause: bool,
}

impl GameScene {
    pub fn new(world: &mut GameWorld, ctx: &mut Context) -> Self {
        let (width, height) = get_size(ctx);
        let (map_width, map_height) = (58, 28);

        generate_map(map_width, map_height, world);

        let position = Position::new(256, 256, Layer::Player as u32);
        let player = player();

        let mut builder = EntityBuilder::new();
        builder.add(position);
        builder.add_bundle(player);

        world.ecs.spawn(builder.build());

        world.camera.position = [width as f32 / 2.0, height as f32 / 2.0].into();
        world.camera.update();

        GameScene {
            pause: false,
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
            if let Some(dir) = key_to_cameradir(key) {
                move_camera(ctx, &mut world.camera, self, dir);
            }
        }

        for key in get_keys_pressed(ctx) {
            if let Some(mdir) = key_to_movedir(key) {
                if let Some(ss) = tick(ctx, world, mdir) {
                    return Ok(ss);
                }
            }
        }

        Ok(SceneSwitch::None)
    }

    fn draw(&mut self, world: &mut GameWorld, ctx: &mut Context, ectx: &mut CtxRef) -> tetra::Result {
        set_transform_matrix(ctx, world.camera.as_matrix());
        render_system(ctx, world);
        hover_system(ctx, ectx, &world.ecs, &world.camera);

        let pos = world.camera.project([100.0, 100.0].into());

        let rect = position(pos2(pos.x, pos.y), vec2(150.0, 100.0));
        Window::new("Information")
            .resizable(false)
            .fixed_rect(rect)
            .show(ectx, |ui| {
                ui.horizontal(|ui| {
                    ui.image(TextureId::User(CustomTexture::Gold.into()), vec2(32.0, 32.0));
                    let money = get_player_money(world);
                    ui.label(format!("Money: {}", money));
                });

                ui.separator();

                for item in get_player_inventory(world).items.iter() {
                    ui.horizontal(|ui| {
                        let texture: CustomTexture = item.name.into();
                        ui.image(TextureId::User(texture.into()), vec2(32.0, 32.0));
                        ui.label(format!("{}: {}", item.name.to_string(), item.amount));
                    });
                }
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
