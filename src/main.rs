use game::Game;
use macroquad::prelude::*;
mod game;
mod snake;

#[macroquad::main("BasicShapes")]
async fn main() {
    let zoom = 1.0;
    let game = game::init_game(2);
    game.update();

    loop {
        set_camera(&Camera2D {
            zoom: vec2(zoom, screen_width() / screen_height() * zoom),
            ..Default::default()
        });


        set_default_camera();
        next_frame().await
    }
}
