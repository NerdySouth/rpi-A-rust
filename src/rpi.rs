use core::arch::global_asm;

global_asm!(include_str!(r#"put-get.S"#));


extern "C" {
	pub fn PUT32(addr: usize, val: u32);
}


extern "C" {
	pub fn GET32(addr: usize) -> u32;
}

extern "C" {
	pub fn nop();
}