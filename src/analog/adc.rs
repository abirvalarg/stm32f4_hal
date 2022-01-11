use crate::volatile::*;

#[repr(C)]
#[allow(non_snake_case)]
struct ADCReg {
	SR: usize,
	CR1: usize,
	CR2: usize,
	SMPR1: usize,
	SMPR2: usize,
	JOFR1: usize,
	JOFR2: usize,
	JOFR3: usize,
	JOFR4: usize,
	HTR: usize,
	LTR: usize,
	SQR1: usize,
	SQR2: usize,
	SQR3: usize,
	JSQR: usize,
	JDR1: usize,
	JDR2: usize,
	JDR3: usize,
	JDR4: usize,
	DR: usize,
	CSR: usize,
	CCR: usize,
	CDR: usize
}

pub struct ADC(*mut ADCReg);

impl ADC {
	pub const unsafe fn new(addr: usize) -> ADC {
		ADC(addr as *mut ADCReg)
	}

	pub fn resolution(&mut self, res: Resolution) -> &mut Self {
		unsafe {
			let cr1 = volatile_read(&(*self.0).CR1) & !(0b11 << 24);
			volatile_write(&mut (*self.0).CR1, cr1 | (res as usize) << 24);
		}
		self
	}

	pub fn start(&mut self) -> &mut Self {
		unsafe {
			let cr2 = volatile_read(&(*self.0).CR2);
			volatile_write(&mut (*self.0).CR2, cr2 | 1);
		}
		self
	}

	pub fn stop(&mut self) {
		unsafe {
			let cr2 = volatile_read(&(*self.0).CR2);
			volatile_write(&mut (*self.0).CR2, cr2 & !1);
		}
	}

	pub fn read(&mut self, channel: usize) -> usize {
		unsafe {
			volatile_write(&mut (*self.0).SQR3, channel & 0x1f);
			let cr2 = volatile_read(&(*self.0).CR2);
			volatile_write(&mut (*self.0).CR2, cr2 | (1 << 30));
			while volatile_read(&(*self.0).SR) & 0b10 == 0 {}
			volatile_read(&(*self.0).DR)
		}
	}
}

pub enum Resolution {
	Bits12 = 0b00,
	Bits10 = 0b01,
	Bits8 = 0b10,
	Bits6 = 0b11
}
