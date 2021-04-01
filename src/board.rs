use crate::rabbit::{Rabbit};
use crate::fox::{Fox};
use crate::traits::Traits;
use rand::Rng;

pub struct Board {
    pub rabbit: Vec<Rabbit>,
    pub fox: Vec<Fox>,
    pub occupied_coord: Vec<(char, f32, f32)>,
    pub border: (f32, f32, f32, f32)
}

impl Board {
    pub fn new(rabbit_count: u32, fox_count: u32, width: f32, height: f32) -> Board {
        let mut rng = rand::thread_rng();
        let mut occupied_coord: Vec<(char, f32, f32)> = Vec::new();
        let rabbit_vec: Vec<Rabbit> = (0..rabbit_count)
        .map(|_x| {
            let x_coord = rng.gen_range(50..width as i32-50) as f32;
            let y_coord = rng.gen_range(50..height as i32-50) as f32;
            occupied_coord.push(('r', x_coord, y_coord));
            Rabbit::new(x_coord, y_coord)
        })
        .collect();

        let fox_vec: Vec<Fox> = (0..fox_count)
        .map(|_x| { 
            let x_coord = rng.gen_range(50..width as i32-50) as f32;
            let y_coord = rng.gen_range(50..height as i32-50) as f32;
            occupied_coord.push(('f', x_coord, y_coord));
            Fox::new(x_coord, y_coord)
        })
        .collect();

        Board {
            rabbit: rabbit_vec,
            fox: fox_vec,
            occupied_coord: occupied_coord,
            border: (50.0, 50.0, width-50.0, height-50.0)
        }
    }

    pub fn fox_move(&mut self) {
        let fox_vec = &self.fox;

        let rabbit_vec:Vec<(f32, f32)> = self.rabbit.iter().map(|x| (x.x_coord, x.y_coord)).collect();
        for i in 0..fox_vec.len() {
            self.fox[i].update_stats();
            for _i in 0..self.fox[i].speed as i32 {
                self.fox[i].determine_state();
                self.fox[i].determine_direction(&rabbit_vec);
                self.fox[i].border_check(&self.border);
                self.fox[i].moves();
            }
        }
    }

    pub fn rabbit_move(&mut self) {
        let rabbit_vec = &self.rabbit;
        let fox_vec:Vec<(f32, f32)> = self.fox.iter().map(|x| (x.x_coord, x.y_coord)).collect();
        for i in 0..rabbit_vec.len() {
            for _i in 0..self.rabbit[i].speed as i32 {
                self.rabbit[i].determine_direction(&fox_vec);
                self.rabbit[i].border_check(&self.border);
                self.rabbit[i].moves();
            }
        }
    }

    pub fn check_eaten(&mut self) {
        //TODO CHECK FOR COLLISION
        let fox_vec = &self.fox;
        for i in 0..fox_vec.len() {
            let index: Option<usize> = self.rabbit.iter().position(|r| r.x_coord == self.fox[i].x_coord && r.y_coord == self.fox[i].y_coord);
            match index {
                Some(y) => {
                    self.rabbit.remove(y);
                    self.fox[i].hunger += 20.0;
                },
                None => ()
            };
        }
    }

    pub fn check_death(&mut self) {
        let fox_vec = &self.fox;
        for i in 0..fox_vec.len() {
            if self.fox[i].state == -1 {
                self.fox.remove(i);
            }
        }
    }
    
}
