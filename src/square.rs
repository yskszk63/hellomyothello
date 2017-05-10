use std::cell::Cell;
use graphics;
use graphics::Context;
use graphics::rectangle;
use opengl_graphics::GlGraphics;
use app::AppSettings;
use stone::Stone;

pub struct Square<'a> {
    stone: Cell<Stone>,
    x: u32,
    y: u32,
    focused: Cell<bool>,
    settings: &'a AppSettings,
}

impl <'a> Square <'a> {
    pub fn new(settings: &'a AppSettings, x: u32, y: u32) -> Square<'a> {
        Square {
            stone: Cell::new(Stone::Empty),
            x: x,
            y: y,
            focused: Cell::new(false),
            settings: settings,
        }
    }

    pub fn render(&self, ctx: &Context, gl: &mut GlGraphics) {
        let margin = self.settings.cell_margin();
        let size = self.settings.cell_size as f64;

        let border_rect = rectangle::square(margin * 0f64, margin * 0f64, size - (margin * 0f64));
        let inner_rect = rectangle::square(margin * 1f64, margin * 1f64, size - (margin * 2f64));

        let color = if self.focused.get() {
            self.settings.focused_background_color
        } else {
            self.settings.background_color
        };

        graphics::rectangle(self.settings.separator_color, border_rect, ctx.transform, gl);
        graphics::rectangle(color, inner_rect, ctx.transform, gl);

        self.stone.get().render(self.settings, ctx, gl)
    }

    pub fn set_stone(&self, stone: Stone) {
        self.stone.set(stone);
    }

    pub fn get_stone(&self) -> Stone {
        self.stone.get()
    }

    pub fn set_focus(&self, focus: bool) {
        self.focused.set(focus);
    }

    pub fn get_x(&self) -> u32 {
        self.x
    }

    pub fn get_y(&self) -> u32 {
        self.y
    }

}
