use opengl_graphics::{GlGraphics};
use piston::input::{RenderArgs};
use board::Board;
use graphics::Transformed;
use graphics::color;

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
    fn default() -> Self {
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

pub struct App<'a> {
    pub win_size: (u32, u32),
    settings: &'a AppSettings,
    board: Board<'a>,
}

impl <'a> App<'a> {
    pub fn new(settings: &'a AppSettings) -> Self {
        let board = Board::new(settings);
        let (w, h) = board.size();
        let win_size = (w + settings.cell_size * 4, h + settings.cell_size * 2);
        App {
            win_size: win_size,
            settings: settings,
            board: board,
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        gl.draw(args.viewport(), |c, gl| {
            let cell_size = self.settings.cell_size;
            self.board.render(&c.trans(cell_size as f64, cell_size as f64), gl);

            let (w, _) = self.board.size();
            let c = c.trans((w + cell_size * 2) as f64, cell_size as f64);
            self.board.get_current_state().render(self.settings, &c, gl);
        });
    }

    pub fn update(&mut self) {
        self.board.update();
    }

    pub fn click(&self) {
        self.board.click();
    }

    pub fn mouse_move(&self, x: f64, y: f64) {
        let x = x as u32;
        let y = y as u32;
        let cell_size = self.settings.cell_size;
        let (w, h) = self.board.size();
        if cell_size <= x && cell_size <= y && (x - cell_size) < w && (y - cell_size) < h {
            let cell_x = (x - cell_size) / cell_size;
            let cell_y = (y - cell_size) / cell_size;
            self.board.focus(cell_x, cell_y);
        }
    }

}