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
		self.reg.f.h = ((i16::from(v) & 0xF) + (i16::from(a) & 0xF)) > 0xF;
		self.reg.f.n = false;
	}

	fn set_carry_sub(&mut self, v: u8, a: u8) {
		self.reg.f.c = !(v.checked_sub(a).is_some());
	}
	fn set_flags_sub(&mut self, v: u8, a: u8) {
		self.reg.f.z = v == 0;
		self.reg.f.h = ((i16::from(v) & 0xF) - (i16::from(a) & 0xF)) < 0x0;
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

	fn push_byte(&mut self, mem: &mut Memory, val: u8) {
		self.reg.sp = self.reg.sp.wrapping_sub(1);
		mem.write(self.reg.sp, val);
	}
	fn pop_byte(&mut self, mem: &Memory) -> u8 {
		self.reg.sp = self.reg.sp.wrapping_add(1);
		mem.read(self.reg.sp)
	}
	fn push_word(&mut self, mem: &mut Memory, val: u16) {
		let b = val.to_be_bytes();
		self.push_byte(mem, b[0]);
		self.push_byte(mem, b[1]);
	}
	fn pop_word(&mut self, mem: &Memory) -> u16 {
		let h = self.pop_byte(mem);
		let l = self.pop_byte(mem);
		u16::from_be_bytes([l,h])
	}

	fn call(&mut self, mem: &mut Memory, addr: u16) {
		self.push_word(mem, self.reg.pc);
		self.reg.pc = addr;
	}
	fn ret(&mut self, mem: &mut Memory) {
		let addr = self.pop_word(mem);
		self.reg.pc = addr;
	}

	pub fn step(&mut self, mem: &mut Memory) -> i8 {
		let op = self.read_byte(mem);
		self.exec(op, mem)
	}

	fn exec(&mut self, op: u8, mem: &mut Memory) -> i8 {
		match op {
			0x00 => {
				//NOP
				4
			}

			0x01 | 0x11 | 0x21 | 0x31 => {
				// LD rr,u16
				let val = self.read_word(mem);
				let reg = (op & 0xF0) >> 4;
				match reg {
					0 => { self.reg.set_bc(val); }
					1 => { self.reg.set_de(val); }
					2 => { self.reg.set_hl(val); }
					3 => { self.reg.sp = val; }
					_ => { unreachable!(); }
				}
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
				self.reg.a = mem.read(self.reg.get_bc());
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

			0xE0 => {
				//LD (FF00+u8),A
				mem.write(0xFF00 + u16::from(self.read_byte(mem)), self.reg.a);
				12
			}
			0xF0 => {
				//LD A,(FF00+u8)
				self.reg.a = mem.read(0xFF00 + u16::from(self.read_byte(mem)));
				12
			}

			// INC/DEC r16

			0x03 | 0x13 | 0x23 | 0x33 => {
				// INC r16
				let reg = (op & 0xF0) >> 4;
				match reg {
					0 => { self.reg.set_bc(self.reg.get_bc().wrapping_add(1)); }
					1 => { self.reg.set_de(self.reg.get_de().wrapping_add(1)); }
					2 => { self.reg.set_hl(self.reg.get_hl().wrapping_add(1)); }
					3 => { self.reg.sp = self.reg.sp.wrapping_add(1); }
					_ => { unreachable!(); }
				}
				8
			}
			
			0x0B | 0x1B | 0x2B | 0x3B => {
				// DEC r16
				let reg = (op & 0xF0) >> 4;
				match reg {
					0 => { self.reg.set_bc(self.reg.get_bc().wrapping_sub(1)); }
					1 => { self.reg.set_de(self.reg.get_de().wrapping_sub(1)); }
					2 => { self.reg.set_hl(self.reg.get_hl().wrapping_sub(1)); }
					3 => { self.reg.sp = self.reg.sp.wrapping_sub(1); }
					_ => { unreachable!(); }
				}
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
			0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D => {
				let t;
				let ov;
				let reg = (op >> 3) & 0x7;
				if reg==6 {
					t = 8;
					let a = self.reg.get_hl();
					ov = mem.read(a);
					mem.write(a, ov.wrapping_sub(1));
				} else {
					t = 4;
					ov = self.reg.get_by_id(reg).unwrap();
					self.reg.set_by_id(reg, ov.wrapping_sub(1));
				}
				self.set_flags_sub(ov, 1);
				t
			}

			// (OP) A, u8
			// TODO ADC,SBC,CP
			0xC6 => {
				//ADD A,u8
				let v = self.read_byte(mem);
				self.set_flags_add(self.reg.a, v);
				self.set_carry_add(self.reg.a, v);
				self.reg.a = self.reg.a.wrapping_add(v);
				8
			}
			0xD6 => {
				//SUB A,u8
				let v = self.read_byte(mem);
				self.set_flags_sub(self.reg.a, v);
				self.set_carry_sub(self.reg.a, v);
				self.reg.a = self.reg.a.wrapping_sub(v);
				8
			}
			0xE6 => {
				//AND A,u8
				self.reg.a &= self.read_byte(mem);
				self.reg.f.z = self.reg.a==0;
				self.reg.f.h = true;
				self.reg.f.c = false;
				self.reg.f.n = false;
				8
			}
			0xF6 => {
				//OR A,u8
				self.reg.a |= self.read_byte(mem);
				self.reg.f.z = self.reg.a==0;
				self.reg.f.h = false;
				self.reg.f.c = false;
				self.reg.f.n = false;
				8
			}
			0xEE => {
				//XOR A,u8
				self.reg.a ^= self.read_byte(mem);
				self.reg.f.z = self.reg.a==0;
				self.reg.f.h = false;
				self.reg.f.c = false;
				self.reg.f.n = false;
				8
			}
			
			// JP
			0xC3 => {
				//JP u16
				self.reg.pc = self.read_word(mem);
				16
			}
			0xE9 => {
				//JP HL
				self.reg.pc = self.reg.get_hl();
				4
			}
			0xC2 => {
				//JP NZ,u16
				let to = self.read_word(mem);
				if !self.reg.f.z {
					self.reg.pc = to;
					24
				} else { 12 }
			}
			0xD2 => {
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

			// JR

			0x18 => {
				//JR i8
				let o = util::to_signed(self.read_byte(mem));
				self.reg.pc = util::u16_offset(self.reg.pc, o);
				12
			}
			0x20 => {
				//JR NZ,i8
				let o = util::to_signed(self.read_byte(mem));
				if !self.reg.f.z {
					self.reg.pc = util::u16_offset(self.reg.pc, o);
					12
				} else { 8 }
			}
			0x30 => {
				//JR NC,i8
				let o = util::to_signed(self.read_byte(mem));
				if !self.reg.f.c {
					self.reg.pc = util::u16_offset(self.reg.pc, o);
					12
				} else { 8 }
			}
			0x28 => {
				//JR Z,i8
				let o = util::to_signed(self.read_byte(mem));
				if self.reg.f.z {
					self.reg.pc = util::u16_offset(self.reg.pc, o);
					12
				} else { 8 }
			}
			0x38 => {
				//JR C,i8
				let o = util::to_signed(self.read_byte(mem));
				if self.reg.f.c {
					self.reg.pc = util::u16_offset(self.reg.pc, o);
					12
				} else { 8 }
			}

			//CALL

			0xCD => {
				//CALL u16
				let to = self.read_word(mem);
				self.call(mem, to);
				24
			}
			0xC4 => {
				//CALL NZ, u16
				let to = self.read_word(mem);
				if !self.reg.f.z {
					self.call(mem, to);
					24
				} else { 12 }
			}
			0xD4 => {
				//CALL NC, u16
				let to = self.read_word(mem);
				if !self.reg.f.c {
					self.call(mem, to);
					24
				} else { 12 }
			}
			0xCC => {
				//CALL Z, u16
				let to = self.read_word(mem);
				if self.reg.f.z {
					self.call(mem, to);
					24
				} else { 12 }
			}
			0xDC => {
				//CALL C, u16
				let to = self.read_word(mem);
				if self.reg.f.c {
					self.call(mem, to);
					24
				} else { 12 }
			}

			// RET

			0xC9 => {
				//RET
				self.ret(mem);
				16
			}
			0xC0 => {
				//RET NZ
				if !self.reg.f.z {
					self.ret(mem);
					20
				} else { 8 }
			}
			0xD0 => {
				//RET NC
				if !self.reg.f.c {
					self.ret(mem);
					20
				} else { 8 }
			}
			0xC8 => {
				//RET Z
				if self.reg.f.z {
					self.ret(mem);
					20
				} else { 8 }
			}
			0xD8 => {
				//RET C
				if self.reg.f.c {
					self.ret(mem);
					20
				} else { 8 }
			}
			
			// PUSH/POP r16

			0xC5 | 0xD5 | 0xE5 | 0xF5 => {
				// PUSH r16
				let reg = ((op & 0xF0) >> 4) - 0xC;
				let val = self.reg.get_union_by_id(reg).unwrap();
				self.push_word(mem, val);
				16
			}

			0xC1 | 0xD1 | 0xE1 | 0xF1 => {
				//POP r16
				let reg = ((op & 0xF0) >> 4) - 0xC;
				let val = self.pop_word(mem);
				self.reg.set_union_by_id(reg, val);
				12
			}

			0x80..=0xBF => {
				// (OP) A,r8
				let reg = op & 0x7;
				let aop = (op >> 3) & 0x7;
				let mut t = 4;
				let val = match reg {
					6 => { t = 8; mem.read(self.reg.get_hl()) }
					_ => { self.reg.get_by_id(reg).unwrap() }
				};
				match aop {
					0 => {
						// ADD
						self.set_flags_add(self.reg.a, val);
						self.set_carry_add(self.reg.a, val);
						self.reg.a = self.reg.a.wrapping_add(val);
					}
					1 => { unimplemented!(); }
					2 => {
						// SUB
						self.set_flags_sub(self.reg.a, val);
						self.set_flags_sub(self.reg.a, val);
						self.reg.a = self.reg.a.wrapping_sub(val);
					}
					3 => { unimplemented!(); }
					4 => {
						// AND
						self.reg.a &= val;
						self.reg.f.z = self.reg.a==0;
						self.reg.f.n = false;
						self.reg.f.h = true;
						self.reg.f.c = false;
					}
					5 => {
						// XOR
						self.reg.a ^= val;
						self.reg.f.z = self.reg.a==0;
						self.reg.f.n = false;
						self.reg.f.h = false;
						self.reg.f.c = false;
					}
					6 => {
						// OR
						self.reg.a |= val;
						self.reg.f.z = self.reg.a==0;
						self.reg.f.n = false;
						self.reg.f.h = false;
						self.reg.f.c = false;
					}
					7 => { unimplemented!(); }
					_ => { unreachable!(); }
				}
				return t
			}

			0x40..=0x75 | 0x77..=0x7F => {
				//LD r,r
				let t;
				let from = op & 0x7;
				let to = (op >> 3) & 0x7;
				let v: u8 = match from {
					6 => { t = 8; mem.read(self.reg.get_hl()) },
					_ => { t = 4; self.reg.get_by_id(from).unwrap() }
				};
				match to {
					6 => { mem.write(self.reg.get_hl(), v); 8 }
					_ => { self.reg.set_by_id(to, v); t }
				}
			}

			0xCB => {
				let cb_op = self.read_byte(mem);
				match cb_op {
					0x40..=0xFF => {
						let h: u8 = (cb_op & 0xC0) >> 6; // type of bit op
						let b: u8 = (cb_op & 0x38) >> 3; // bit
						let r: u8 = cb_op & 7;			 // register
						match h {
							1 => {
								// BIT
								self.reg.f.n = false;
								self.reg.f.h = true;
								if r==6 {
									self.reg.f.z = !util::get_bit(mem.read(self.reg.get_hl()), b);
									16
								} else {
									let v = self.reg.get_by_id(r).unwrap();
									self.reg.f.z = !util::get_bit(v, b);
									8
								}
							}
							2 | 3 => {
								//RES, SET
								let v = h==3;
								if r==6 {
									let a = self.reg.get_hl();
									mem.write(a, util::set_bit(mem.read(a), b, v));
									16
								} else {
									let rv = self.reg.get_by_id(r).unwrap();
									self.reg.set_by_id(r, util::set_bit(rv, b, v));
									8
								}
							}
							_ => { unreachable!(); }
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