pub fn get_bit(a: u8, b: u8) -> bool {
	a & (1 << b) != 0
}
pub fn set_bit(a: u8, b: u8, v: bool) -> u8 {
	(a & !(1u8 << b)) | ((v as u8) << b)
}