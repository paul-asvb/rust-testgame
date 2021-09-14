use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut x: f32 = 100.0;
    let mut y: f32 = 100.0;
    let mut xspeed: f32 = 10.0;
    let mut yspeed: f32 = 10.0;
    const SIZE: f32 = 20.0;

    loop {
        //clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        x = x + xspeed;
        y = y + yspeed;

        if (x > screen_width() - SIZE) || (x < SIZE) {
            xspeed = xspeed * -1.0;
        }
        if (y > screen_height() - SIZE) || (y < SIZE) {
            yspeed = yspeed * -1.0;
        }

        draw_circle(x, y, SIZE, YELLOW);

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
