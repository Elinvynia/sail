use crate::scenes::{MenuScene, Scene, SceneSwitch, Scenes};
use crate::utils::position;
use crate::world::GameWorld;
use egui::{pos2, vec2, Button, CtxRef, Window};
use log::info;
use tetra::input::Key;
use tetra::window::get_size;
use tetra::{Context, Event};

// The pause menu, giving the player time to chill.
#[derive(Debug, Default)]
pub struct PauseScene {
    resume: bool,
    quit: bool,
}

impl PauseScene {
    pub fn new(_world: &mut GameWorld, _ctx: &mut Context) -> Self {
        PauseScene::default()
    }
}

impl Scene for PauseScene {
    fn update(&mut self, world: &mut GameWorld, ctx: &mut Context) -> tetra::Result<SceneSwitch> {
        if self.resume {
            return Ok(SceneSwitch::Pop);
        }

        // Quitting from the gave gives us just the default main menu.
        if self.quit {
            return Ok(SceneSwitch::ReplaceAll(Scenes::Menu(MenuScene::new(world, ctx))));
        }

        Ok(SceneSwitch::None)
    }

    fn draw(&mut self, world: &mut GameWorld, ctx: &mut Context, ectx: &mut CtxRef) -> tetra::Result {
        let (width, height) = get_size(ctx);
        let pos = world.camera.project([(width / 2) as f32, (height / 2) as f32].into());
        let rect = position(pos2(pos.x, pos.y), vec2(250.0, 350.0));

        Window::new("Pause")
            .resize(|r| r.with_stroke(true))
            .title_bar(true)
            .collapsible(false)
            .resizable(false)
            .fixed_rect(rect)
            .show(ectx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    let back = ui.add(Button::new("Back"));
                    if back.clicked() {
                        info!("Clicked back");
                        self.resume = true;
                    }

                    let quit = ui.add(Button::new("Quit"));
                    if quit.clicked() {
                        info!("Clicked quit");
                        self.quit = true;
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
