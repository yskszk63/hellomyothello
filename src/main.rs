extern crate opengl_graphics;
extern crate sdl2_window;
extern crate piston;
extern crate graphics;
extern crate rand;

use app::{App, AppSettings};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{Input, RenderEvent, UpdateEvent, PressEvent, MouseCursorEvent, Button};
use piston::window::{OpenGLWindow, WindowSettings};
use sdl2_window::Sdl2Window;

mod app;
mod board;

fn main() {
    let app_settings = AppSettings::default();
    let app = App::new(&app_settings);

    let opengl = OpenGL::V2_1;
    let window: Sdl2Window = WindowSettings::new(env!("CARGO_PKG_NAME"), app.win_size)
        .opengl(opengl)
        .srgb(false)
        .exit_on_esc(true)
        .build()
        .expect("failed to build window");
    let gl = GlGraphics::new(opengl);

    event_loop::run(window, gl, handle_event, app);
}

fn handle_event(window: &mut Sdl2Window, gl: &mut GlGraphics, e: Input, app: &mut App) {
    if let Some(ref args) = e.render_args() {
        window.make_current();
        app.render(args, gl);
    }

    if let Some(_) = e.update_args() {
        app.update();
    }

    if let Some(pos) = e.mouse_cursor_args() {
        app.mouse_move(pos[0], pos[1]);
    }

    if let Some(Button::Mouse(_)) = e.press_args() {
        app.click();
    }
}

#[cfg(not(target_os = "emscripten"))]
mod event_loop {
    use piston::event_loop::{EventSettings, Events};
    use piston::input::Input;
    use sdl2_window::Sdl2Window;
    use opengl_graphics::GlGraphics;

    pub fn run<T>(mut window: Sdl2Window, mut gl: GlGraphics,
            handler: fn(window: &mut Sdl2Window, gl: &mut GlGraphics, e: Input, arg: &mut T), mut arg: T) {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut window) {
            handler(&mut window, &mut gl, e, &mut arg);
        }
    }
}

#[cfg(target_os = "emscripten")]
mod event_loop {

    extern crate emscripten_sys;

    use piston::input::{Input, AfterRenderArgs, RenderArgs, UpdateArgs};
    use piston::window::Window;
    use sdl2_window::Sdl2Window;
    use opengl_graphics::GlGraphics;
    use std::mem;
    use std::os::raw::c_void;

    struct Context<T> {
        last_updated: f64,
        window: Sdl2Window,
        gl: GlGraphics,
        handler: fn(window: &mut Sdl2Window, gl: &mut GlGraphics, e: Input, arg: &mut T),
        arg: T
    }

    pub fn run<T>(window: Sdl2Window, gl: GlGraphics,
            handler: fn(window: &mut Sdl2Window, gl: &mut GlGraphics, e: Input, arg: &mut T), arg: T) {

        unsafe {
            let mut events = Box::new(Context {
                last_updated: emscripten_sys::emscripten_get_now() as f64,
                window: window,
                gl: gl,
                handler: handler,
                arg: arg
            });
            let ptr = &mut *events as *mut Context<_> as *mut c_void;
            emscripten_sys::emscripten_set_main_loop_arg(Some(main_loop_c::<T>), ptr, 0, 1);
            mem::forget(events);
        }
    }

    extern "C" fn main_loop_c<T>(arg: *mut c_void) {
        unsafe {
            let mut ctx: &mut Context<T> = mem::transmute(arg);
            let window = &mut ctx.window;
            let gl = &mut ctx.gl;
            let handler = ctx.handler;
            let arg = &mut ctx.arg;
            window.swap_buffers();

            let e = Input::AfterRender(AfterRenderArgs);
            handler(window, gl, e, arg);

            while let Some(e) = window.poll_event() {
                handler(window, gl, e, arg);
            }

            if window.should_close() {
                emscripten_sys::emscripten_cancel_main_loop();
                return;
            }

            let now = emscripten_sys::emscripten_get_now() as f64;
            let dt = now - ctx.last_updated;
            ctx.last_updated = now;

            let e = Input::Update(UpdateArgs {dt: dt});
            handler(window, gl, e, arg);

            let size = window.size();
            let draw_size = window.draw_size();
            let e = Input::Render(RenderArgs {
                ext_dt: dt,
                width: size.width,
                height: size.height,
                draw_width: draw_size.width,
                draw_height: draw_size.height
            });
            handler(window, gl, e, arg);
        }
    }

}