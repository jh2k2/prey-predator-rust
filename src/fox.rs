pub struct Fox {
    pub x_coord: i32,
    pub y_coord: i32,
}

impl Fox {
    pub fn new(x: i32, y: i32) -> Fox {
        Fox {
            x_coord: x,
            y_coord: y
        }
    }
}