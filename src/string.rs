use core::ops::Add;

use crate::malloc::*;

pub struct String {
	data: *mut u8,
	len: usize,
	capacity: usize
}

impl String {
	pub const fn new() -> Self {
		String {
			data: 0 as *mut _,
			len: 0,
			capacity: 0
		}
	}
}

impl From<&str> for String {
	fn from(value: &str) -> Self {
		unsafe {
			let addr = malloc(value.len()) as *mut _;
			if addr == core::ptr::null_mut() {
				panic!("out of heap memory");
			}
			let container = core::slice::from_raw_parts_mut(addr, value.len());
			for (idx, val) in value.as_bytes().iter().enumerate() {
				container[idx] = *val;
			}
			String {
				data: addr,
				len: value.len(),
				capacity: value.len(),
			}
		}
	}
}

impl core::ops::Deref for String {
	type Target = str;
	fn deref(&self) -> &Self::Target {
		unsafe {
			core::str::from_utf8(core::slice::from_raw_parts(self.data, self.len)).unwrap()
		}
	}
}

impl Drop for String {
	fn drop(&mut self) {
		unsafe {
			if self.data != core::ptr::null_mut() {
				free(self.data as _);
			}
		}
	}
}

pub trait ToString {
	fn to_string(&self) -> String;
}

impl ToString for u32 {
	fn to_string(&self) -> String {
		let mut val = *self;
		let mut buffer = [0; 32];
		let mut num_digits = 0;
		while val != 0 {
			let digit = val % 10;
			val /= 10;
			buffer[31 - num_digits] = digit as u8 + '0' as u8;
			num_digits += 1;
		}
		if num_digits == 0 {
			"0".into()
		} else {
			core::str::from_utf8(&buffer[31 - num_digits..]).unwrap().into()
		}
	}
}
