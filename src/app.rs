use opengl_graphics::{GlGraphics};
use piston::input::{RenderArgs};
use board::Board;
use graphics::Transformed;

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
            background_color: hex_color("008800"),
            focused_background_color: hex_color("00ff00"),
            separator_color: hex_color("000000"),
            black_stone_color: hex_color("000000"),
            white_stone_color: hex_color("ffffff")
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


fn hex_color(color_code: &str) -> [f32; 4] {
    let mut rgb_chars = color_code.chars();
    let r: f32 = (hex_to_dec( rgb_chars.nth(0).unwrap() ) * 16
        + hex_to_dec( rgb_chars.nth(0).unwrap() ) ) as f32 / 255f32;
    let g: f32 = (hex_to_dec( rgb_chars.nth(0).unwrap() ) * 16
        + hex_to_dec( rgb_chars.nth(0).unwrap() ) ) as f32 / 255f32;
    let b: f32 = (hex_to_dec( rgb_chars.nth(0).unwrap() ) * 16
        + hex_to_dec( rgb_chars.nth(0).unwrap() ) ) as f32 / 255f32;
    return [r, g, b, 1.0];
}

fn hex_to_dec(c: char) -> i32 {
    return match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' => 10,
        'b' => 11,
        'c' => 12,
        'd' => 13,
        'e' => 14,
        'f' => 15,
        _ => 0,
    };
}