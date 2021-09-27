use cgmath::{num_traits::ToPrimitive, Vector2};
use macroquad::{
    color::{hsl_to_rgb, rgb_to_hsl},
    prelude::*,
};

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

fn random_color() -> Color {
    let random_hue = rand::gen_range(0, 100).to_f32().unwrap() / 100.0;
    hsl_to_rgb(random_hue, 0.5, 0.5)
}
#[derive(Clone, Debug)]
struct Part {
    location: Vector2<f32>,
    direction: Vector2<f32>,
    color: Color,
}
#[derive(Clone, Debug)]
pub struct Snake {
    parts: Vec<Part>,
}

pub fn new_snake(width: f32, height: f32) -> Snake {
    let topspeed = 1;

    let start_position = Vector2 {
        x: rand::gen_range(0, height as i64).to_f32().unwrap(),
        y: rand::gen_range(0, width as i64).to_f32().unwrap(),
    };
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
    pub fn add(&mut self, width: f32, height: f32) {
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

    pub fn draw(&self) {
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
