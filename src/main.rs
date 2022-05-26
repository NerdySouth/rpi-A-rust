// trick to stick contents of start.S here inline
#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;
use nox::{gpio, rpi, uart, timer};

global_asm!(include_str!(r#"./asm/start.S"#));

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// #[no_mangle]
// pub fn _bootmain() -> ! {
// 	unsafe {
// 		rpi::dev_barrier();
//    		rpi::PUT32(0x20215004, 0);
// 		rpi::dev_barrier();

// 		let err_pin = 21;
// 		let succ_pin = 20;
// 		gpio::gpio_set_output(err_pin);
// 		gpio::gpio_set_output(succ_pin);
// 		nox::blink(err_pin, 3000);
// 		nox::blink(succ_pin, 3000);

// 		uart::uart_init();
// 		let code = nox::get_code(err_pin, succ_pin);
// 		match code {
// 			Ok(code)  => rpi::BRANCHTO(code),
// 			Err(boot_err) => nox::reboot()
// 		}
// 	}
//     loop {}
// }

#[no_mangle]
pub fn _rmain() -> ! {
	unsafe {
		rpi::dev_barrier();
   		rpi::PUT32(0x20215004, 0);
		rpi::dev_barrier();

		let pin = 20;
		gpio::gpio_set_output(pin);

		uart::uart_init();
		rpi::dev_barrier();
		timer::delay_ms(200);
		uart::write_byte(0xab);
		uart::write_byte(0xab);
		let hello: [u8; 7] = [104, 101, 108, 108, 111, 111, 10];
		for byte in hello {
			nox::blink(pin, 500);
			uart::write_byte(byte as u32);
		}

		for byte in hello {
			nox::blink(pin, 500);
			uart::write_byte(byte as u32);
		}
		nox::reboot();
	}
    loop {}
}