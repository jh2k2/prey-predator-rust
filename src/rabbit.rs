use rand::Rng;

pub struct Rabbit {
    pub x_coord: f32,
    pub y_coord: f32,
    pub speed: f32,
    pub size: f32,
    pub acceleration: f32,
    pub x_direction: f32,
    pub y_direction: f32,
    pub state: i32,
    pub detection_range: f32,
    pub num_move: f32,
}

impl Rabbit {
    pub fn new(x: f32, y: f32) -> Rabbit {
        Rabbit {
            x_coord: x,
            y_coord: y,
            speed: 3.0,
            size: 1.0,
            acceleration: 0.2,
            x_direction: 0.0,
            y_direction: 0.0,
            state: 1,
            num_move: 0.0,
            detection_range: 10.0
        }
    }

    pub fn determine_direction(&mut self, fox_vec: &Vec<(f32, f32)>) -> () {
        //TODO obstacle

        //Target check
        if self.state == 1 {
            let mut fox_distance: Vec<(f32, f32, f32)> = fox_vec
            .iter()
            .map(|x| (((self.x_coord - x.0).powf(2.0) + (self.y_coord - x.1).powf(2.0)).sqrt().abs(), x.0, x.1))
            .collect();
            
            fox_distance.sort_by(|a, b| a.partial_cmp(b).unwrap());

            if fox_distance.len() != 0 && fox_distance[0].0 <= self.detection_range {
                //go toward target
                match &self.x_coord - fox_distance[0].1 {
                    y if y < 0.0 => self.x_direction = -1.0,
                    y if y > 0.0 => self.x_direction = 1.0,
                    _ => self.x_direction = 0.0,
                }

                match &self.y_coord - fox_distance[0].2 {
                    y if y < 0.0 => self.y_direction = -1.0,
                    y if y > 0.0 => self.y_direction = 1.0,
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
        self.x_coord += self.x_direction;
        self.y_coord += self.y_direction;
    }

    pub fn border_check(&mut self, border: &(f32, f32, f32, f32)) {
        //border check for x
        if self.x_coord + self.x_direction < border.0 ||  self.x_coord + self.x_direction >= border.2 {
            self.x_coord -= self.x_direction;
            self.num_move = 0.0;
        }
        //border check for y
        if self.y_coord + self.y_direction < border.1 || self.y_coord + self.y_direction >= border.3 {
            self.y_coord -= self.y_direction;
            self.num_move = 0.0;
        }
    }
}