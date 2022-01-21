use crate::volatile::*;

#[repr(C)]
#[allow(non_snake_case)]
struct Reg {
	IMR: usize,
	EMR: usize,
	RTSR: usize,
	FTSR: usize,
	SWIER: usize,
	PR: usize,
}

pub struct Exti(*mut Reg);

impl Exti {
	pub const unsafe fn new(addr: usize) -> Self {
		Exti(addr as *mut Reg)
	}

	pub fn set_channel(&mut self, channel: u8, mode: Edge) -> &mut Self {
		unsafe {
			if mode == Edge::Rising || mode == Edge::Both {
				let reg = volatile_read(&(*self.0).RTSR);
				volatile_write(&mut (*self.0).RTSR, reg | (1 << channel));
			}
			if mode == Edge::Falling || mode == Edge::Both {
				let reg = volatile_read(&(*self.0).FTSR);
				volatile_write(&mut (*self.0).FTSR, reg | (1 << channel));
			}
			let mask = volatile_read(&(*self.0).IMR);
			volatile_write(&mut (*self.0).IMR, mask | (1 << channel));
		}
		self
	}

	pub fn get_pending(&self) -> usize {
		unsafe {
			volatile_read(&(*self.0).PR)
		}
	}

	pub fn clear_pending(&mut self) {
		unsafe {
			volatile_write(&mut (*self.0).PR, self.get_pending());
		}
	}
}

#[derive(PartialEq)]
pub enum Edge {
	Rising,
	Falling,
	Both
}
