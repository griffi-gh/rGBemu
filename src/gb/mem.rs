#[derive(Debug)]
pub struct Memory {
	pub rom:  [u8; 0x8000],
	pub vram: [u8; 0x2000],
	pub wram: [u8; 0x2000],
	pub hram: [u8; 0x007F],
}

impl Memory {
	pub fn new() -> Self {
		Self{
			rom:  [0; 0x8000],
			vram: [0; 0x2000],
			wram: [0; 0x2000],
			hram: [0; 0x007F],
		}
	}
	pub fn read(&self, addr: u16) -> u8 {
		//println!("READ 0x{:X}", addr);
		match addr {
			0x0000..=0x7FFF => self.rom[addr as usize],
			0x8000..=0x9FFF => self.vram[(addr-0x8000) as usize],
			0xA000..=0xBFFF => self.wram[(addr-0xA000) as usize],
			0xE000..=0xFDFF => self.wram[(addr-0xE000) as usize],
			0xFF80..=0xFFFE => self.hram[(addr-0xFF80) as usize],
			_ => 0
		}
	}
	pub fn write(&mut self, addr: u16, value: u8) {
		//println!("WRITE 0x{:X} 0x{:X}", addr, value);
		match addr {
			0x8000..=0x9FFF => { self.vram[(addr-0x8000) as usize] = value },
			0xA000..=0xBFFF => { self.wram[(addr-0xA000) as usize] = value },
			0xE000..=0xFDFF => { self.wram[(addr-0xE000) as usize] = value },
			0xFF80..=0xFFFE => { self.hram[(addr-0xFF80) as usize] = value }, 
			_ => { }
		}
	}
}