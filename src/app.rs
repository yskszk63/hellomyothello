use opengl_graphics::{GlGraphics};
use piston::input::{RenderArgs};
use board::Board;
use stone::Stone;
use graphics::Transformed;
use graphics::color;
use std::cell::Cell;

pub struct AppSettings {
    pub cell_size: u32,
    pub cols: u32,
    pub rows: u32,
    pub background_color: [f32; 4],
    pub focused_background_color: [f32; 4],
    pub separator_color: [f32; 4],
    pub black_stone_color: [f32; 4],
    pub white_stone_color: [f32; 4],
}

impl Default for AppSettings {
    fn default() -> AppSettings {
        AppSettings {
            cell_size: 40,
            cols: 8,
            rows: 8,
            background_color: color::hex("008800"),
            focused_background_color: color::hex("00ff00"),
            separator_color: color::hex("000000"),
            black_stone_color: color::hex("000000"),
            white_stone_color: color::hex("ffffff"),
        }
    }
}
impl AppSettings {
    pub fn cell_margin(&self) -> f64 {
        self.cell_size as f64 * 0.05f64
    }
}

pub struct AppEnv<'a> {
    pub settings: &'a AppSettings,
    pub board: Board,
    pub invalidate: Cell<bool>,
    pub current: Cell<Stone>,
}

pub struct App<'a> {
    env: AppEnv<'a>,
}

impl <'a> App<'a> {
    pub fn new(settings: &'a AppSettings) -> Self {
        let board = Board::new(settings);
        let env = AppEnv {
            settings: settings,
            board: board,
            invalidate: Cell::new(false),
            current: Cell::new(Stone::White),
        };
        App { env: env, }
    }

    pub fn size(&self) -> (u32, u32) {
        let (w, h) = self.env.board.size(&self.env);
        (w + self.env.settings.cell_size * 4, h + self.env.settings.cell_size * 2)
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        gl.draw(args.viewport(), |c, gl| {
            let cell_size = self.env.settings.cell_size;
            self.env.board.render(&self.env, &c.trans(cell_size as f64, cell_size as f64), gl);

            let (w, _) = self.env.board.size(&self.env);
            let c = c.trans((w + cell_size * 2) as f64, cell_size as f64);
            self.env.current.get().render(&self.env, &c, gl);
        });
    }

    pub fn update(&mut self) {
        self.env.board.update(&self.env);
    }

    pub fn click(&self) {
        self.env.board.click();
    }

    pub fn mouse_move(&self, x: f64, y: f64) {
        let x = x as u32;
        let y = y as u32;
        let cell_size = self.env.settings.cell_size;
        let (w, h) = self.env.board.size(&self.env);
        if cell_size <= x && cell_size <= y && (x - cell_size) < w && (y - cell_size) < h {
            let cell_x = (x - cell_size) / cell_size;
            let cell_y = (y - cell_size) / cell_size;
            self.env.board.focus(&self.env, cell_x, cell_y);
        }
    }

}