#![allow(dead_code)]

mod gb;

fn main() {
	let mut gb = gb::Gameboy::new();
	gb.mem.load_rom(&String::from("C:/Users/User/Desktop/Games/TESTGAME.gb"));
	loop{
		gb.step();
	}
}