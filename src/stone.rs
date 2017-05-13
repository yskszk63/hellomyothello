use app::AppEnv;
use graphics;
use graphics::Context;
use graphics::rectangle;
use opengl_graphics::GlGraphics;

#[derive(Copy, Clone, PartialEq)]
pub enum Stone {
    White,
    Black,
    Empty,
}

impl Stone {
    pub fn render(&self, env: &AppEnv, ctx: &Context, gl: &mut GlGraphics) {
        let margin = env.settings.cell_margin() * 2f64;
        let size = env.settings.cell_size as f64;
        let rect = rectangle::square(margin, margin, size - (margin * 2f64));

        let color = match *self {
            Stone::Black => env.settings.black_stone_color,
            Stone::White => env.settings.white_stone_color,
            Stone::Empty => return
        };

        graphics::ellipse(color, rect, ctx.transform, gl)
    }
}
