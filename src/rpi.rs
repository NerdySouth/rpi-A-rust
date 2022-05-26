use core::arch::global_asm;

global_asm!(include_str!("./asm/put-get.S"));
global_asm!(include_str!(r#"./asm/mem-barrier.S"#));

// Basic  PUT and GET for 32 bit words
extern "C" {
    pub fn PUT32(addr: usize, val: u32);
}

// Basic  PUT and GET for 32 bit words
extern "C" {
    pub fn PUT8(addr: usize, val: u8);
}

extern "C" {
    pub fn GET32(addr: usize) -> u32;
}

extern "C" {
    pub fn BRANCHTO(addr: u32);
}


extern "C" {
    fn dmb_c();
}

pub fn dmb() {
	unsafe { dmb_c(); }
}

extern "C" {
    fn dsb_c();
}

pub fn dsb() {
	unsafe { dsb_c(); }
}

/// NOP instr
extern "C" {
    fn nop_c();
}

pub fn nop() {
	unsafe { nop_c(); }
}

/// Memory barriers
extern "C" {
    fn dev_barrier_c();
}

pub fn dev_barrier() {
    unsafe { dev_barrier_c() };
}

