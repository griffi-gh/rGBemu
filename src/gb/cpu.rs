use crate::gb::{
	mem::Memory,
	reg::Registers,
};

#[derive(Debug)]
pub struct Cpu{
	pub reg: Registers,
}
impl Cpu {
	pub fn new() -> Self {
		Self {
			reg: Registers::new()
		}
	}

	fn set_carry_add(&mut self, v: u8, a: u8) {
		self.reg.f.c = !(v.checked_add(a).is_some());
	}
	fn set_flags_add(&mut self, v: u8, a: u8) {
		self.reg.f.z = v == 0;
		self.reg.f.h = ((v & 0xF) + (a & 0xF)) > 0xF;
		self.reg.f.n = false;
	}

	fn set_carry_sub(&mut self, v: u8, a: u8) {
		self.reg.f.c = !(v.checked_sub(a).is_some());
	}
	fn set_flags_sub(&mut self, v: u8, a: u8) {
		self.reg.f.z = v == 0;
		self.reg.f.h = ((v & 0xF) - (a & 0xF)) > 0xF;
		self.reg.f.n = true;
	}
	
	fn read_byte(&mut self, mem: &Memory) -> u8 {
		let value = mem.read(self.reg.pc);
		self.reg.pc += 1;
		value
	}
	fn read_word(&mut self, mem: &Memory) -> u16 {
		let h = self.read_byte(mem) as u16;
		let l = self.read_byte(mem) as u16;
		h + (l << 8)
	}

	pub fn step(&mut self, mem: &mut Memory) -> i8 {
		let op = self.read_byte(mem);
		self.exec(op, mem)
	}
	fn exec(&mut self, op: u8, mem: &mut Memory) -> i8 {
		//let reg = &mut self.reg;
		match op {
			0x00 => {
				//NOP
				4
			}

			// LD rr,u16

			0x01 => {
				//LD BC,u16
				let v = self.read_word(mem);
				self.reg.set_bc(v);
				12
			}
			0x11 => {
				//LD DE,u16
				let v = self.read_word(mem);
				self.reg.set_de(v);
				12
			}
			0x21 => {
				//LD HL,u16
				let v = self.read_word(mem);
				self.reg.set_hl(v);
				12
			}
			0x31 => {
				//LD SP,u16
				self.reg.sp = self.read_word(mem);
				12
			}

			0x02 => {
				//LD (BC),A
				mem.write(self.reg.get_bc(), self.reg.a);
				8
			}
			0x12 => {
				//LD (DE),A
				mem.write(self.reg.get_de(), self.reg.a);
				8
			}

			// INC rr

			0x03 => {
				//INC BC
				self.reg.set_bc(self.reg.get_bc().wrapping_add(1));
				8
			}
			0x13 => {
				//INC DE
				self.reg.set_de(self.reg.get_de().wrapping_add(1));
				8
			}
			0x23 => {
				//INC HL
				self.reg.set_hl(self.reg.get_hl().wrapping_add(1));
				8
			}
			0x33 => {
				//INC SP
				self.reg.sp = self.reg.sp.wrapping_add(1);
				8
			}

			// DEC rr

			0x0B => {
				//DEC BC
				self.reg.set_bc(self.reg.get_bc().wrapping_add(1));
				8
			}
			0x1B => {
				//DEC DE
				self.reg.set_de(self.reg.get_de().wrapping_add(1));
				8
			}
			0x2B => {
				//DEC HL
				self.reg.set_hl(self.reg.get_hl().wrapping_add(1));
				8
			}
			0x3B => {
				//DEC SP
				self.reg.sp = self.reg.sp.wrapping_add(1);
				8
			}

			//INC r

			0x04 => {
				//INC B
				let v: u8 = self.reg.b.wrapping_add(1);
				self.set_flags_add(v, 1);
				self.reg.b = v;
				4
			}
			0x0C => {
				//INC C
				let v: u8 = self.reg.c.wrapping_add(1);
				self.set_flags_add(v, 1);
				self.reg.c = v;
				4
			}
			0x14 => {
				//INC D
				let v: u8 = self.reg.d.wrapping_add(1);
				self.set_flags_add(v, 1);
				self.reg.d = v;
				4
			}
			0x1C => {
				//INC E
				let v: u8 = self.reg.e.wrapping_add(1);
				self.set_flags_add(v, 1);
				self.reg.e = v;
				4
			}
			0x24 => {
				//INC H
				let v: u8 = self.reg.h.wrapping_add(1);
				self.set_flags_add(v, 1);
				self.reg.h = v;
				4
			}
			0x2C => {
				//INC L
				let v: u8 = self.reg.l.wrapping_add(1);
				self.set_flags_add(v, 1);
				self.reg.l = v;
				4
			}
			0x34 => {
				//INC (HL)
				let a: u16 = self.reg.get_hl();
				let v: u8 = mem.read(a).wrapping_add(1);
				self.set_flags_add(v, 1);
				mem.write(a, v);
				12
			}
			0x3C => {
				//INC A
				let v: u8 = self.reg.a.wrapping_add(1);
				self.set_flags_add(v, 1);
				self.reg.a = v;
				4
			}

			// DEC r
			
			0x05 => {
				//DEC B
				let v: u8 = self.reg.b.wrapping_sub(1);
				self.set_flags_sub(v, 1);
				self.reg.b = v;
				4
			}
			0x0D => {
				//DEC C
				let v: u8 = self.reg.c.wrapping_sub(1);
				self.set_flags_sub(v, 1);
				self.reg.c = v;
				4
			}
			0x15 => {
				//DEC D
				let v: u8 = self.reg.d.wrapping_sub(1);
				self.set_flags_sub(v, 1);
				self.reg.d = v;
				4
			}
			0x1D => {
				//DEC E
				let v: u8 = self.reg.e.wrapping_sub(1);
				self.set_flags_sub(v, 1);
				self.reg.e = v;
				4
			}
			0x25 => {
				//DEC H
				let v: u8 = self.reg.h.wrapping_sub(1);
				self.set_flags_sub(v, 1);
				self.reg.h = v;
				4
			}
			0x2D => {
				//DEC L
				let v: u8 = self.reg.l.wrapping_sub(1);
				self.set_flags_sub(v, 1);
				self.reg.l = v;
				4
			}
			0x35 => {
				//DEC (HL)
				let a: u16 = self.reg.get_hl();
				let v: u8 = mem.read(a).wrapping_sub(1);
				self.set_flags_sub(v, 1);
				mem.write(a, v);
				12
			}
			0x3D => {
				//DEC A
				let v: u8 = self.reg.l.wrapping_sub(1);
				self.set_flags_sub(v,1);
				self.reg.a = v;
				4
			}

			_ => {
				panic!("Opcode not implemented: 0x{:X}", op)
			}
		}
	}
}