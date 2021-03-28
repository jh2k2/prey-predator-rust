pub struct Rabbit {
    pub x_coord: i32,
    pub y_coord: i32,
}

impl Rabbit {
    pub fn new(x: i32, y: i32) -> Rabbit {
        Rabbit {
            x_coord: x,
            y_coord: y
        }
    }
}