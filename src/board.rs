use crate::rabbit::{Rabbit};
use crate::fox::{Fox};
use rand::Rng;

pub struct Board {
    pub rabbit: Vec<Rabbit>,
    pub fox: Vec<Fox>,
    pub occupied_coord: Vec<(char, i32, i32)>
}

impl Board {
    pub fn new(rabbit_count: u32, fox_count: u32, height: i32, width: i32) -> Board {
        let mut rng = rand::thread_rng();
        let mut occupied_coord: Vec<(char, i32, i32)> = Vec::new();
        let rabbit_vec: Vec<Rabbit> = (0..rabbit_count)
        .map(|_x| {
            let x_coord = rng.gen_range(0..width);
            let y_coord = rng.gen_range(0..height);
            occupied_coord.push(('r', x_coord, y_coord));
            Rabbit::new(x_coord, y_coord)
        })
        .collect();

        let fox_vec: Vec<Fox> = (0..fox_count)
        .map(|_x| { 
            let x_coord = rng.gen_range(0..width);
            let y_coord = rng.gen_range(0..height);
            occupied_coord.push(('f', x_coord, y_coord));
            Fox::new(x_coord, y_coord)
        })
        .collect();

        Board {
            rabbit: rabbit_vec,
            fox: fox_vec,
            occupied_coord: occupied_coord
        }
    }

    pub fn check_eaten(&mut self) {
        for fox in &self.fox {
            let index: Option<usize> = self.rabbit.iter().position(|r| r.x_coord == fox.x_coord && r.y_coord == fox.y_coord);
            match index {
                Some(y) => {
                    println!("Rabbit removed! Size: {}", self.rabbit.len());
                    self.rabbit.remove(y)
                },
                None => Rabbit::new(1, 1)
            };
        }
    }
    
}
