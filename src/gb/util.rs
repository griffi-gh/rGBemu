pub fn get_bit(a: u8, b: u8) -> bool {
	a & (1u8 << b) != 0
}
pub fn set_bit(a: u8, b: u8, v: bool) -> u8 {
	(a & !(1u8 << b)) | ((v as u8) << b)
}
pub fn to_signed(a: u8) -> i8 {
	(i16::from(a) - 0x100) as i8
}
pub fn u16_offset(a: u16, b: i8) -> u16 {
	(i32::from(a) + i32::from(b)) as u16
}