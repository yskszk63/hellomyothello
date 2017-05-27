extern crate opengl_graphics;
extern crate sdl2_window;
extern crate piston;
extern crate graphics;
extern crate rand;

use app::{App, AppSettings};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{Input, RenderEvent, UpdateEvent, PressEvent, MouseCursorEvent, Button};
use piston::window::{WindowSettings, Window};
use sdl2_window::Sdl2Window;
use piston::input::{EventId, GenericEvent, TouchEvent, Touch};
use piston::event_loop::{Events, EventSettings};

mod app;
mod board;
mod square;
mod stone;
mod player;

fn main() {
    let app_settings = AppSettings::default();
    let mut app = App::new(&app_settings);

    let opengl = OpenGL::V2_1;
    let mut window: Sdl2Window = WindowSettings::new(env!("CARGO_PKG_NAME"), app.size())
        .opengl(opengl)
        .srgb(false)
        .exit_on_esc(true)
        .build()
        .expect("failed to build window");
    let mut gl = GlGraphics::new(opengl);

    event_loop(&mut window, |e, window| {
        e.render(|args| {
            app.render(args, &mut gl)
        });

        e.update(|_| {
            app.update()
        });

        e.mouse_cursor(|x, y| {
            app.mouse_move(x, y)
        });

        e.press(|button| {
            if let Button::Mouse(_) = button {
                app.click()
            }
        });

        e.touch(|touch| {
            if let Touch::Start = touch.touch {
                let size = window.size();
                let x = (size.width as f64) * touch.x;
                let y = (size.height as f64) * touch.y;
                app.mouse_move(x, y);
                app.click();
            }
        });
    })
}

fn event_loop<W, F>(window: &mut W, mut handler: F)
        where W: Window, F: FnMut(Input, &mut W) {

    let mut events = Events::new(EventSettings::new());
    main_loop::run(|| {
        loop {
            if let Some(e) = events.next(window) {
                match e.event_id() {
                    EventId("piston/idle") => return true,
                    _ => handler(e, window)
                }
            } else {
                return false
            }
        }
    })
}

mod main_loop {

    #[cfg(target_os="emscripten")]
    mod emscripten {
        use std::cell::RefCell;
        use std::ptr::null_mut;
        use std::os::raw::{c_int, c_void};

        #[allow(non_camel_case_types)]
        type em_callback_func = unsafe extern fn();

        extern {
            pub fn emscripten_set_main_loop(func: em_callback_func, fps: c_int, simulate_infini_loop: c_int);
            pub fn emscripten_cancel_main_loop();
        }

        thread_local!(static MAIN_LOOP_CALLBACK: RefCell<*mut c_void> = RefCell::new(null_mut()));

        pub fn set_main_loop_callback<F>(callback: &mut F) where F: FnMut() -> bool {
            MAIN_LOOP_CALLBACK.with(|log| {
                *log.borrow_mut() = callback as *const _ as *mut c_void;
            });

            unsafe { emscripten_set_main_loop(wrapper::<F>, 0, 1) }

            unsafe extern "C" fn wrapper<F>() where F: FnMut() -> bool {
                MAIN_LOOP_CALLBACK.with(|z| {
                    let closure = *z.borrow_mut() as *mut F;
                    if !(*closure)() {
                        emscripten_cancel_main_loop()
                    }
                })
            }
        }
    }

    pub fn run<F>(mut f: F) where F: FnMut() -> bool {
        #[cfg(not(target_os="emscripten"))]
        loop {
            if !f() {
                break
            }
        }

        #[cfg(target_os="emscripten")]
        emscripten::set_main_loop_callback(&mut f)
    }
}