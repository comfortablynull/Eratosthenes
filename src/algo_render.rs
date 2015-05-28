extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use piston::event::*;
use self::opengl_graphics::{ GlGraphics, OpenGL };
use graphics::*;
use sieve;

const PROGRESS:usize = 15;
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

pub struct App<'s>{
	algo: &'s mut sieve::Sieve,
	stop:usize,
	start:usize,
	gl: GlGraphics,
	slices:usize,
	keep_rendering:bool,
	width:usize,
	size:usize,
	first_run:bool
}

impl <'s>App<'s>{
	pub fn new(algo:&'s mut sieve::Sieve,width:usize,size:usize)->App<'s>{
		let opengl = OpenGL::_3_2;
		algo.make_next_gen();
		App{
			gl: GlGraphics::new(opengl),
			stop:0,
			start: 0,
			algo:algo,
			slices: width/size as usize,
			keep_rendering:true,
			width:width,
			size:size,
			first_run:true
		}
	}
	pub fn render(&mut self,args: &RenderArgs){
		if self.keep_rendering{
			if self.first_run{
				self.first_run = false;
				self.gl.draw(args.viewport(),|c, gl| {
					clear(GREEN, gl);
				});
			}
			self.draw_generation(&args);
			self.draw_lines(&args);
			self.update();
		}
	}
	fn draw_lines(&mut self,args: &RenderArgs){
		let x_slices = &self.slices;
		let width = &self.width;
		let size = &self.size;
		self.gl.draw(args.viewport(),|c, gl| {
			for i in 0..x_slices + 1{
				let x = i as f64 * *size as f64;
				rectangle([0.0, 0.0, 0.0, 1.0], [0.0,0.0,1.0, *width as f64],c.transform.trans(x,0.0),gl);
				rectangle([0.0, 0.0, 0.0, 1.0], [0.0,0.0, *width as f64,1.0],c.transform.trans(0.0,x),gl);
			}
		});

	}
	fn draw_generation(&mut self,args:&RenderArgs){
		let gen = &self.algo.generation;
		let stop = &self.stop;
		let start = &self.start;
		let x_slices = &self.slices;
		let width = &self.width;
		let size = &self.size;
		let square = rectangle::square(0.0, 0.0, self.size as f64);
		self.gl.draw(args.viewport(),|c, gl| {
			for n in *start..*stop{
				let i = gen.numbers[n];
				let x = if (i % x_slices) == 0 { *width - *size } else { ((i % *x_slices) * *size)-*size};
				let y = if i > *x_slices { ((i / *x_slices) - if i%*x_slices == 0 { 1 } else { 0 } )* *size }else { 0 };
				rectangle(gen.color, square,c.transform.trans(x as f64,y as f64),gl);
			}
		});
	}
	pub fn update(&mut self){
		let size = self.algo.generation.numbers.len();
		self.start = self.stop;
		if self.stop < size{
			self.update_stop();
		}
		else{
			if self.algo.make_next_gen(){
				self.stop = 0;
				self.update_stop();
				self.start = 0;
			}
			else{
				self.keep_rendering = false;
			}
		}
	}
	pub fn update_stop(&mut self){
		let size = self.algo.generation.numbers.len();
		self.stop += if size < PROGRESS { 1 } else { PROGRESS };
		if self.stop > size{
			self.stop = size;
		}
	}
}
