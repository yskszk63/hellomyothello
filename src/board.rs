use opengl_graphics::{GlGraphics};
use graphics;
use graphics::{Transformed, Context};
use std::cell::{Cell, RefCell};
use std::iter;
use std::collections::VecDeque;
use rand;
use app::{AppEnv,AppSettings};
use square::Square;
use stone::Stone;


pub struct Board {
    squares: Vec<Square>,
    focus: Cell<Option<(u32, u32)>>,
    queue: RefCell<VecDeque<(u32, u32)>>,
}

impl Board {
    pub fn new(settings: &AppSettings) -> Board {
        let mut board = Board {
            squares: vec![],
            focus: Cell::new(None),
            queue: RefCell::new(VecDeque::new()),
        };

        let herf_of_cols = settings.cols / 2;
        let herf_of_rows = settings.rows / 2;
        let white1 = (herf_of_cols - 1, herf_of_rows - 1);
        let white2 = (herf_of_cols - 0, herf_of_rows - 0);
        let black1 = (herf_of_cols - 0, herf_of_rows - 1);
        let black2 = (herf_of_cols - 1, herf_of_rows - 0);
        for n in 0..(settings.cols * settings.rows) {
            let x = n as u32 % settings.cols;
            let y = n as u32 / settings.rows;
            let square = Square::new(x, y);
            match (x, y) {
                (x, y) if (x, y) == white1 || (x, y) == white2 => square.set_stone(Stone::White),
                (x, y) if (x, y) == black1 || (x, y) == black2 => square.set_stone(Stone::Black),
                _ => {}
            }
            board.squares.push(square);
        }

        board
    }

    pub fn render(&self, env: &AppEnv, ctx: &Context, gl: &mut GlGraphics) {
        if env.invalidate.get() {
            env.invalidate.set(false);

            graphics::clear(env.settings.background_color, gl);

            for (i, square) in self.squares.iter().enumerate() {
                let x = i as u32 % env.settings.cols;
                let y = i as u32 / env.settings.rows;

                let cell_size = env.settings.cell_size;
                let square_ctx = ctx.trans((x * cell_size) as f64, (y * cell_size) as f64);
                square.render(env, &square_ctx, gl);
            }
        }
    }

    pub fn update(&self, env: &AppEnv) {
        env.invalidate.set(true);
        match env.current.get() {
            Stone::Black => self.cpu(env),
            Stone::White => if let Some((x, y)) = self.queue.borrow_mut().pop_front() { self.put(env, x, y) },
            _ => {}
        }
    }

    pub fn size(&self, env: &AppEnv) -> (u32, u32){
        (env.settings.cols * env.settings.cell_size, env.settings.cols * env.settings.cell_size)
    }

    fn cpu(&self, env: &AppEnv) {
        let x = rand::random::<u32>() % env.settings.rows;
        let y = rand::random::<u32>() % env.settings.cols;
        self.put(env, x, y);
    }

    fn get_square<'b>(&'b self, env: &AppEnv, x: u32, y: u32) -> Option<&'b Square> {
        let index = (y * env.settings.rows + x) as usize;
        if self.squares.len() > index {
            Some(&self.squares[index])
        } else {
            None
        }
    }

    pub fn focus(&self, env: &AppEnv, x: u32, y: u32) {
        if let Some(square) = self.focus.get().and_then(|(x,y)| { self.get_square(env, x, y) }) {
            square.set_focus(false);
        }
        if let Some(square) = self.get_square(env, x, y) {
            self.focus.set(Some((square.get_x(), square.get_y())));
            square.set_focus(true);
        }
    }

    pub fn click(&self) {
        if let Some((x, y)) = self.focus.get() {
            self.queue.borrow_mut().push_back((x, y));
        }
    }

    fn put(&self, env: &AppEnv, x: u32, y: u32) {
        if let Some(square) = self.get_square(env, x, y) {
            let current = env.current.get();

            match square.get_stone() {
                Stone::Empty => {
                    if let Some(reversibles) = self.search_reversible(env, x, y, current) {
                        square.set_stone(current);
                        for (x, y) in reversibles {
                            self.get_square(env, x, y).unwrap().set_stone(current);
                        }
                        env.current.set(match env.current.get() {
                            Stone::Black => Stone::White,
                            Stone::White => Stone::Black,
                            _ => Stone::Black
                        });
                    }
                },
                _ => {}
            }
        }
    }

    fn search_reversible(&self, env: &AppEnv, x: u32, y: u32, my: Stone) -> Option<Vec<(u32, u32)>> {
        let left = || { (0..x).rev() };
        let right = || { ((x + 1)..env.settings.cols) };
        let up = || { (0..y).rev() };
        let down = || { ((y + 1)..env.settings.rows) };
        let stayx = || { iter::repeat(x) };
        let stayy = || { iter::repeat(y) };

        let mut iters: Vec<Box<Iterator<Item=(u32, u32)>>> = vec![];
        iters.push(Box::new(left().zip(stayy())));
        iters.push(Box::new(left().zip(up())));
        iters.push(Box::new(stayx().zip(up())));
        iters.push(Box::new(right().zip(up())));
        iters.push(Box::new(right().zip(stayy())));
        iters.push(Box::new(right().zip(down())));
        iters.push(Box::new(stayx().zip(down())));
        iters.push(Box::new(left().zip(down())));

        let iters = iters;
        let mut vec = Vec::new();

        for iter in iters {
            let mut candidates = Vec::new();
            for (tx, ty) in iter {
                if let Some(other) = self.get_square(env, tx, ty).map(|square| { square.get_stone() }) {
                    match (my, other) {
                        (Stone::White, Stone::Black) | (Stone::Black, Stone::White) => candidates.push((tx, ty)),
                        (Stone::White, Stone::White) | (Stone::Black, Stone::Black) => {vec.append(&mut candidates); break},
                        (_, Stone::Empty) => break,
                        _ => {}
                    }
                }
            }
        }

        if vec.len() > 0 { Some(vec) } else { None }
    }

}

