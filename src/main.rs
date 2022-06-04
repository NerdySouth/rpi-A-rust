// trick to stick contents of start.S here inline
#![no_std]
#![no_main]
#![crate_type = "staticlib"]

use core::panic::PanicInfo;
use core::{arch::global_asm, ptr};
use nox::{gpio, rpi, timer, uart};

global_asm!(include_str!(r#"./asm/start.S"#));

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub fn println(msg: &[u8]) {
    let len = msg.len();
    for i in 0..len {
        unsafe {
            nox::boot_put32(msg[i] as u32);
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

        uart::uart_init();
        let pin = 21;
        gpio::gpio_set_output(pin);
        nox::blink_n(pin, 500, 3);
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
            nox::boot_put32(hello[i] as u32);
        }
        let st = b"peeeeeeeeeee";
        println(b"pe");
        nox::reboot()
    }
}
