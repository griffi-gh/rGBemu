pub mod util;
pub mod reg;
pub mod mem;
pub mod cpu;

#[derive(Debug)]
pub struct Gameboy {
	pub mem: mem::Memory,
	pub cpu: cpu::Cpu,
}

impl Gameboy {
	pub fn new() -> Self {
		Self{
			mem: mem::Memory::new(),
			cpu: cpu::Cpu::new(),
		}
	}
	pub fn step(&mut self) {
		println!("op: {:X} at: {:X}", self.mem.read(self.cpu.reg.pc), self.cpu.reg.pc);
		self.cpu.step(&mut self.mem);
	}
}