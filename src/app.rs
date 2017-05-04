use geom::{Size};
use opengl_graphics::{GlGraphics};
use piston::input::{RenderArgs, UpdateArgs, Button};

pub struct AppSettings {
    pub win_size: Size
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            win_size: Size(1024, 768)
        }
    }
}

pub struct App {
    win_size: Size
}

impl App {
    pub fn new(settings: &AppSettings) -> Self {
        App {
            win_size: settings.win_size
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {

    }

    pub fn update(&mut self, args: &UpdateArgs) {

    }

    pub fn press(&mut self, args: &Button) {

    }

}