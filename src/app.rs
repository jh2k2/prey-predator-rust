use opengl_graphics::GlGraphics;
use piston::input::*;
use crate::board::{Board};

pub struct App {
    gl: GlGraphics,
    board: Board,
    width: i32,
    height: i32
}

impl App {
    pub fn new(graphic: GlGraphics, width: i32, height: i32) -> App {
        App {
            gl: graphic,
            board: Board::new(15, 1, width, height),
            width: width,
            height: height
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const FOX_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RABBIT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let rabbit_vec = &self.board.rabbit;
        let fox_vec = &self.board.fox;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);

            //draw rabbit
            for i in 0..rabbit_vec.len()  {
                let trans = c.transform.trans(
                        rabbit_vec[i].x_coord as f64,
                        rabbit_vec[i].y_coord as f64,
                    );
                ellipse(RABBIT_COLOR, [0.0, 0.0, 5.0, 5.0], trans, gl);
            }

            //draw fox
             for i in 0..fox_vec.len() {
                 let trans = c.transform.trans(
                        fox_vec[i].x_coord as f64,
                        fox_vec[i].y_coord as f64,
                    );
                ellipse(FOX_COLOR, [0.0, 0.0, 5.0, 5.0], trans, gl);
            }
        });
    }

    pub fn update(&mut self) {
        //rabbit
        let rabbit_vec = &self.board.rabbit;
        let fox_vec:Vec<(i32, i32)> = self.board.fox.iter().map(|x| (x.x_coord, x.y_coord)).collect();
        for i in 0..rabbit_vec.len() {
            self.board.rabbit[i].run_away(&fox_vec, (self.width, self.height));
        }

         //fox
        let fox_vec = &self.board.fox;
        let rabbit_vec:Vec<(i32, i32)> = self.board.rabbit.iter().map(|x| (x.x_coord, x.y_coord)).collect();
        for i in 0..fox_vec.len() {
            self.board.fox[i].find_prey(&rabbit_vec);
        }

        //board
        self.board.check_eaten();
    }
}