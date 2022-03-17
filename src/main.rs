use macroquad::{prelude::*, ui::root_ui};
mod game;
mod snake;

#[macroquad::main("zatake")]
async fn main() {
    //let zoom = 1000.0;
    //let mut game = game::init_game(100);
    let mut y = 100.0;

    loop {
        //clear_background(RED);
        // set_camera(&Camera2D {
        //     zoom: vec2(zoom, zoom),
        //     ..Default::default()
        // });
        root_ui().label(None, "hello megaui");

        //clear_background(RED);

        draw_line(50.0, y, 250.0, y, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);
        y = y + 1.0;
        //game.render();

        //set_default_camera();
        next_frame().await
    }
}
