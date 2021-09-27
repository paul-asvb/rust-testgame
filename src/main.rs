use macroquad::prelude::*;
use snake::Snake;
mod snake;

#[macroquad::main("BasicShapes")]
async fn main() {
    let snakes: Vec<Snake> = [0..10].map(|e| -> Snake { snake::new_snake() }).to_vec();

    loop {
        for snake in &snakes {
            snake.add(screen_width(), screen_height());
            snake.draw();
        }
        next_frame().await
    }
}
