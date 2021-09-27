use cgmath::{num_traits::ToPrimitive, InnerSpace, Vector2};
use macroquad::{
    color::{hsl_to_rgb, rgb_to_hsl},
    input,
    prelude::*,
};

fn init_mover() -> Mover {
    Mover {
        location: Vector2 { x: 20.0, y: 20.0 },
        velocity: Vector2 { x: 0.01, y: 0.01 },
        acceleration: Vector2 { x: 0.01, y: 0.01 },
        limit: Vector2 { x: 10.01, y: 10.01 },
    };
}

struct Mover {
    location: Vector2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
    limit: Vector2<f32>,
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
