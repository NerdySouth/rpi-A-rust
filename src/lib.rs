#![no_std]
#![feature(const_trait_impl)]
pub mod gpio;

pub mod rpi;
pub mod crc32;
pub mod timer;
pub mod uart;
pub mod bootloader;
use core::ptr;

/// reboot functionality for the pi

// simple address definitions for reboot process
const PM_RSTC: usize = 0x2010001c;
const PM_WDOG: usize = 0x20100024;
const PM_PASSWORD: u32 = 0x5a000000;
const PM_RSTC_WRCFG_FULL_RESET: u32 = 0x00000020;
// never  going to return
#[no_mangle]
pub fn reboot() -> ! {
    rpi::dev_barrier();
    unsafe {
        for i in 0..1000000 {
            rpi::nop();
        }
        rpi::PUT32(PM_WDOG, PM_PASSWORD | 1);
        rpi::PUT32(PM_RSTC, PM_PASSWORD | PM_RSTC_WRCFG_FULL_RESET);
    }
    loop {}
}

