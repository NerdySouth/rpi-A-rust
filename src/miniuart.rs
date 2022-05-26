pub struct MiniUartInternal<'a> {
	tx_pin: &'a u8,
	rx_pin:  u8,
	aux_enables: &'static usize,
	aux_mu_io_reg: &'static usize,
	aux_mu_ier_reg: &'static usize,
	aux_mu_iir_reg: &'static usize,
	aux_mu_lcr_reg: &'static usize,
	aux_mu_lsr_reg: &'static usize,
	aux_mu_cntl_reg: &'static usize,
	aux_mu_stat_reg: &'static usize,
	aux_mu_baud_reg: &'static usize,
	baud: u32,
}



impl<'a> Default for MiniUartInternal<'a> {
	fn default() -> MiniUartInternal<'a> {
		MiniUartInternal {
			tx_pin: 14,
			rx_pin: 15,
			aux_enables: 0x20215004,
			aux_mu_io_reg: 0x20215040,
			aux_mu_ier_reg: 0x20215044,
			aux_mu_iir_reg: 0x20215048,
			aux_mu_lcr_reg: 0x2021504C,
			aux_mu_lsr_reg: 0x20215054,
			aux_mu_cntl_reg: 0x20215060,
			aux_mu_stat_reg: 0x20215064,
			aux_mu_baud_reg: 0x20215068,
			baud: 270, // gives us 115200 baud
		}
	}
}

impl<'a> MiniUartInternal<'a> {
	pub const fn new() -> MiniUartInternal<'a> {
		MiniUartInternal::default()
	}
}