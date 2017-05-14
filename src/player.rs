use opengl_graphics::glyph_cache::GlyphCache;
use stone::Stone;
use std::cell::Cell;
use app::AppEnv;
use graphics;
use graphics::{Transformed, Context};
use opengl_graphics::{GlGraphics};

pub struct Player {
    pub stone: Stone,
    pub cpu: bool,
    pub cnt: Cell<u32>,
    pub turn: Cell<bool>
}
impl Player {
    pub fn new(stone: Stone, cpu: bool) -> Player {
        Player {
            stone: stone,
            cpu: cpu,
            cnt: Cell::new(0),
            turn: Cell::new(false),
        }
    }
    pub fn render(&self, env: &AppEnv, ctx: &Context, gl: &mut GlGraphics) {
        let font: &mut GlyphCache = &mut env.font.borrow_mut();
        let color = env.settings.white_stone_color;
        let cell_size = env.settings.cell_size;

        if self.turn.get() && !env.is_done() {
            graphics::text::Text::new_color(color, 32).draw(
                ">", font, &ctx.draw_state, ctx.trans(0f64, (cell_size) as f64).transform, gl);
        }

        let trans = ctx.trans(cell_size as f64, 0f64);
        self.stone.render(env, &trans, gl);

        let trans = ctx.trans(cell_size as f64 * 2f64, (cell_size) as f64);
        graphics::text::Text::new_color(color, 32).draw(
            &format!("{}", self.cnt.get()), font, &ctx.draw_state, trans.transform, gl);

    }
    pub fn inc(&self) {
        self.cnt.set(self.cnt.get() + 1 )
    }
    pub fn dec(&self) {
        self.cnt.set(self.cnt.get() - 1 )
    }
}