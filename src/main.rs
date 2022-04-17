// trick to stick contents of start.S here inline
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;
use nox::{rpi, gpio};

global_asm!(include_str!(r#"start.S"#));

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _rmain() -> ! {
	let led = 20;
	gpio::gpio_set_output(led);

	for i in 0..9 {
		gpio::gpio_set_on(led);
		let mut ticks1 = 1000000;
		while ticks1 > 0 {
			unsafe {
				rpi::nop();
			}
			ticks1 -= 1;
		}

		gpio::gpio_set_off(led);
		//delay
		let mut ticks2 = 1000000;
		while ticks2 > 0 {
			unsafe {
				rpi::nop();
			}
			ticks2 -= 1;
		}
	};

	loop {}
}