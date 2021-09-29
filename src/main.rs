use macroquad::prelude::*;
use snake::Snake;
mod snake;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut snakes: Vec<Snake> = (0..10)
        .map(|_| -> Snake { snake::new_snake() })
        .collect::<Vec<Snake>>();
    let zoom = 1.0;
    loop {
        set_camera(&Camera2D {
            zoom: vec2(zoom, screen_width() / screen_height()),
            ..Default::default()
        });

        snakes.iter_mut().for_each(|snake| {
            snake.add();
            snake.draw();
        });

        set_default_camera();
        next_frame().await
    }
}
