extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use piston::event::*;
use self::opengl_graphics::{ GlGraphics, OpenGL };
use sieve;

const PROGRESS:usize = 10;
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

pub struct App<'gs>{
	generations: &'gs Vec<sieve::Generation>,
	current:usize,
	current_gen:usize,
	gl: GlGraphics,
	slices:usize,
	keep_rendering:bool,
	width:usize,
	height:usize,
	size:usize
}

impl <'gs>App<'gs>{
	pub fn new(generations:&'gs Vec<sieve::Generation>,width:usize,height:usize,size:usize)->App<'gs>{
		let opengl = OpenGL::_3_2;
		App{
			gl: GlGraphics::new(opengl),
			current:0,
			current_gen:0,
			generations:generations,
			slices: width/size as usize,
			keep_rendering:true,
			width:width,
			height:height,
			size:size
		}
	}
	pub fn render(&mut self,args: &RenderArgs){
		if self.keep_rendering{
			use graphics::*;
			let square = rectangle::square(0.0, 0.0, self.size as f64);
			let x_slices = &self.slices;
			let generations = self.generations;
			let current_gen = &self.current_gen;
			let current = &self.current;
			let width = self.width;
			let size = self.size;
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
						 let x = if (i % x_slices) == 0 { width - size } else { ((i % x_slices) * size)-size};
						 let y = if i > x_slices { ((i / x_slices) - if i%x_slices == 0 { 1 } else { 0 } )* size }else { 0 };
						 rectangle(color, square,c.transform.trans(x as f64,y as f64),gl);		
						 ic += 1;		 
					 }
					 if k == *current_gen{
						 break;
					 }
					 k += 1;
				 }
				 for i in 0..x_slices + 1{
					 let x = i as f64 * size as f64;
					 rectangle([0.0, 0.0, 0.0, 1.0], [0.0,0.0,1.0,width as f64],c.transform.trans(x,0.0),gl);
					 rectangle([0.0, 0.0, 0.0, 1.0], [0.0,0.0,width as f64,1.0],c.transform.trans(0.0,x),gl);
				}
						
			});
		}
	}
	pub fn update(&mut self){
		let gen_size = self.generations.len() - 1;
		let len = self.generations[self.current_gen].numbers.len();
		let size = if len>0{ len-1 }else{0};
		
		if self.current < size{
			self.current += if len < PROGRESS { 1 } else { PROGRESS }; ;
			if self.current > size{
				self.current = size;
			}
		}
		else{
			if self.current_gen != gen_size{
				self.current_gen += 1;
				self.current = 0;
			}
			else{
				self.keep_rendering = false;
			}
		}
	}
}
