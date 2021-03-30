use rand::Rng;

pub struct Fox {
    pub x_coord: f32,
    pub y_coord: f32,
    pub speed: f32,
    pub size: f32,
    pub acceleration: f32,
    pub x_direction: f32,
    pub y_direction: f32,
    pub num_move: f32,
    pub detection_range: f32,
    pub state: i32 //0 = idle, 1 = hunt
}

impl Fox {
    pub fn new(x: f32, y: f32) -> Fox {
        Fox {
            x_coord: x,
            y_coord: y,
            speed: 1.0,
            size: 2.0,
            acceleration: 0.2,
            x_direction: 1.0,
            y_direction: 1.0,
            num_move: 0.0,
            detection_range: 30.0,
            state: 1,
        }
    }

    pub fn determine_direction(&mut self, rabbit_vec: &Vec<(f32, f32)>, border: &(f32, f32, f32, f32)) -> () {
        //border check
        //TODO obstacle

        //Target check
        if self.state == 1 {
            let mut rabbit_distance: Vec<(f32, f32, f32)> = rabbit_vec
            .iter()
            .map(|x| (((self.x_coord - x.0).powf(2.0) + (self.y_coord - x.1).powf(2.0)).sqrt().abs(), x.0, x.1))
            .collect();
            
            rabbit_distance.sort_by(|a, b| a.partial_cmp(b).unwrap());

            if rabbit_distance.len() != 0 && rabbit_distance[0].0 <= self.detection_range {
                //go toward target
                match &self.x_coord - rabbit_distance[0].1 {
                    y if y < 0.0 => self.x_direction = 1.0,
                    y if y > 0.0 => self.x_direction = -1.0,
                    _ => self.x_direction = 0.0,
                }

                match &self.y_coord - rabbit_distance[0].2 {
                    y if y < 0.0 => self.y_direction = 1.0,
                    y if y > 0.0 => self.y_direction = -1.0,
                    _ => self.y_direction = 0.0
                }
                self.num_move = 0.0;
                return ();
            }
        }

        if self.num_move > 0.0 { 
            self.num_move -= 1.0;
            return ();
        }

        let mut rng = rand::thread_rng();
        self.num_move = rng.gen_range(4..12) as f32;

        //randomize direction
        self.x_direction = rng.gen_range(-1..2) as f32;
        self.y_direction = rng.gen_range(-1..2) as f32;
    }

    pub fn moves(&mut self) {
        self.x_coord += self.speed * self.x_direction;
        self.y_coord += self.speed * self.y_direction;
    }
}