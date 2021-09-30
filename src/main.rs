use cgmath::{Deg, Rad, Rotation, Rotation2};
use macroquad::prelude::*;
use snake::Snake;
mod snake;

#[macroquad::main("BasicShapes")]
async fn main() {
    let zoom = 1.0;
    let location = vec2(0.0, 0.0);
    let mut dir = vec2(0.1, 0.0);
    loop {
        set_camera(&Camera2D {
            zoom: vec2(zoom, screen_width() / screen_height()),
            ..Default::default()
        });
        //let deg: Deg<f32> = Deg(180.0);

        //let bla: = Rotation2::from_angle(Rad(0.5f64 * std::f64::consts::PI))

        //bla.rotate_vetor();

        if is_key_down(KeyCode::Right) {
            dir = dir + vec2(0.1, 0.1);
        }
        draw_line(location.x, location.y, dir.x, dir.y, 0.01, BLUE);

        set_default_camera();
        next_frame().await
    }
}
