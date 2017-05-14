use opengl_graphics::{GlGraphics};
use graphics;
use graphics::{Transformed, Context};
use std::cell::{Cell, RefCell};
use std::iter;
use std::collections::VecDeque;
use std::collections::BTreeSet;
use app::{AppEnv,AppSettings};
use square::Square;
use stone::Stone;
use player::Player;


pub struct Board {
    squares: Vec<Square>,
    focus: Cell<Option<(u32, u32)>>,
    queue: RefCell<VecDeque<(u32, u32)>>,
    pub empties: RefCell<BTreeSet<(u32, u32)>>,
}

impl Board {
    pub fn new(settings: &AppSettings) -> Board {
        let mut board = Board {
            squares: vec![],
            focus: Cell::new(None),
            queue: RefCell::new(VecDeque::new()),
            empties: RefCell::new(BTreeSet::new()),
        };

        for n in 0..(settings.cols * settings.rows) {
            let x = n as u32 % settings.cols;
            let y = n as u32 / settings.rows;
            board.squares.push(Square::new(x, y));
            board.empties.borrow_mut().insert((x, y));
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
        if !self.empties.borrow().is_empty() {
            let player = env.current_player();

            if !self.is_putable(env, player) {
                env.switch_player();
            } else {
                match env.current_player().cpu {
                    true => self.cpu(env, env.current_player()),
                    false => if let Some((x, y)) = self.queue.borrow_mut().pop_front() { self.put(env, x, y) },
                }
            }
        } else {
            env.done()
        }
    }

    pub fn size(&self, env: &AppEnv) -> (u32, u32){
        (env.settings.cols * env.settings.cell_size, env.settings.cols * env.settings.cell_size)
    }

    fn is_putable(&self, env: &AppEnv, player: &Player) -> bool {
        let empties = self.empties.borrow().clone();
        for &(x, y) in empties.iter() {
            if let Some(_) = self.search_reversible(env, x, y, player.stone) {
                return true;
            }
        }
        false
    }

    fn cpu(&self, env: &AppEnv, player: &Player) {
        let empties = self.empties.borrow().clone();
        let mut max = None;

        for &(x, y) in empties.iter() {
            if let Some(reversibles) = self.search_reversible(env, x, y, player.stone) {
                if let Some((n, _, _)) = max {
                    if n < reversibles.len() {
                        max = Some((reversibles.len(), x, y))
                    }
                } else {
                    max = Some((reversibles.len(), x, y))
                }
            }
        }
        if let Some((_, x, y)) = max {
            self.put(env, x, y)
        }
    }

    pub fn get_square<'b>(&'b self, env: &AppEnv, x: u32, y: u32) -> Option<&'b Square> {
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
            let current = env.current_player().stone;

            match square.get_stone() {
                Stone::Empty => {
                    if let Some(reversibles) = self.search_reversible(env, x, y, current) {
                        square.put_stone(env, current);
                        for (x, y) in reversibles {
                            self.get_square(env, x, y).unwrap().put_stone(env, current);
                        }
                        env.switch_player();
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

