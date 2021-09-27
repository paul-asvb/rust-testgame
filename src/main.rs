use macroquad::prelude::*;
use snake::Snake;
mod snake;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut snakes: Vec<Snake> = (0..10)
        .map(|_| -> Snake { snake::new_snake(screen_width(), screen_height()) })
        .collect::<Vec<Snake>>();
    println!("{:?}", snakes.len());
    loop {
        snakes.iter_mut().for_each(|snake| {
            snake.add(screen_width(), screen_height());
            snake.draw();
        });

        next_frame().await
    }
}
