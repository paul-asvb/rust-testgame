use macroquad::prelude::{is_key_down, KeyCode};

use crate::snake;
pub struct Game {
    snakes: Vec<snake::Snake>,
}

pub fn init_game(number_of_snakes: i8) -> Game {
    Game {
        snakes: (0..number_of_snakes).map(|_| snake::new_snake()).collect(),
    }
}

impl Game {
    pub fn render(&mut self) {
        for snake in self.snakes.iter_mut() {
            if is_key_down(KeyCode::Right) {
                snake.right();
            }

            if is_key_down(KeyCode::Left) {
                snake.left();
            }

            snake.add();

            snake.render();
        }
    }
}
