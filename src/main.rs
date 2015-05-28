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

const WIDTH:usize = 1920;
const HEIGHT:usize = 1000;
const SIZE:usize = 10;

fn main() {
    let opengl = OpenGL::_3_2;
    let window = Window::new(
        opengl,
        WindowSettings::new(
            "Eratosthenes",
            [WIDTH as u32, HEIGHT as u32]
        )
        .exit_on_esc(true)
    );
    let max = (WIDTH/SIZE) * (HEIGHT/SIZE);
	let mut sieve_algo = sieve::Sieve::new(max);
    let mut app = algo_render::App::new(&mut sieve_algo,WIDTH,HEIGHT,SIZE);
    for e in window.events() {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
    }
}
