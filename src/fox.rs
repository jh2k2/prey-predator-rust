//use rand::Rng;

pub struct Fox {
    pub x_coord: i32,
    pub y_coord: i32,
    pub speed: i32
}

impl Fox {
    pub fn new(x: i32, y: i32) -> Fox {
        Fox {
            x_coord: x,
            y_coord: y,
            speed: 1
        }
    }

    pub fn find_prey(&mut self, rabbit_location: &Vec<(i32, i32)>) {
        let mut distance: Vec<(i32, i32, i32)> = rabbit_location
        .iter()
        .map(|x| ((((self.x_coord - x.0).pow(2) + (self.y_coord - x.1).pow(2)) as f64).sqrt() as i32, x.0, x.1))
        .collect();

        distance.sort();
        let target: (i32, i32) = (distance[0].1, distance[0].2);

        match &self.x_coord - target.0 {
            y if y < 0 => self.x_coord += self.speed,
            y if y > 0 => self.x_coord -= self.speed,
            _ => print!("")
        }

        match &self.y_coord - target.1 {
            y if y < 0 => self.y_coord += self.speed,
            y if y > 0 => self.y_coord -= self.speed,
            _ => print!("")
        }

    }
}