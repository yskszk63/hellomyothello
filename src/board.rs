use opengl_graphics::{GlGraphics};
use graphics;
use graphics::{Transformed, Context};
use graphics::math::Matrix2d;
use app::AppSettings;
use std::cell::Cell as StdCell;
use std::iter;
use rand;

#[derive(Copy, Clone, PartialEq)]
pub enum CellState {
    Black,
    White,
    Empty,
}

impl CellState {
    fn render(&self, cell: &Cell, transform: Matrix2d, gl: &mut GlGraphics) {
        let cell_margin = cell.margin();
        let cell_size = cell.settings.cell_size;
        let stone = graphics::rectangle::square(cell_margin * 2f64, cell_margin * 2f64, cell_size as f64 - (cell_margin * 4f64));

        match *self {
            CellState::Black => {
                graphics::ellipse(cell.settings.black_stone_color, stone, transform, gl)
            },
            CellState::White => {
                graphics::ellipse(cell.settings.white_stone_color, stone, transform, gl)
            },
            CellState::Empty => {}
        }
    }
}

struct Cell<'a> {
    state: StdCell<CellState>,
    x: u32,
    y: u32,
    focused: StdCell<bool>,
    settings: &'a AppSettings,
}

impl <'a> Cell<'a> {
    fn new(settings: &'a AppSettings, x: u32, y: u32) -> Cell<'a> {
        Cell {
            state: StdCell::new(CellState::Empty),
            x: x,
            y: y,
            focused: StdCell::new(false),
            settings: settings,
        }
    }

    fn render(&self, ctx: Matrix2d, gl: &mut GlGraphics) {
        let cell_margin = self.margin();
        let cell_size = self.settings.cell_size;
        let inner = graphics::rectangle::square(cell_margin * 1f64, cell_margin * 1f64, cell_size as f64 - (cell_margin * 2f64));
        let outer = graphics::rectangle::square(cell_margin * 0f64, cell_margin * 0f64, cell_size as f64 - (cell_margin * 0f64));

        let color = if self.focused.get() { self.settings.focused_background_color} else { self.settings.background_color };
        graphics::rectangle(self.settings.separator_color, outer, ctx, gl);
        graphics::rectangle(color, inner, ctx, gl);
        self.state.get().render(self, ctx, gl);
    }

    fn margin(&self) -> f64 {
        self.settings.cell_size as f64 * 0.05f64
    }

}

pub struct Board<'a> {
    cells: Vec<Cell<'a>>,
    current: StdCell<CellState>,
    settings: &'a AppSettings,
    focus: StdCell<Option<(u32, u32)>>,
    invalidate: StdCell<bool>,
}

impl <'a> Board<'a> {
    pub fn new(settings: &'a AppSettings) -> Board<'a> {
        let mut board = Board {
            cells: Vec::<Cell>::new(),
            current: StdCell::new(CellState::Black),
            settings: settings,
            focus: StdCell::new(None),
            invalidate: StdCell::new(true),
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
            let cell = Cell::new(settings, x, y);
            match (x, y) {
                (x, y) if (x, y) == white1 || (x, y) == white2 => cell.state.set(CellState::White),
                (x, y) if (x, y) == black1 || (x, y) == black2 => cell.state.set(CellState::Black),
                _ => {}
            }
            board.cells.push(cell);
        }

        board
    }

    pub fn render(&self, ctx: &Context, gl: &mut GlGraphics) {
        if self.invalidate.get() {
            self.invalidate.set(false);

            graphics::clear(self.settings.background_color, gl);

            for (i, cell) in self.cells.iter().enumerate() {
                let x = i as u32 % self.settings.cols;
                let y = i as u32 / self.settings.rows;

                let transform = ctx.transform.trans((x * self.settings.cell_size) as f64, (y * self.settings.cell_size) as f64);
                cell.render(transform, gl);
            }
        }
    }

    pub fn update(&self) {
        self.invalidate.set(true);
        if self.current.get() == CellState::Black {
            self.cpu();
        }
    }

    fn cpu(&self) {
        let x = rand::random::<u32>() % self.settings.rows;
        let y = rand::random::<u32>() % self.settings.cols;
        self.put(x, y);
    }

    fn get_cell<'b>(&'b self, x: u32, y: u32) -> Option<&'b Cell<'a>> {
        let index = (y * self.settings.rows + x) as usize;
        if self.cells.len() > index {
            Some(&self.cells[index])
        } else {
            None
        }
    }

    pub fn focus(&self, x: u32, y: u32) {
        if let Some(cell) = self.focus.get().and_then(|(x,y)| { self.get_cell(x, y) }) {
            cell.focused.set(false);
        }
        if let Some(cell) = self.get_cell(x, y) {
            self.focus.set(Some((cell.x, cell.y)));
            cell.focused.set(true);
        }
    }

    pub fn click(&self) {
        if self.current.get() == CellState::White {
            if let Some((x, y)) = self.focus.get() {
                self.put(x, y);
            }
        }
    }

    fn put(&self, x: u32, y: u32) {
        if let Some(cell) = self.get_cell(x, y) {
            let current = self.current.get();

            match cell.state.get() {
                CellState::Empty => {
                    if let Some(reversibles) = self.search_reversible(x, y, current) {
                        cell.state.set(current);
                        for reversible in reversibles {
                            self.get_cell(reversible.0, reversible.1).unwrap().state.set(current);
                        }
                        self.current.set(match self.current.get() {
                            CellState::Black => CellState::White,
                            CellState::White => CellState::Black,
                            _ => CellState::Black
                        });
                    }
                },
                _ => {}
            }
        }
    }

    fn search_reversible(&self, x: u32, y: u32, my: CellState) -> Option<Vec<(u32, u32)>> {
        let left = || { (0..x).rev() };
        let right = || { ((x + 1)..self.settings.rows) };
        let up = || { (0..y).rev() };
        let down = || { ((y + 1)..self.settings.cols) };
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
                if let Some(state) = self.get_cell(tx, ty).map(|cell| { cell.state.get() }) {
                    match (my, state) {
                        (CellState::White, CellState::Black) | (CellState::Black, CellState::White) => candidates.push((tx, ty)),
                        (CellState::White, CellState::White) | (CellState::Black, CellState::Black) => {vec.append(&mut candidates); break},
                        (_, CellState::Empty) => break,
                        _ => {}
                    }
                }
            }
        }

        if vec.len() > 0 { Some(vec) } else { None }
    }

}

