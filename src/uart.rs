use crate::gpio;
use crate::rpi;
use crate::timer::delay_ms;

const tx_pin: u8 = 14;
const rx_pin: u8 = 15;
const aux_enables: usize = 0x20215004;
const aux_mu_io_reg: usize = 0x20215040;
const aux_mu_ier_reg: usize = 0x20215044;
const aux_mu_iir_reg: usize = 0x20215048;
const aux_mu_lcr_reg: usize = 0x2021504C;
const aux_mu_mcr_reg: usize = 0x20215050;
const aux_mu_lsr_reg: usize = 0x20215054;
const aux_mu_cntl_reg: usize = 0x20215060;
const aux_mu_stat_reg: usize = 0x20215064;
const aux_mu_baud_reg: usize = 0x20215068;
const baud: u32 = 270;

const GPFSEL1: usize = 0x20200004;
const GPSET0:  usize = 0x2020001C;
const GPCLR0: usize =  0x20200028;
const GPPUD: usize = 0x20200094;
const GPPUDCLK0: usize = 0x20200098;

#[no_mangle]
pub unsafe fn uart_init() {
    rpi::dev_barrier();
    gpio::gpio_set_uart_func(tx_pin);
    gpio::gpio_set_uart_func(rx_pin);
    rpi::dev_barrier();

    //enable uart
    let mut enb_val: u32 = rpi::GET32(aux_enables);
    enb_val |= 0b1;
    rpi::PUT32(aux_enables, enb_val);

    rpi::dev_barrier();

    // disable fifos
    let mut cntl_val: u32 = rpi::GET32(aux_mu_cntl_reg);
    cntl_val &= 0b00;
    rpi::PUT32(aux_mu_cntl_reg, cntl_val);

    //clear fifos
    rpi::PUT32(aux_mu_iir_reg, 0b110 as u32);

    //disable interrupts
    rpi::PUT32(aux_mu_ier_reg, 0 as u32);

    //set 8bit mode
    rpi::PUT32(aux_mu_lcr_reg, 3 as u32);
    rpi::PUT32(aux_mu_mcr_reg, 0);

    //set baud
    rpi::PUT32(aux_mu_baud_reg, baud);

    rpi::PUT32(GPPUD, 0);
    delay_ms(200);
    rpi::PUT32(GPPUDCLK0, (1 << 14) | (1 << 15));
    delay_ms(200);
    rpi::PUT32(GPPUDCLK0, 0);



    //enable fifos
    let mut cntl_val: u32 = rpi::GET32(aux_mu_cntl_reg);
    cntl_val |= 0b11;
    rpi::PUT32(aux_mu_cntl_reg, cntl_val);
    rpi::dev_barrier();
}

// 	/// hardware MiniUart rx methods

// check if the rx fifo has a byte
#[no_mangle]
pub unsafe fn rx_has_byte() -> bool {
    rpi::dev_barrier();
    let rx_stat = rpi::GET32(aux_mu_lsr_reg) & 0b1;
    rpi::dev_barrier();
    match rx_stat {
        1 => true,
        _ => false,
    }
}

// get a byte from the rx fifo, block until one is available
#[no_mangle]
pub unsafe fn read_byte() -> u8 {
    rpi::dev_barrier();
    while !rx_has_byte() {
        rpi::nop();
    }
    // can now read from uart
    let val = (rpi::GET32(aux_mu_io_reg) & 0x00ff) as u8;
    rpi::dev_barrier();
    val
}

/// hardware mini uart tx methods

// space to fit at least one more byte in the tx fifo
#[no_mangle]
pub unsafe fn tx_space_available() -> bool {
    rpi::dev_barrier();
    let tx_lsr = rpi::GET32(aux_mu_stat_reg) & 0b10;
    rpi::dev_barrier();
    match tx_lsr {
        0 => false,
        _ => true,
    }
}

// check if miniuart tx fifo is empty
#[no_mangle]
pub unsafe fn tx_empty() -> bool {
    rpi::dev_barrier();
    let mut lsr_val = rpi::GET32(aux_mu_lsr_reg);
    rpi::dev_barrier();
    lsr_val &= (1 << 6);
    match lsr_val {
        0 => true,
        _ => false,
    }
}

// flush uart tx hardware fifo
#[no_mangle]
pub unsafe fn flush_tx() {
    rpi::dev_barrier();
    while !tx_empty() {
        rpi::nop();
    }
    rpi::dev_barrier();
}

//write byte to tx fifo
#[no_mangle]
pub unsafe fn write_byte(byte: u32) {
    rpi::dev_barrier();
    while !tx_space_available() {
        rpi::nop();
    }

    // can now put onto tx fifo
    rpi::PUT32(aux_mu_io_reg, byte);
    rpi::dev_barrier();
}

/// disable hardware mini uart

// disable miniuart hardware
#[no_mangle]
pub unsafe fn disable() {
    rpi::dev_barrier();

    flush_tx();

    rpi::dev_barrier();

    //disable uart
    let mut enb_val: u32 = rpi::GET32(aux_enables);
    enb_val &= 0b0;
    rpi::PUT32(aux_enables, enb_val);
    rpi::dev_barrier();
}

// pub struct MiniUart {
// 	uart: Option<MiniUartInternal>,
// }

// enum UartError {
// 	OFF,
// }

// impl MiniUart {
// 	pub fn new() -> MiniUart {
// 		unsafe {
// 			let mut uart: MiniUart = MiniUart {
// 				uart: Some(MiniUartInternal::new())
// 			};

// 			uart
// 		}
// 	}

// 	fn transmit_byte(&mut self, byte: u8) -> Result<(), UartError> {
// 		match &mut self.uart {
// 			Some(uart) => {
// 				// aready called new, have an initialized hardware instance
// 				unsafe {
// 					uart.write_byte(byte);
// 				}
// 				Ok(())
// 			},
// 			None => Err(UartError::OFF)
// 		}
// 	}

// 	fn transmit_bytes(&mut self, bytes: &[u8]) -> Result<(), UartError> {
// 		match &mut self.uart {
// 			Some(_uart) => {
// 				for byte in bytes.iter() {
// 					self.transmit_byte(*byte);
// 				}
// 				Ok(())
// 			}
// 			None => Err(UartError::OFF)
// 		}
// 	}

// 	fn receive_byte(&self) -> Result<u8, UartError> {
// 		match &self.uart {
// 			Some(uart) => {
// 				unsafe {
// 					Ok(uart.rx_get_byte())
// 				}
// 			}
// 			None => Err(UartError::OFF)
// 		}
// 	}
// }
