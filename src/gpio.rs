use crate::rpi;
use crate::timer;

// constants for GPIO memory IO addresses
static GPIO_BASE: usize = 0x20200000;
static GPIO_SET0: usize = GPIO_BASE + 0x1c;
static GPIO_CLR0: usize = GPIO_BASE + 0x28;
static GPIO_LEV0: usize = GPIO_BASE + 0x34;

static NUM_PINS: u8 = 31;

// static constants of possible functions we can select for a gpio pin
pub static GPIO_FUNC_INPUT: u32 = 0;
pub static GPIO_FUNC_OUTPUT: u32 = 1;
pub static GPIO_FUNC_ALT0: u32 = 4;
pub static GPIO_FUNC_ALT1: u32 = 5;
pub static GPIO_FUNC_ALT2: u32 = 6;
pub static GPIO_FUNC_ALT3: u32 = 7;
pub static GPIO_FUNC_ALT4: u32 = 3;
pub static GPIO_FUNC_ALT5: u32 = 2;

// set gpio pin <pin> to gpio function  <func>
#[no_mangle]
unsafe fn gpio_set_function(pin: u8, func: u32) {
    // should not pass any higher pin numbers
    if pin > NUM_PINS {
        return;
    }
    if (func & 0b111) != func {
        return;
    }

    let off: u32 = ((pin % 10) * 3).into();
    let g: usize = GPIO_BASE + (pin as usize / 10) * 4;

    unsafe {
        let mut val = rpi::GET32(g);
        val &= !(0b111 << off);
        val |= (func) << off;
        rpi::PUT32(g, val);
    }
}

#[no_mangle]
pub fn gpio_set_uart_func(pin: u8) {
	unsafe {
		gpio_set_function(pin, GPIO_FUNC_ALT5);
	}
}

#[no_mangle]
pub fn gpio_set_output(pin: u8) {
	unsafe {
		gpio_set_function(pin, GPIO_FUNC_OUTPUT);
	}
}

#[no_mangle]
pub fn gpio_set_input(pin: u8) {
	unsafe {
		gpio_set_function(pin, GPIO_FUNC_INPUT);
	}
}

#[no_mangle]
pub fn gpio_set_on(pin: u8) {
    if pin > NUM_PINS {
        return;
    }

    unsafe {
        rpi::PUT32(GPIO_SET0, 1 << pin);
    }
}

#[no_mangle]
pub fn gpio_set_off(pin: u8) {
    if pin > NUM_PINS {
        return;
    }

    unsafe {
        rpi::PUT32(GPIO_CLR0, 1 << pin);
    }
}

#[no_mangle]
pub fn gpio_write(pin: u8, val: u8) {
    if pin > NUM_PINS {
        return;
    }

    if val == 1 {
        gpio_set_on(pin);
    } else if val == 0 {
        gpio_set_off(pin);
    }
}

#[no_mangle]
pub fn gpio_read(pin: u8) -> u8 {
    if pin > NUM_PINS {
        return 0xFF;
    }

    let bank = GPIO_LEV0 as u32 +( pin as u32)/ 32;
    let offset = pin % 32;
    let mut ret = unsafe { rpi::GET32(bank as usize) };
    ret >>= offset;
    ret &= 1;
    ret as u8
}


#[no_mangle]
pub fn blink(pin: u8, dur: u32) {
    gpio_set_on(pin);
    timer::delay_ms(dur / 2);
    gpio_set_off(pin);
    timer::delay_ms(dur / 2);
}

pub fn blink_n(pin: u8, dur: u32, n: u32) {
    for i in 0..n {
        blink(pin, dur);
    }
}