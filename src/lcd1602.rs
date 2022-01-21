use crate::gpio::{Pin, Slice};
use crate::abstr::timer::Timer;
use crate::Mutex;

pub struct Lcd<'a, Tim: Timer> {
	bus: Slice<4>,
	enable: Pin,
	reg_sel: Pin,
	height: u8,
	delay_timer: &'a Mutex<Tim>
}

impl<'a, Tim: Timer> Lcd<'a, Tim> {
	pub fn new(bus: Slice<4>, enable: Pin, reg_sel: Pin, height: u8, delay_timer: &'a Mutex<Tim>) -> Lcd<'a, Tim> {
		if height != 1 && height != 2 && height != 4 {
			panic!("unsupported height");
		}

		Lcd {
			bus,
			enable,
			reg_sel,
			height,
			delay_timer
		}
	}

	fn send(&mut self, data_reg: bool, data: u8) {
		let mut delay_timer = self.delay_timer.lock();

		self.reg_sel.write(data_reg);
		self.bus.write(((data >> 4) & 0xf) as usize);
		self.enable.write(true);
		delay_timer.start();
		while !delay_timer.clear_flag() {}
		self.enable.write(false);
		self.bus.write((data & 0xf) as usize);
		self.enable.write(true);
		delay_timer.start();
		while !delay_timer.clear_flag() {}
		self.enable.write(false);
		self.bus.write((data & 0xf) as usize);
	}

	pub fn init(&mut self, high_res: bool) {
		let mut tim = self.delay_timer.lock();
		tim.one_pulse(true);
		tim.set_reload(500);
		self.send(false,
			0b110000
			| ((self.height != 1) as u8) << 3
			| (high_res as u8) << 2
		);
		self.send(false, 0b10);
		self.send(false, 0b1);
		tim.set_reload(25000);
		tim.start();
		while !tim.clear_flag() {}
		tim.set_reload(500);
	}

	pub fn raw_char(&mut self, ch: u8) {
		self.send(true, ch);
	}

	pub fn put_str(&mut self, data: &str) {
		for ch in data.as_bytes() {
			self.raw_char(*ch);
		}
	}

	pub fn set_pos(&mut self, x: u32, y: u32) {
		let pos = match self.height {
			1 => x,
			2 => x + y * 20,
			4 => (match y {
					0 => 0,
					1 => 40,
					2 => 20,
					3 => 80,
					_ => panic!("bad y coordinate")
				}) + x,
			_ => panic!("unsupported height")
		};
		self.send(false, (1 << 7) | pos as u8);
	}
}
