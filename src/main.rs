use cgmath::{InnerSpace, Vector2};
use macroquad::{input, prelude::*};

struct Mover {
    location: Vector2<f32>,
    velocity: Vector2<f32>,
}

impl Mover {
    fn check_edges(&mut self, width: f32, height: f32) {
        if self.location.x > width {
            self.location.x = 0.0;
        } else if self.location.x < 0.0 {
            self.location.x = width;
        }

        if self.location.y > height {
            self.location.y = 0.0;
        } else if self.location.y < 0.0 {
            self.location.y = height;
        }
    }

    fn update(&mut self) {
        self.location = self.location + self.velocity;
    }

    fn display(&self) {
        draw_circle(self.location.x, self.location.y, 30.0, PINK);
    }
}
#[macroquad::main("BasicShapes")]
async fn main() {
    let mut position = Vector2 { x: 20.0, y: 20.0 };
    let mut speed = Vector2 { x: 10.0, y: 10.0 };
    const SIZE: f32 = 20.0;
    let mut my_mover = Mover {
        location: Vector2 { x: 20.0, y: 20.0 },
        velocity: Vector2 { x: 10.0, y: 10.0 },
    };

    loop {
        //clear_background(RED);
        let center = Vector2 {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
        };

        let mouse_position = Vector2 {
            x: input::mouse_position().0,
            y: input::mouse_position().1,
        };

        draw_line(
            center.x,
            center.y,
            mouse_position.x,
            mouse_position.y,
            15.0,
            BLUE,
        );

        let norm = Vector2 {
            x: center.x + mouse_position.x,
            y: center.x + mouse_position.y,
        };

        draw_line(center.x + 20.0, center.y + 20.0, norm.x, norm.y, 10.0, GOLD);

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

        my_mover.update();
        my_mover.check_edges(screen_width(), screen_height());
        my_mover.display();
        next_frame().await
    }
}
