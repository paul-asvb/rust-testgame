use cgmath::Vector2;
use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut position = Vector2 { x: 20.0, y: 20.0 };
    let mut speed = Vector2 { x: 10.0, y: 10.0 };
    const SIZE: f32 = 20.0;

    loop {
        //clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        position = position + speed;

        if (position.x > screen_width() - SIZE) || (position.x < SIZE) {
            speed.x = speed.x * -1.0;
        }
        if (position.y > screen_height() - SIZE) || (position.y < SIZE) {
            speed.y = speed.y * -1.0;
        }

        draw_circle(position.x, position.y, SIZE, YELLOW);

        draw_text("press enter!", 20.0, 20.0, 30.0, DARKGRAY);
        if is_key_down(KeyCode::Right) {
            draw_line(20.0, 20.0, 120.0, 100.0, 15.0, BROWN);
        }
        if is_key_down(KeyCode::Enter) {
            draw_text("IT WORKS!", 220.0, 220.0, 30.0, RED);
        }
        next_frame().await
    }
}
