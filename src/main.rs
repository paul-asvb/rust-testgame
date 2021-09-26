use cgmath::{num_traits::ToPrimitive, InnerSpace, Vector2};
use macroquad::{
    color::{hsl_to_rgb, rgb_to_hsl},
    input,
    prelude::*,
};

fn random_color() -> Color {
    let random_hue = rand::gen_range(0, 100).to_f32().unwrap() / 100.0;
    hsl_to_rgb(random_hue, 0.5, 0.5)
}

fn check_edges(v: &mut Vector2<f32>, width: f32, height: f32) {
    if v.x > width {
        v.x = 0.0;
    } else if v.x < 0.0 {
        v.x = width;
    }

    if v.y > height {
        v.y = 0.0;
    } else if v.y < 0.0 {
        v.y = height;
    }
}

struct Mover {
    location: Vector2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
    limit: Vector2<f32>,
}

struct Part {
    location: Vector2<f32>,
    direction: Vector2<f32>,
    color: Color,
}

struct Snake {
    parts: Vec<Part>,
}

fn new_snake() -> Snake {
    let topspeed = 1;
    let start_position = Vector2 { x: 300.0, y: 300.0 };

    let start_direction = Vector2 {
        x: rand::gen_range(-topspeed, topspeed).to_f32().unwrap() / 100.0,
        y: rand::gen_range(-topspeed, topspeed).to_f32().unwrap() / 100.0,
    };

    let first_part = Part {
        location: start_position,
        direction: start_direction,
        color: random_color(),
    };
    Snake {
        parts: vec![first_part],
    }
}

impl Snake {
    fn add(&mut self, width: f32, height: f32) {
        let last_part = &self.parts[self.parts.len() - 1];
        let topspeed = 30;
        let rand_direction = Vector2 {
            x: rand::gen_range(-topspeed, topspeed).to_f32().unwrap(),
            y: rand::gen_range(-topspeed, topspeed).to_f32().unwrap(),
        };
        let mut new_location = last_part.location + last_part.direction;

        check_edges(&mut new_location, width, height);

        let mut new_color = rgb_to_hsl(last_part.color);

        new_color.0 = new_color.0 + 0.01;
        if new_color.0 > 99.0 {
            new_color.0 = 0.0;
        }

        self.parts.push(Part {
            location: new_location,
            direction: rand_direction,
            color: hsl_to_rgb(new_color.0, new_color.1, new_color.2),
        })
    }

    fn draw(&self) {
        for part in &self.parts {
            draw_line(
                part.location.x,
                part.location.y,
                part.location.x + part.direction.x,
                part.location.y + part.direction.y,
                12.0,
                part.color,
            )
        }
    }
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
        let topspeed = 50;
        self.acceleration = Vector2 {
            x: rand::gen_range(-topspeed, topspeed).to_f32().unwrap() / 100.0,
            y: rand::gen_range(-topspeed, topspeed).to_f32().unwrap() / 100.0,
        };

        self.velocity = self.velocity + self.acceleration;
        if self.velocity.x > self.limit.x || self.velocity.y > self.limit.y {
            self.velocity = self.limit
        }
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
        velocity: Vector2 { x: 0.01, y: 0.01 },
        acceleration: Vector2 { x: 0.01, y: 0.01 },
        limit: Vector2 { x: 10.01, y: 10.01 },
    };

    let mut snake = new_snake();

    loop {
        snake.add(screen_width(), screen_height());
        snake.draw();

        //clear_background(RED);
        let center = Vector2 {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
        };

        let mut mouse_position = Vector2 {
            x: input::mouse_position().0,
            y: input::mouse_position().1,
        };

        draw_line(
            center.x,
            center.y,
            mouse_position.x,
            mouse_position.y,
            20.0,
            BLUE,
        );

        // println!("{:?}", bla.overlaps(&bla1));

        // draw_rectangle(bla.x, bla.y, bla.w, bla.h, RED);

        mouse_position = mouse_position - center;
        mouse_position = mouse_position.normalize();
        mouse_position = mouse_position * 250.0;

        let norm = Vector2 {
            x: center.x + mouse_position.x,
            y: center.y + mouse_position.y,
        };

        draw_line(center.x, center.y, norm.x, norm.y, 10.0, GOLD);

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
