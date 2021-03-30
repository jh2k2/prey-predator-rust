use opengl_graphics::GlGraphics;
use piston::input::*;
use crate::board::{Board};

const FOX_COUNT: u32 = 3;
const RABBIT_COUNT: u32 = 15;

pub struct App {
    gl: GlGraphics,
    board: Board,
}

impl App {
    pub fn new(graphic: GlGraphics, width: f32, height: f32) -> App {
        App {
            gl: graphic,
            board: Board::new(RABBIT_COUNT, FOX_COUNT, width, height),
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
                let trans = c.transform
                .trans(
                        rabbit_vec[i].x_coord as f64,
                        rabbit_vec[i].y_coord as f64,
                    )
                .scale(rabbit_vec[i].size as f64, rabbit_vec[i].size as f64);

                rectangle(RABBIT_COLOR, [0.0, 0.0, 5.0, 5.0], trans, gl);
            }

            //draw fox
             for i in 0..fox_vec.len() {
                 let trans = c.transform
                 .trans(
                        fox_vec[i].x_coord as f64,
                        fox_vec[i].y_coord as f64,
                    )
                .scale(fox_vec[i].size as f64, fox_vec[i].size as f64);
                rectangle(FOX_COLOR, [0.0, 0.0, 5.0, 5.0], trans, gl);
            }
        });
    }

    pub fn update(&mut self) {
        //board
        self.board.rabbit_move();
        self.board.fox_move();
        self.board.check_eaten();
    }
}