use crate::rpi;

const TIMER_ADDR: usize = 0x20003004;

pub fn timer_get_usec() -> u32 {
	unsafe { rpi::GET32(TIMER_ADDR) }
}

pub fn delay_us(usecs: u32) {
	let start = timer_get_usec();
	let mut curr = timer_get_usec();
	while curr - start < usecs {
		rpi::nop();
		curr = timer_get_usec();
	}
}

pub fn delay_ms(msecs: u32) {
	delay_us(1000 * msecs);
}