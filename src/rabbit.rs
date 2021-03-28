pub struct Rabbit {
    pub x_coord: i32,
    pub y_coord: i32,
    pub speed: i32
}

impl Rabbit {
    pub fn new(x: i32, y: i32) -> Rabbit {
        Rabbit {
            x_coord: x,
            y_coord: y,
            speed: 1
        }
    }

    pub fn run_away(&mut self, fox_location: &Vec<(i32, i32)>, border: (i32, i32)) {
        let mut distance: Vec<(i32, i32, i32)> = fox_location
        .iter()
        .map(|x| ((((self.x_coord - x.0).pow(2) + (self.y_coord - x.1).pow(2)) as f64).sqrt() as i32, x.0, x.1))
        .collect();

        distance.sort();
        let target: (i32, i32) = (distance[distance.len() - 1].1, distance[distance.len() - 1].2);
        if self.x_coord < border.0 && self.x_coord > 0{
            match &self.x_coord - target.0 {
                y if y < 0 => {
                    self.x_coord -= self.speed
                },
                y if y > 0 => {
                    self.x_coord += self.speed
                },
                _ => print!("")
            }
        }

        if self.y_coord < border.1 && self.y_coord > 0 {
            match &self.y_coord - target.1 {
                y if y < 0 => {
                    self.y_coord -= self.speed
                },
                y if y > 0 => {
                    self.y_coord += self.speed
                },
                _ => print!("")
            }
        }
    }
}