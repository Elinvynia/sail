use crate::scenes::{Scene, SceneSwitch};
use crate::systems::get_island_inventory;
use crate::utils::{position, CustomTexture};
use crate::world::GameWorld;
use egui::*;
use hecs::Entity;
use log::info;
use tetra::input::Key;
use tetra::window::get_size;
use tetra::{Context, Event};

// The trade menu, for exchanging goods with islands..
#[derive(Debug)]
pub struct TradeScene {
    island: Entity,
    resume: bool,
}

impl TradeScene {
    pub fn new(entity: Entity) -> Self {
        TradeScene {
            island: entity,
            resume: false,
        }
    }
}

impl Scene for TradeScene {
    fn update(&mut self, _world: &mut GameWorld, _ctx: &mut Context) -> tetra::Result<SceneSwitch> {
        if self.resume {
            return Ok(SceneSwitch::Pop);
        }

        Ok(SceneSwitch::None)
    }

    fn draw(&mut self, world: &mut GameWorld, ctx: &mut Context, ectx: &mut CtxRef) -> tetra::Result {
        let (width, height) = get_size(ctx);
        let pos = world.camera.project([(width / 2) as f32, (height / 2) as f32].into());
        let rect = position(pos2(pos.x, pos.y), vec2(250.0, 350.0));

        Window::new("Trade")
            .resize(|r| r.with_stroke(true))
            .title_bar(true)
            .collapsible(false)
            .resizable(false)
            .fixed_rect(rect)
            .show(ectx, |ui| {
                for item in get_island_inventory(world, self.island).items.iter() {
                    ui.horizontal(|ui| {
                        let texture: CustomTexture = item.name.into();
                        ui.image(TextureId::User(texture.into()), vec2(32.0, 32.0));
                        ui.label(format!("{}: {}", item.name.to_string(), item.amount));
                    });
                }

                ui.vertical_centered_justified(|ui| {
                    let back = ui.add(Button::new("Back"));
                    if back.clicked() {
                        info!("Clicked back");
                        self.resume = true;
                    }
                });
            });

        Ok(())
    }

    fn event(&mut self, _world: &mut GameWorld, _ctx: &mut Context, event: Event) -> tetra::Result {
        if let Event::KeyPressed { key } = event {
            if key == Key::Escape {
                self.resume = true;
            }
        }

        Ok(())
    }

    fn draw_previous(&self) -> bool {
        true
    }
}
