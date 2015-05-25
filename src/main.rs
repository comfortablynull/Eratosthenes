extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
mod algo_render;
mod sieve;

fn main() {
    let opengl = OpenGL::_3_2;
    let window = Window::new(
        opengl,
        WindowSettings::new(
            "Eratosthenes",
            [1920, 1000]
        )
        .exit_on_esc(true)
    );
    let mut app = algo_render::App::new();
    for e in window.events() {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
		if let Some(u) = e.update_args(){
			app.update();
		}
    }
}
