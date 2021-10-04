use cgmath::{
    num_traits::float, Basis2, Deg, Matrix2, Point2, Rad, Rotation, Rotation2, Vector1, Vector2,
};
use macroquad::prelude::*;
use snake::{new_snake, Snake};
mod snake;

#[macroquad::main("BasicShapes")]
async fn main() {
    let zoom = 1.0;
    let location = Vector2::new(0.0, 0.0);
    let mut dir = Vector2::new(0.05, 0.0);
    let mut snake = new_snake();

    loop {
        set_camera(&Camera2D {
            zoom: vec2(zoom, screen_width() / screen_height()),
            ..Default::default()
        });

        if is_key_down(KeyCode::Right) {
            let rot = Basis2::from_angle(Deg(-5.0));
            dir = rot.rotate_vector(dir);
        }

        if is_key_down(KeyCode::Left) {
            let rot = Basis2::from_angle(Deg(5.0));
            dir = rot.rotate_vector(dir);
        }
        snake.add(dir);

        snake.draw();

        draw_line(location.x, location.y, dir.x, dir.y, 0.01, BLUE);

        set_default_camera();
        next_frame().await
    }
}
