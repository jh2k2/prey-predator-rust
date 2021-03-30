//use rand::Rng;

pub struct Rabbit {
    pub x_coord: f32,
    pub y_coord: f32,
    pub speed: f32,
    pub size: f32,
    pub acceleration: f32,
    pub x_direction: f32,
    pub y_direction: f32,
}

impl Rabbit {
    pub fn new(x: f32, y: f32) -> Rabbit {
        Rabbit {
            x_coord: x,
            y_coord: y,
            speed: 1.0,
            size: 1.0,
            acceleration: 0.2,
            x_direction: 0.0,
            y_direction: 0.0,
        }
    }
}