use crate::gb::{
	mem::Memory,
	reg::Registers,
	util
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
		let value: u8 = mem.read(self.reg.pc);
		self.reg.pc = self.reg.pc.wrapping_add(1);
		value
	}
	fn read_word(&mut self, mem: &Memory) -> u16 {
		let h: u8 = self.read_byte(mem);
		let l: u8 = self.read_byte(mem);
		u16::from_be_bytes([l,h])
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
			0x22 => {
				//LD (HL+),A
				let hl: u16 = self.reg.get_hl();
				mem.write(hl, self.reg.a);
				self.reg.set_hl(hl.wrapping_add(1));
				8
			}
			0x32 => {
				//LD (HL-),A
				let hl: u16 = self.reg.get_hl();
				mem.write(hl, self.reg.a);
				self.reg.set_hl(hl.wrapping_sub(1));
				8
			}

			// LD A,(rr)

			0x0A => {
				//LD A,(BC)
				self.reg.a = mem.read(self.reg.get_hl());
				8
			}
			0x1A => {
				//LD A,(DE)
				self.reg.a = mem.read(self.reg.get_de());
				8
			}
			0x2A => {
				//LD A,(HL+)
				let hl: u16 = self.reg.get_hl();
				self.reg.a = mem.read(hl);
				self.reg.set_hl(hl.wrapping_add(1));
				8
			}
			0x3A => {
				//LD A,(HL-)
				let hl: u16 = self.reg.get_hl();
				self.reg.a = mem.read(hl);
				self.reg.set_hl(hl.wrapping_sub(1));
				8
			}

			// LD r,u8

			0x06 => {
				//LD B,u8
				self.reg.b = self.read_byte(mem);
				8
			}
			0x0E => {
				//LD B,u8
				self.reg.c = self.read_byte(mem);
				8
			}
			0x16 => {
				//LD D,u8
				self.reg.d = self.read_byte(mem);
				8
			}
			0x1E => {
				//LD E,u8
				self.reg.e = self.read_byte(mem);
				8
			}
			0x26 => {
				//LD H,u8
				self.reg.h = self.read_byte(mem);
				8
			}
			0x2E => {
				//LD L,u8
				self.reg.l = self.read_byte(mem);
				8
			}
			0x36 => {
				//LD (HL),u8
				mem.write(self.reg.get_hl(), self.read_byte(mem));
				8
			}
			0x3E => {
				//LD A,u8
				self.reg.a = self.read_byte(mem);
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

			// DEC u8
			
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
			
			// JP
			0xC3 => {
				//JP u16
				self.reg.pc = self.read_word(mem);
				16
			}
			0xC4 => {
				//JP NZ,u16
				let to = self.read_word(mem);
				if !self.reg.f.z {
					self.reg.pc = to;
					24
				} else { 12 }
			}
			0xD4 => {
				//JP NC,u16
				let to = self.read_word(mem);
				if !self.reg.f.c {
					self.reg.pc = to;
					24
				} else { 12 }
			}
			0xCA => {
				//JP Z,u16
				let to = self.read_word(mem);
				if self.reg.f.z {
					self.reg.pc = to;
					24
				} else { 12 }
			}
			0xDA => {
				//JP C,u16
				let to = self.read_word(mem);
				if self.reg.f.c {
					self.reg.pc = to;
					24
				} else { 12 }
			}
			0xE9 => {
				//JP HL
				self.reg.pc = self.reg.get_hl();
				4
			}

			0x40..=0x75 | 0x77..=0x7F => {
				unimplemented!();
				let from = op & 7;
				let to = (op & 0x38) >> 3;
				let v: u8 = match from {
					0 => self.reg.b,
					1 => self.reg.c,
					2 => self.reg.d,
					3 => self.reg.e,
					4 => self.reg.h,
					5 => self.reg.l,
					6 => mem.read(self.reg.get_hl()),
					_ => { panic!(); }
				};
				4
			}

			0xCB => {
				let cb_op = self.read_byte(mem);
				match cb_op {
					0x40..=0xFF => {
						let h: u8 = (cb_op & 0xC0) >> 6; // type of bit op
						let r: u8 = cb_op & 7;			 	// register
						let b: u8 = (cb_op & 0x38) >> 3; // bit
						match h {
							1 => {
								// BIT
								self.reg.f.n = false;
								self.reg.f.h = true;
								match r {
									0 => { self.reg.f.z = !util::get_bit(self.reg.b, b); 8 }
									1 => { self.reg.f.z = !util::get_bit(self.reg.c, b); 8 }
									2 => { self.reg.f.z = !util::get_bit(self.reg.d, b); 8 }
									3 => { self.reg.f.z = !util::get_bit(self.reg.e, b); 8 }
									4 => { self.reg.f.z = !util::get_bit(self.reg.h, b); 8 }
									5 => { self.reg.f.z = !util::get_bit(self.reg.l, b); 8 }
									6 => { self.reg.f.z = !util::get_bit(mem.read(self.reg.get_hl()), b); 16 }
									7 => { self.reg.f.z = !util::get_bit(self.reg.a, b); 8 }
									_ => { panic!(); }
								}
							}
							2 | 3 => {
								//RES, SET
								let v = h==3;
								match r {
									0 => { util::set_bit(self.reg.b, b, v); 8 }
									1 => { util::set_bit(self.reg.c, b, v); 8 }
									2 => { util::set_bit(self.reg.d, b, v); 8 }
									3 => { util::set_bit(self.reg.e, b, v); 8 }
									4 => { util::set_bit(self.reg.h, b, v); 8 }
									5 => { util::set_bit(self.reg.l, b, v); 8 }
									6 => { let a = self.reg.get_hl(); mem.write(a, util::set_bit(mem.read(a), b, v)); 16 }
									7 => { util::set_bit(self.reg.a, b, v); 8 }
									_ => { panic!(); }
								}
							}
							_ => { panic!(); }
						}
					}

					_ => {
						panic!("Opcode not implemented: CB {:X}", op)
					}
				}
			}

			_ => {
				panic!("Opcode not implemented: {:X}", op)
			}
		}
	}
}