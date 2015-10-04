extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod colors;
mod gameobject;
mod app;
mod renderable;
mod physics;

use app::*;

fn main() {
    use glutin_window::GlutinWindow as Window;
    use piston::window::WindowSettings;
    use opengl_graphics::{GlGraphics, OpenGL};
    use piston::event_loop::*;
    use piston::input::*;

    let opengl = OpenGL::V3_2;
    let maybe_window: Result<Window, _> = WindowSettings::new(
            "test",
            [640, 480]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build();
    
    if let Ok(window) = maybe_window {
        let mut app = Game::new();
        let mut graphics = GlGraphics::new(opengl);

        for e in window.events() {
            if let Some(ref u_args) = e.update_args() { app.update(u_args); }
            if let Some(ref r_args) = e.render_args() { app.render(&mut graphics, r_args); }
        }
    } else {
        panic!("Starting OpenGL failed.");
    }
}
