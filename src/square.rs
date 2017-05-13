use std::cell::Cell;
use graphics;
use graphics::Context;
use graphics::rectangle;
use opengl_graphics::GlGraphics;
use app::AppEnv;
use stone::Stone;

pub struct Square {
    stone: Cell<Stone>,
    x: u32,
    y: u32,
    focused: Cell<bool>,
}

impl Square {
    pub fn new(x: u32, y: u32) -> Square {
        Square {
            stone: Cell::new(Stone::Empty),
            x: x,
            y: y,
            focused: Cell::new(false),
        }
    }

    pub fn render(&self, env: &AppEnv, ctx: &Context, gl: &mut GlGraphics) {
        let margin = env.settings.cell_margin();
        let size = env.settings.cell_size as f64;

        let border_rect = rectangle::square(margin * 0f64, margin * 0f64, size - (margin * 0f64));
        let inner_rect = rectangle::square(margin * 1f64, margin * 1f64, size - (margin * 2f64));

        let color = if self.focused.get() {
            env.settings.focused_background_color
        } else {
            env.settings.background_color
        };

        graphics::rectangle(env.settings.separator_color, border_rect, ctx.transform, gl);
        graphics::rectangle(color, inner_rect, ctx.transform, gl);

        self.stone.get().render(env, ctx, gl)
    }

    pub fn put_stone(&self, env: &AppEnv, stone: Stone) {
        match self.stone.get() {
            Stone::Black => env.player_for(Stone::Black).dec(),
            Stone::White => env.player_for(Stone::White).dec(),
            Stone::Empty => {},
        }
        self.stone.set(stone);
        match self.stone.get() {
            Stone::Black => env.player_for(Stone::Black).inc(),
            Stone::White => env.player_for(Stone::White).inc(),
            Stone::Empty => {},
        }
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
