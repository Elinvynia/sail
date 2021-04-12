use crate::scenes::{GameScene, OptionsScene, Scene, SceneSwitch, Scenes};
use crate::utils::center;
use crate::world::GameWorld;
use egui::*;
use log::info;
use tetra::{Context, Event};

// The main menu of the game.
#[derive(Debug, Default)]
pub struct MenuScene {
    play_clicked: bool,
    options_clicked: bool,
    quit_clicked: bool,
}

impl MenuScene {
    pub fn new(_world: &mut GameWorld, _ctx: &mut Context) -> Self {
        MenuScene::default()
    }
}

impl Scene for MenuScene {
    fn update(&mut self, world: &mut GameWorld, ctx: &mut Context) -> tetra::Result<SceneSwitch> {
        if self.play_clicked {
            self.play_clicked = false;

            let scene = GameScene::new(world, ctx);
            return Ok(SceneSwitch::Push(Scenes::Game(scene)));
        };

        if self.options_clicked {
            self.options_clicked = false;

            let scene = OptionsScene::new(world, ctx);
            return Ok(SceneSwitch::Push(Scenes::Options(scene)));
        };

        if self.quit_clicked {
            tetra::window::quit(ctx);
        };

        Ok(SceneSwitch::None)
    }

    fn draw(&mut self, _world: &mut GameWorld, ctx: &mut Context, ectx: &mut CtxRef) -> tetra::Result {
        let rect = center(ctx, vec2(150.0, 150.0));

        Window::new("Sail")
            .title_bar(true)
            .collapsible(false)
            .resizable(false)
            .fixed_rect(rect)
            .show(ectx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    let play = ui.add(Button::new("Play"));
                    if play.clicked() {
                        info!("Play clicked");
                        self.play_clicked = true;
                    };

                    let options = ui.add(Button::new("Options"));
                    if options.clicked() {
                        info!("Options clicked");
                        self.options_clicked = true;
                    }

                    let quit = ui.add(Button::new("Quit"));
                    if quit.clicked() {
                        info!("Quit clicked");
                        self.quit_clicked = true;
                    }
                });
            });

        Ok(())
    }

    fn event(&mut self, _world: &mut GameWorld, _ctx: &mut Context, _event: Event) -> tetra::Result {
        Ok(())
    }
}
