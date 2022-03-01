use macroquad::prelude::*;
mod game;
mod snake;

#[macroquad::main("zatake")]
async fn main() {
    let zoom = 1.0;
    let mut game = game::init_game(100);

    loop {
        //clear_background(RED);
        set_camera(&Camera2D {
            zoom: vec2(zoom, zoom),
            ..Default::default()
        });
       // set_default_camera();

        game.render();

        set_default_camera();
        next_frame().await
    }
}
