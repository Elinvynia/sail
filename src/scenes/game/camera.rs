use crate::input::CameraDir;
use crate::scenes::GameScene;
use tetra::graphics::Camera;
use tetra::window::get_size;
use tetra::Context;

pub fn move_camera(ctx: &Context, camera: &mut Camera, scene: &GameScene, dir: CameraDir) {
    let (width, height) = get_size(ctx);
    match dir {
        CameraDir::Up => {
            let top = camera.position.y - (height / 2) as f32;
            if top - 5.0 > 0.0 {
                camera.position.y -= 5.0;
            }
        }
        CameraDir::Down => {
            let bottom = camera.position.y + (height / 2) as f32;
            if bottom + 5.0 < scene.height as f32 {
                camera.position.y += 5.0;
            }
        }
        CameraDir::Left => {
            let left = camera.position.x - (width / 2) as f32;
            if left - 5.0 > 0.0 {
                camera.position.x -= 5.0;
            }
        }
        CameraDir::Right => {
            let right = camera.position.x + (width / 2) as f32;
            if right + 5.0 < scene.width as f32 {
                camera.position.x += 5.0;
            }
        }
    }

    camera.update();
}
