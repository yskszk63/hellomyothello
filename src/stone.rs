use app::AppSettings;
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
    pub fn render(&self, settings: &AppSettings, ctx: &Context, gl: &mut GlGraphics) {
        let margin = settings.cell_margin() * 2f64;
        let size = settings.cell_size as f64;
        let rect = rectangle::square(margin, margin, size - (margin * 2f64));

        let color = match *self {
            Stone::Black => settings.black_stone_color,
            Stone::White => settings.white_stone_color,
            Stone::Empty => return
        };

        graphics::ellipse(color, rect, ctx.transform, gl)
    }
}
