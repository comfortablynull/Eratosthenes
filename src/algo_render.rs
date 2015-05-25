extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use sieve;

const WIDTH:usize = 1920;
const HEIGHT:usize = 1000;
const SIZE:usize = 10;
const PROGRESS:usize = 10;
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

pub struct App{
	generations: Vec<sieve::generation>,
	current:usize,
	current_gen:usize,
	gl: GlGraphics,
	slices:usize
}

impl App{
	pub fn new()->App{
		let opengl = OpenGL::_3_2;
		let max = (WIDTH/SIZE) * (HEIGHT/SIZE);
		let floor = (max as f64).sqrt() as usize;
		let mut sieve_algo = sieve::Sieve::new(max);
		sieve_algo.run();
		App{
			gl: GlGraphics::new(opengl),
			current:0,
			current_gen:0,
			generations: sieve_algo.generations,
			slices: WIDTH/SIZE as usize
		}
	}
	pub fn render(&mut self,args: &RenderArgs){
		use graphics::*;
        let square = rectangle::square(0.0, 0.0, SIZE as f64);
        let x_slices = &self.slices;
        let generations = &self.generations;
        let current_gen = &self.current_gen;
        let current = &self.current;
         self.gl.draw(args.viewport(),|c, gl| {
			 clear(GREEN, gl);
			 let mut k:usize = 0;
			 for g in generations{
				 let color:[f32;4] = g.color;
				 let nums = &g.numbers;
				 let mut ic:usize = 1;
				 for i in nums{
					 if ic == *current && k == *current_gen{
						 break;
					 }
					 let x = if (i % x_slices) == 0 { WIDTH - SIZE } else { ((i % x_slices) * SIZE)-SIZE};
					 let y = if i > x_slices { ((i / x_slices) - if i%x_slices == 0 { 1 } else { 0 } )* SIZE }else { 0 };
					 rectangle(color, square,c.transform.trans(x as f64,y as f64),gl);		
					 ic += 1;		 
				 }
				 if k == *current_gen{
					 break;
				 }
				 k += 1;
			 }
			 for i in 0..x_slices + 1{
				 let x = i as f64 * SIZE as f64;
				 rectangle([0.0, 0.0, 0.0, 1.0], [0.0,0.0,1.0,WIDTH as f64],c.transform.trans(x,0.0),gl);
				 rectangle([0.0, 0.0, 0.0, 1.0], [0.0,0.0,WIDTH as f64,1.0],c.transform.trans(0.0,x),gl);
			}
					
		});
	}
	pub fn update(&mut self){
		let gen_size = self.generations.len() - 1;
		let len = self.generations[self.current_gen].numbers.len();
		let size = if len>0{ len-1 }else{0};
		let progress = if len < PROGRESS { 1 } else { PROGRESS }; 
		if self.current < size{
			self.current += progress;
			if self.current > size{
				self.current = size;
			}
		}
		else{
			if self.current_gen != gen_size{
				self.current_gen += 1;
				self.current = 0;
			}
		}
	}
}
