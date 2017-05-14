use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use piston::input::RenderArgs;
use board::Board;
use stone::Stone;
use player::Player;
use graphics::Transformed;
use graphics::color;
use std::cell::Cell;
use std::cell::RefCell;
use rand;

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
            background_color: color::hex("008000"), // green
            focused_background_color: color::hex("3cb371"), // mediumseagreen
            separator_color: color::hex("000000"), // black
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
    current: Cell<usize>,
    players: [Player; 2],
    pub font: RefCell<GlyphCache<'a>>,
    done: Cell<bool>,
}
impl <'a> AppEnv<'a> {
    pub fn current_player<'b>(&'b self) -> &'b Player {
        &self.players[self.current.get()]
    }
    pub fn switch_player(&self) {
        self.current_player().turn.set(false);
        let n = self.current.get();
        self.current.set((n + 1) % self.players.len());
        self.current_player().turn.set(true);
    }
    pub fn player_for<'b> (&'b self, stone: Stone) -> &'b Player {
        &self.players.iter().find(|p| p.stone == stone).unwrap()
    }
    pub fn done(&self) {
        self.done.set(true)
    }
    pub fn is_done(&self) -> bool {
        self.done.get()
    }
}

pub struct App<'a> {
    env: AppEnv<'a>,
}

impl <'a> App<'a> {
    pub fn new(settings: &'a AppSettings) -> Self {
        let black_is_player = rand::random::<bool>();
        let board = Board::new(settings);
        let font = GlyphCache::from_bytes(include_bytes!("../assets/FiraMono-Regular.ttf")).unwrap();
        let env = AppEnv {
            settings: settings,
            board: board,
            invalidate: Cell::new(false),
            current: Cell::new(0),
            players: [
                Player::new(Stone::White, !black_is_player),
                Player::new(Stone::Black, black_is_player) ],
            font: RefCell::new(font),
            done: Cell::new(false),
        };
        env.current_player().turn.set(true);

        let herf_of_cols = settings.cols / 2;
        let herf_of_rows = settings.rows / 2;
        
        env.board.get_square(&env, herf_of_cols - 1, herf_of_rows - 1).unwrap().put_stone(&env, Stone::White);
        env.board.get_square(&env, herf_of_cols - 0, herf_of_rows - 0).unwrap().put_stone(&env, Stone::White);
        env.board.get_square(&env, herf_of_cols - 0, herf_of_rows - 1).unwrap().put_stone(&env, Stone::Black);
        env.board.get_square(&env, herf_of_cols - 1, herf_of_rows - 0).unwrap().put_stone(&env, Stone::Black);

        App { env: env, }
    }

    pub fn size(&self) -> (u32, u32) {
        let (w, h) = self.env.board.size(&self.env);
        (w + self.env.settings.cell_size * 6, h + self.env.settings.cell_size * 2)
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        gl.draw(args.viewport(), |c, gl| {
            let cell_size = self.env.settings.cell_size;
            self.env.board.render(&self.env, &c.trans(cell_size as f64, cell_size as f64), gl);

            let (w, _) = self.env.board.size(&self.env);
            let c = c.trans((w + cell_size * 2) as f64, cell_size as f64);
            for (i, player) in self.env.players.iter().enumerate() {
                let c = c.trans(0f64, (cell_size * i as u32) as f64);
                player.render(&self.env, &c, gl);
            }
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