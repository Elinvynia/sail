use crate::world::GameWorld;
use egui::CtxRef;
use tetra::{Context, Event};

mod game;
pub use game::GameScene;
mod menu;
pub use menu::MenuScene;
mod options;
pub use options::OptionsScene;
mod pause;
pub use pause::PauseScene;

// Holds the stack of scenes and takes care of actual logic.
pub struct SceneStack {
    pub(crate) world: GameWorld,
    pub(crate) scenes: Vec<Scenes>,
}

impl SceneStack {
    pub fn new(world: GameWorld, initial: Scenes) -> Self {
        Self {
            world,
            scenes: vec![initial],
        }
    }

    fn push(&mut self, scene: Scenes) {
        self.scenes.push(scene)
    }

    fn pop(&mut self) -> Scenes {
        self.scenes.pop().expect("Tried to pop an empty scene stack.")
    }

    // Recursively pass through all scenes that allow drawing previous.
    // Then draw them in order from oldest to newest.
    fn draw_scenes(
        scenes: &mut [Scenes],
        world: &mut GameWorld,
        ctx: &mut Context,
        ectx: &mut CtxRef,
    ) -> tetra::Result {
        let (current, rest) = scenes.split_last_mut().expect("No scenes exist.");
        if current.draw_previous() {
            SceneStack::draw_scenes(rest, world, ctx, ectx)?;
        }
        current.draw(world, ctx, ectx)?;
        Ok(())
    }

    // Handles drawing the scenes.
    pub fn draw(&mut self, ctx: &mut Context, ectx: &mut CtxRef) -> tetra::Result {
        SceneStack::draw_scenes(&mut self.scenes, &mut self.world, ctx, ectx)?;
        Ok(())
    }

    // Handles updating the actual scene logic.
    pub fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        let change = {
            let current_scene = self.scenes.last_mut().expect("Tried to update empty scene stack.");
            current_scene.update(&mut self.world, ctx)?
        };
        match change {
            SceneSwitch::None => {}
            SceneSwitch::Push(s) => self.push(s),
            SceneSwitch::Pop => {
                let _ = self.pop();
            }
            SceneSwitch::Replace(s) => {
                let _ = self.pop();
                self.push(s);
            }
            SceneSwitch::ReplaceAll(s) => {
                for _ in 0..self.scenes.len() {
                    let _ = self.pop();
                }
                self.push(s);
            }
        };
        Ok(())
    }

    // Handles events like pressing keys and mouse buttons.
    pub fn event(&mut self, ctx: &mut Context, event: Event) -> tetra::Result {
        let current_scene = self.scenes.last_mut().expect("Tried to do input for empty scene stack");
        current_scene.event(&mut self.world, ctx, event)?;
        Ok(())
    }
}

// This is what you have to implement for a new scene.
pub trait Scene {
    fn update(&mut self, world: &mut GameWorld, ctx: &mut Context) -> tetra::Result<SceneSwitch>;
    fn draw(&mut self, world: &mut GameWorld, ctx: &mut Context, ectx: &mut CtxRef) -> tetra::Result;
    fn event(&mut self, world: &mut GameWorld, ctx: &mut Context, event: Event) -> tetra::Result;
    fn draw_previous(&self) -> bool {
        false
    }
}

// The result of calling scene.update()
#[allow(dead_code)]
pub enum SceneSwitch {
    None,
    Push(Scenes),
    Replace(Scenes),
    ReplaceAll(Scenes),
    Pop,
}

// This is a little boilerplate to avoid dynamic dispatch.
#[derive(Debug)]
pub enum Scenes {
    Menu(MenuScene),
    Game(GameScene),
    Options(OptionsScene),
    Pause(PauseScene),
}

// Failing to add a new scene here will result in a compilation error.
impl Scene for Scenes {
    fn update(&mut self, world: &mut GameWorld, ctx: &mut Context) -> tetra::Result<SceneSwitch> {
        match self {
            Scenes::Menu(s) => s.update(world, ctx),
            Scenes::Game(s) => s.update(world, ctx),
            Scenes::Options(s) => s.update(world, ctx),
            Scenes::Pause(s) => s.update(world, ctx),
        }
    }

    fn draw(&mut self, world: &mut GameWorld, ctx: &mut Context, ectx: &mut CtxRef) -> tetra::Result {
        match self {
            Scenes::Menu(s) => s.draw(world, ctx, ectx),
            Scenes::Game(s) => s.draw(world, ctx, ectx),
            Scenes::Options(s) => s.draw(world, ctx, ectx),
            Scenes::Pause(s) => s.draw(world, ctx, ectx),
        }
    }

    fn event(&mut self, world: &mut GameWorld, ctx: &mut Context, event: Event) -> tetra::Result {
        match self {
            Scenes::Menu(s) => s.event(world, ctx, event),
            Scenes::Game(s) => s.event(world, ctx, event),
            Scenes::Options(s) => s.event(world, ctx, event),
            Scenes::Pause(s) => s.event(world, ctx, event),
        }
    }

    fn draw_previous(&self) -> bool {
        match self {
            Scenes::Menu(s) => s.draw_previous(),
            Scenes::Game(s) => s.draw_previous(),
            Scenes::Options(s) => s.draw_previous(),
            Scenes::Pause(s) => s.draw_previous(),
        }
    }
}
