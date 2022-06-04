// trick to stick contents of start.S here inline
#![no_std]
#![no_main]
#![feature(core_intrinsics)]

use core::arch::global_asm;
use core::intrinsics::black_box;
use core::mem;
use core::panic::PanicInfo;
use nox::{
    blink, gpio, rpi,
    timer::{self, delay_ms, delay_us},
    uart,
};

global_asm!(include_str!(r#"./asm/start.S"#));

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub fn _rmain() -> ! {
    unsafe {
        rpi::dev_barrier();
        rpi::PUT32(0x20215004, 0);
        rpi::dev_barrier();
        uart::uart_init();
        let pin = 21;
        gpio::gpio_set_output(pin);
        blink(pin, 250);
        let hello: [u8; 6] = [104, 101, 108, 108, 111, 16];
        for i in 0..hello.len() {
            nox::boot_put32(hello[i] as u32);
        }
        
        nox::reboot()
    }
}
