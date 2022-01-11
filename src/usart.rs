use crate::volatile::*;
use crate::Box;

#[repr(C)]
#[allow(non_snake_case)]
struct Reg {
	SR: usize,
	DR: usize,
	BRR: usize,
	CR1: usize,
	CR2: usize,
	CR3: usize,
	GTPR: usize
}

pub struct Usart {
	hw: *mut Reg,
	rx_handler: Option<Box<dyn FnMut()>>,
	tx_empty_handler: Option<Box<dyn FnMut()>>
}

impl Usart {
	pub const unsafe fn new(addr: usize) -> Usart {
		Usart {
			hw: addr as *mut Reg,
			rx_handler: None,
			tx_empty_handler: None
		}
	}

	pub fn enable(&mut self, state: bool) -> &mut Self {
		unsafe {
			let mut cr1 = volatile_read(&(*self.hw).CR1) & !(1 << 13);
			if state {
				cr1 |= 1 << 13;
			}
			volatile_write(&mut (*self.hw).CR1, cr1);
		}
		self
	}

	pub fn tx_enable(&mut self, state: bool) -> &mut Self {
		unsafe {
			let mut cr1 = volatile_read(&(*self.hw).CR1) & !(1 << 3);
			if state {
				cr1 |= 1 << 3;
			}
			volatile_write(&mut (*self.hw).CR1, cr1);
		}
		self
	}

	pub fn rx_enable(&mut self, state: bool) -> &mut Self {
		unsafe {
			let mut cr1 = volatile_read(&(*self.hw).CR1) & !(1 << 2);
			if state {
				cr1 |= 1 << 2;
			}
			volatile_write(&mut (*self.hw).CR1, cr1);
		}
		self
	}

	pub fn on_rx<F: FnMut() + 'static>(&mut self, h: F) -> &mut Self {
		self.rx_handler = Some(Box::new(h));
		unsafe {
			let cr1 = volatile_read(&(*self.hw).CR1);
			volatile_write(&mut (*self.hw).CR1, cr1 | (1 << 5));
		}
		self
	}

	pub fn on_tx_empty<F: FnMut() + 'static>(&mut self, h: F) -> &mut Self {
		self.tx_empty_handler = Some(Box::new(h));
		unsafe {
			let cr1 = volatile_read(&(*self.hw).CR1);
			volatile_write(&mut (*self.hw).CR1, cr1 | (1 << 7));
		}
		self
	}

	pub fn baudrate(&mut self, brr: usize) -> &mut Self {
		unsafe {
			volatile_write(&mut (*self.hw).BRR, brr);
		}
		self
	}

	pub fn check_state(&mut self) {
		let sr = unsafe { volatile_read(&(*self.hw).SR) };
		if sr & (1 << 7) != 0 { // TXE
			if let Some(h) = &mut self.tx_empty_handler {
				h();
			}
		}
		if sr & (1 << 5) != 0 {	// RXNE
			if let Some(h) = &mut self.rx_handler {
				h();
			}
		}
	}
}

impl crate::abstr::comm::Communication<u8> for Usart {
    fn send(&mut self, data: u8) {
        unsafe {
			while volatile_read(&(*self.hw).SR) & (1 << 7) == 0 {}
			volatile_write(&mut (*self.hw).DR, data as usize);
		}
    }

    fn recv(&mut self) -> u8 {
		unsafe {
			while volatile_read(&(*self.hw).SR) & (1 << 5) == 0 {}
			volatile_read(&(*self.hw).DR) as u8
		}
    }
}
