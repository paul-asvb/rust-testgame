use cgmath::{num_traits::ToPrimitive, Basis2, Deg, Point2, Rotation, Rotation2, Vector2};
use macroquad::{color::hsl_to_rgb, prelude::*};

const SPEED: f32 = 0.01;
const THICKNESS: f32 = 0.01;
const HEAD_COLOR: Color = YELLOW;
const CURVINESS: f32 = 5.0;

fn check_edges(v: &mut Point2<f32>) {
    if v.x > 1.0 {
        v.x = -1.0;
    } else if v.x < -1.0 {
        v.x = 1.0;
    }

    if v.y > 1.0 {
        v.y = -1.0;
    } else if v.y < -1.0 {
        v.y = 1.0;
    }
}

fn random_color() -> Color {
    let random_hue = rand::gen_range(0, 100).to_f32().unwrap() / 100.0;
    hsl_to_rgb(random_hue, 0.5, 0.5)
}

#[derive(Clone, Debug)]
struct Part {
    location: Point2<f32>,
    direction: Vector2<f32>,
}

#[derive(Clone, Debug)]
pub struct Snake {
    speed: f32,
    direction: Vector2<f32>,
    parts: Vec<Part>,
    color: Color,
}

pub fn new_snake() -> Snake {
    let start_position = Point2 {
        x: rand::gen_range(-100, 100).to_f32().unwrap() / 100.0,
        y: rand::gen_range(-100, 100).to_f32().unwrap() / 100.0,
    };
    let start_direction = Vector2 {
        x: rand::gen_range(-SPEED, SPEED).to_f32().unwrap(),
        y: rand::gen_range(-SPEED, SPEED).to_f32().unwrap(),
    };

    //println!("{:?}", start_position);

    let first_part = Part {
        location: start_position,
        direction: start_direction,
    };
    Snake {
        color: random_color(),
        parts: vec![first_part],
        speed: SPEED,
        direction: start_direction,
    }
}

impl Snake {
    pub fn add(&mut self) {
        let last_part = &self.parts[self.parts.len() - 1];

        let mut new_location = last_part.location + last_part.direction;

        check_edges(&mut new_location);

        self.parts.push(Part {
            location: new_location,
            direction: self.direction,
        })
    }

    pub fn render(&self) {
        for part in &self.parts {
           draw_circle(part.location.x, part.location.y, THICKNESS, self.color)
        }

        let last_part = &self.parts[self.parts.len() - 1];
        draw_circle(
            last_part.location.x,
            last_part.location.y,
            THICKNESS,
            HEAD_COLOR,
        )
    }

    pub fn right(&mut self) {
        self.direction = Basis2::from_angle(Deg(-CURVINESS)).rotate_vector(self.direction);
    }

    pub fn left(&mut self) {
        self.direction = Basis2::from_angle(Deg(CURVINESS)).rotate_vector(self.direction);
    }
}
