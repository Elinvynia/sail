use crate::egui::{handle_event, render_ui};
use crate::scenes::{MenuScene, SceneStack, Scenes};
use crate::world::GameWorld;
use egui::{CtxRef, RawInput};
use std::collections::HashMap;
use tetra::graphics::mesh::Mesh;
use tetra::graphics::{clear, Color};
use tetra::time::get_delta_time;
use tetra::{Context, Event, State};

// Semi-mutable global state, handles switching scenes.
pub struct MainState {
    scenes: SceneStack,
    egui: CtxRef,
    egui_cache: HashMap<String, Mesh>,
    input: RawInput,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let mut world = GameWorld::new();
        let scene = MenuScene::new(&mut world, ctx);
        let scenes = SceneStack::new(world, Scenes::Menu(scene));

        let egui = CtxRef::default();
        let egui_cache = HashMap::new();
        let input = RawInput::default();

        Ok(MainState {
            scenes,
            egui,
            egui_cache,
            input,
        })
    }
}

// Wrapper for egui, otherwise it just simply passes calls into the SceneStack.
impl State for MainState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        clear(ctx, Color::rgb(0.8, 0.8, 0.95));

        let new = match &mut self.input.time {
            Some(prev) => Some(*prev + get_delta_time(ctx).as_secs_f64()),
            None => Some(get_delta_time(ctx).as_secs_f64()),
        };

        self.input.time = new;
        self.egui.begin_frame(self.input.take());
        self.input.time = new;
        self.scenes.draw(ctx, &mut self.egui)?;
        let (_output, shapes) = self.egui.end_frame();
        render_ui(ctx, &mut self.egui, &mut self.egui_cache, shapes);

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.scenes.update(ctx)?;
        Ok(())
    }

    fn event(&mut self, ctx: &mut Context, event: Event) -> tetra::Result {
        handle_event(ctx, &mut self.input, &event);
        self.scenes.event(ctx, event)?;
        Ok(())
    }
}
