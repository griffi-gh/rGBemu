#![allow(dead_code)]
#![allow(unused_variables)]

mod gb;

fn main() {
	let mut gb = gb::Gameboy::new();
	loop{
		gb.step();
	}
}