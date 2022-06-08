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
    fn _dmb();
}

pub fn dmb() {
    unsafe {
        _dmb();
    }
}

extern "C" {
    fn _dsb();
}

pub fn dsb() {
    unsafe {
        _dsb();
    }
}

/// NOP instr
extern "C" {
    fn _nop();
}

pub fn nop() {
    unsafe {
        _nop();
    }
}

/// Memory barriers
extern "C" {
    fn _dev_barrier();
}

pub fn dev_barrier() {
    unsafe { _dev_barrier() };
}
