use crate::input::CameraDir;
use crate::scenes::GameScene;
use tetra::window::get_size;
use tetra::Context;

pub fn move_camera(ctx: &Context, scene: &mut GameScene, dir: CameraDir) {
    let (width, height) = get_size(ctx);
    match dir {
        CameraDir::Up => {
            let top = scene.camera.position.y - (height / 2) as f32;
            if top - 5.0 > 0.0 {
                scene.camera.position.y -= 5.0;
            }
        }
        CameraDir::Down => {
            let bottom = scene.camera.position.y + (height / 2) as f32;
            if bottom + 5.0 < scene.height as f32 {
                scene.camera.position.y += 5.0;
            }
        }
        CameraDir::Left => {
            let left = scene.camera.position.x - (width / 2) as f32;
            if left - 5.0 > 0.0 {
                scene.camera.position.x -= 5.0;
            }
        }
        CameraDir::Right => {
            let right = scene.camera.position.x + (width / 2) as f32;
            if right + 5.0 < scene.width as f32 {
                scene.camera.position.x += 5.0;
            }
        }
    }

    scene.camera.update();
}
