extern crate rand;
use self::rand::distributions::{Range, IndependentSample};
pub struct Generation{
	pub color:[f32;4],
	pub numbers:Vec<usize>
}
pub struct Sieve{
	pub generations:Vec<Generation>,
	max:usize,
	possible_primes:Vec<bool>
}
impl Generation{
	fn new()->Generation{
		let empty:Vec<usize>=  Vec::new();
		let mut rng = rand::thread_rng();
		let range = Range::new(0.0, 1.0);
		let r:f32 = range.ind_sample(&mut rng);
		let g:f32 = 0.5;
		let b:f32 = range.ind_sample(&mut rng);
		Generation{
			color: [r,g,b,1.0], 
			numbers: empty
		}
	}
	fn add(&mut self, num:usize){
		self.numbers.push(num);
	}
}
impl Sieve{
	pub fn new(max:usize)->Sieve{
		let empty:Vec<Generation> = Vec::new();
		Sieve{
			max:max,
			generations:empty,
			possible_primes:vec![true;max+1]
		}
	}
	pub fn run(&mut self){
		let floor = (self.max as f32).sqrt() as usize;
		for i in 2..floor{
			if self.possible_primes[i]{
				let idx = i;
				let gen = self.make_gen(idx);
				self.generations.push(gen);
			}
		}
	}
	pub fn make_gen(&mut self,idx:usize)->Generation{
		let mut gen = Generation::new();
		let mut count = 0;
		if idx == 2{
			gen.add(1);
		}
		loop{
			let j = (((idx as i64).pow(2))+(count * idx as i64)) as usize;
			if j > self.max { break; }
			if self.possible_primes[j]{
				self.possible_primes[j] = false;
				gen.add(j as usize);
			}
			count += 1;
		}
		return gen;
	}
}
