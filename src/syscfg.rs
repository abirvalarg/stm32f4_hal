use crate::volatile::*;

#[repr(C)]
#[allow(non_snake_case)]
struct Reg {
	MEMRMP: usize,
	PMC: usize,
	EXTICR1: usize,
	EXTICR2: usize,
	EXTICR3: usize,
	EXTICR4: usize,
	CMPCR: usize
}

pub struct Syscfg(*mut Reg);

impl Syscfg {
	#[cold]
	pub const unsafe fn new(addr: usize) -> Self {
		Syscfg(addr as *mut Reg)
	}

	#[cold]
	pub fn set_channel(&mut self, ctl: u8, channel: u8) -> &mut Self {
		unsafe {
			let reg = match channel / 4 {
				0 => &mut (*self.0).EXTICR1,
				1 => &mut (*self.0).EXTICR2,
				2 => &mut (*self.0).EXTICR3,
				3 => &mut (*self.0).EXTICR4,
				_ => panic!("Bad channel")
			};
			let val = volatile_read(reg) & !(0xf << (channel % 4));
			volatile_write(reg, val | ((ctl as usize & 0xf) << (channel % 4)));
		}
		self
	}
}
