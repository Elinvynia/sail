use crate::scenes::{Scene, SceneSwitch};
use crate::world::GameWorld;
use egui::*;
use log::info;
use tetra::{Context, Event};

#[derive(Debug, Default)]
pub struct OptionsScene {
    back_clicked: bool,
}

impl OptionsScene {
    pub fn new(_world: &mut GameWorld, _ctx: &mut Context) -> Self {
        OptionsScene::default()
    }
}

impl Scene for OptionsScene {
    fn update(&mut self, _world: &mut GameWorld, _ctx: &mut Context) -> tetra::Result<SceneSwitch> {
        if self.back_clicked {
            return Ok(SceneSwitch::Pop);
        }

        Ok(SceneSwitch::None)
    }

    fn draw(&mut self, _world: &mut GameWorld, ctx: &mut Context, ectx: &mut CtxRef) -> tetra::Result {
        let size = tetra::window::get_size(ctx);
        let rect = Rect::from_center_size(pos2((size.0 / 2) as f32, (size.1 / 2) as f32), vec2(200.0, 200.0));
        Window::new("Options")
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
                        self.back_clicked = true;
                    }
                });
            });

        Ok(())
    }

    fn event(&mut self, _world: &mut GameWorld, _ctx: &mut Context, _event: Event) -> tetra::Result {
        Ok(())
    }
}
