// trick to stick contents of start.S here inline
#![no_std]
#![no_main]
#![crate_type = "staticlib"]

use core::panic::PanicInfo;
use core::{arch::global_asm, ptr};
use nox::{gpio, rpi, timer, uart, bootloader};

global_asm!(include_str!(r#"./asm/start.S"#));

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub fn println(msg: &'static str) {
	let len = msg.len();
	let bytes = msg.as_bytes();
    for i in 0..len {
        unsafe {
            bootloader::boot_put32(bytes[i] as u32);
        }
    }
}



#[no_mangle]
pub fn _rmain() -> ! {
    unsafe {
        rpi::dev_barrier();
        rpi::PUT32(0x20215004, 0);
        rpi::dev_barrier();

        //zero __bss_start__
        extern "C" {
            static mut __bss_start: u8;
            static mut __bss_end: u8;
            static mut __data_start: u8;
            static mut __data_end: u8;
            static mut _sidata: u8;
        }

        let count = &__bss_end as *const u8 as usize - &__bss_start as *const u8 as usize;
        ptr::write_bytes(&mut __bss_start as *mut u8, 0, count);

        let count = &__data_end as *const u8 as usize - &__data_start as *const u8 as usize;
        ptr::copy_nonoverlapping(&_sidata as *const u8, &mut __data_start as *mut u8, count);

        uart::uart_init();
        let pin = 21;
        gpio::gpio_set_output(pin);
        gpio::blink_n(pin, 500, 3);
        //let code = nox::get_code();

        timer::delay_ms(500);
        //match code {
        //Ok(code) => {
        //rpi::BRANCHTO(code);
        //  nox::reboot();
        //}
        //  Err(boot_err) => nox::reboot(),
        //}
        //
        let hello: [u8; 6] = [104, 101, 108, 108, 111, 16];
        for i in 0..6 {
            bootloader::boot_put32(hello[i] as u32);
        }
		#[used]
		static string: &'static str = "pee";
		println(string);
        nox::reboot()
    }
}
