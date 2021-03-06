use crate::gb::util as util;

#[derive(Debug, Default)]
pub struct Flags {
	pub z: bool,
	pub n: bool,
	pub h: bool,
	pub c: bool,
}

#[derive(Debug)]
pub struct Registers {
	pub a: u8,
	pub b: u8,
	pub c: u8,
	pub d: u8,
	pub e: u8,
	pub f: Flags,
	pub h: u8,
	pub l: u8,
	pub pc: u16,
	pub sp: u16,
}

impl Registers {
	pub fn new() -> Self {
		Self {
			a: 0,
			b: 0,
			c: 0,
			d: 0,
			e: 0,
			f: Flags{..Default::default()},
			h: 0,
			l: 0,
			pc: 0x0100,
			sp: 0xFFFE,
		}
	}
	
	pub fn set_f(&mut self, value: u8){
		self.f.z = util::get_bit(value,7);
		self.f.n = util::get_bit(value,6);
		self.f.h = util::get_bit(value,5);
		self.f.c = util::get_bit(value,4);
	}
	pub fn get_f(&self) -> u8 {
		let mut value: u8 = 0;
		value = util::set_bit(value, 7, self.f.z);
		value = util::set_bit(value, 6, self.f.n);
		value = util::set_bit(value, 5, self.f.h);
		value = util::set_bit(value, 4, self.f.c);
		value
	}

	pub fn set_af(&mut self, value: u16) {
		let b = value.to_be_bytes();
		self.a = b[0];
		self.set_f(b[1]);
	}
	pub fn set_bc(&mut self, value: u16) {
		let b = value.to_be_bytes();
		self.b = b[0];
		self.c = b[1];
	}
	pub fn set_de(&mut self, value: u16) {
		let b = value.to_be_bytes();
		self.d = b[0];
		self.e = b[1];
	}
	pub fn set_hl(&mut self, value: u16) {
		let b = value.to_be_bytes();
		self.h = b[0];
		self.l = b[1];
	}

	pub fn get_af(&self) -> u16 {
		u16::from_be_bytes([self.a, self.get_f()])
	}
	pub fn get_bc(&self) -> u16 {
		u16::from_be_bytes([self.b, self.c])
	}
	pub fn get_de(&self) -> u16 {
		u16::from_be_bytes([self.d, self.e])
	}
	pub fn get_hl(&self) -> u16 {
		u16::from_be_bytes([self.h, self.l])
	}

	pub fn set_by_id(&mut self, id: u8, v: u8) -> bool{
		match id {
			0 => { self.b = v; true }
			1 => { self.c = v; true }
			2 => { self.d = v; true }
			3 => { self.e = v; true }
			4 => { self.h = v; true }
			5 => { self.l = v; true }
			6 => { false }
			7 => { self.a = v; true }
			_ => { false }
		}
	}

	pub fn get_by_id(&self, id: u8) -> Option<u8> {
		match id {
			0 => { Some(self.b) }
			1 => { Some(self.c) }
			2 => { Some(self.d) }
			3 => { Some(self.e) }
			4 => { Some(self.h) }
			5 => { Some(self.l) }
			6 => { None }
			7 => { Some(self.a) }
			_ => { None }
		}
	}

	pub fn set_union_by_id(&mut self, id: u8, v: u16) -> bool {
		match id {
			0 => { self.set_bc(v); true }
			1 => { self.set_de(v); true }
			2 => { self.set_hl(v); true }
			3 => { self.set_af(v); true }
			_ => { false }
		}
	}
	pub fn get_union_by_id(&self, id: u8) -> Option<u16> {
		match id {
			0 => { Some(self.get_bc()) }
			1 => { Some(self.get_de()) }
			2 => { Some(self.get_hl()) }
			3 => { Some(self.get_af()) }
			_ => { None }
		}
	}
}