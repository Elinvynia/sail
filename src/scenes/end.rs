use crate::scenes::{MenuScene, Scene, SceneSwitch, Scenes};
use crate::utils::center;
use crate::world::GameWorld;
use egui::*;
use log::info;
use tetra::{Context, Event};

// The game over menu, should have some stats later on.
#[derive(Debug, Default)]
pub struct EndScene {
    quit: bool,
}

impl EndScene {
    pub fn new() -> Self {
        EndScene::default()
    }
}

impl Scene for EndScene {
    fn update(&mut self, world: &mut GameWorld, ctx: &mut Context) -> tetra::Result<SceneSwitch> {
        // Quitting from the gave gives us just the default main menu.
        if self.quit {
            return Ok(SceneSwitch::ReplaceAll(Scenes::Menu(MenuScene::new(world, ctx))));
        }

        Ok(SceneSwitch::None)
    }

    fn draw(&mut self, _world: &mut GameWorld, ctx: &mut Context, ectx: &mut CtxRef) -> tetra::Result {
        let rect = center(ctx, vec2(250.0, 350.0));

        Window::new("Game Over")
            .resize(|r| r.with_stroke(true))
            .title_bar(true)
            .collapsible(false)
            .resizable(false)
            .fixed_rect(rect)
            .show(ectx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.label("Unfortunately, you lost.");
                    ui.label("Thanks for playing, though!");

                    let quit = ui.add(Button::new("Quit"));
                    if quit.clicked() {
                        info!("Clicked quit");
                        self.quit = true;
                    }
                });
            });

        Ok(())
    }

    fn event(&mut self, _world: &mut GameWorld, _ctx: &mut Context, _event: Event) -> tetra::Result {
        Ok(())
    }
}
