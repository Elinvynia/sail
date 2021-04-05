#![warn(missing_debug_implementations)]

use crate::state::MainState;
use tetra::ContextBuilder;

mod components;
mod egui;
mod entities;
mod input;
mod scenes;
mod state;
mod systems;
mod utils;
mod world;

pub fn get_assets_folder() -> String {
    "assets/".into()
}

fn main() -> tetra::Result {
    dotenv::dotenv().expect("Failed to setup dotenv.");
    env_logger::init();

    ContextBuilder::new("Sail", 1280, 720)
        .show_mouse(true)
        .build()?
        .run(MainState::new)
}
