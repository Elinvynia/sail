use crate::scenes::{Scene, SceneSwitch};
use crate::utils::center;
use crate::world::GameWorld;
use egui::*;
use log::info;
use tetra::input::Key;
use tetra::{Context, Event};

// The trade menu, for exchanging goods with islands..
#[derive(Debug, Default)]
pub struct TradeScene {
    resume: bool,
}

impl TradeScene {
    pub fn new() -> Self {
        TradeScene::default()
    }
}

impl Scene for TradeScene {
    fn update(&mut self, _world: &mut GameWorld, _ctx: &mut Context) -> tetra::Result<SceneSwitch> {
        if self.resume {
            return Ok(SceneSwitch::Pop);
        }

        Ok(SceneSwitch::None)
    }

    fn draw(&mut self, _world: &mut GameWorld, ctx: &mut Context, ectx: &mut CtxRef) -> tetra::Result {
        let rect = center(ctx, vec2(250.0, 350.0));

        Window::new("Trade")
            .resize(|r| r.with_stroke(true))
            .title_bar(true)
            .collapsible(false)
            .resizable(false)
            .fixed_rect(rect)
            .show(ectx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.label("Trade menu W.I.P.");
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
