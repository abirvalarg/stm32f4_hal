mod slice;
pub use slice::Slice;

use crate::volatile::*;
use crate::mutex::block_irq;

#[repr(C)]
#[allow(non_snake_case)]
struct GpioReg {
    MODER: usize,
    OTYPER: usize,
    OSPEEDR: usize,
    PUPDR: usize,
    IDR: usize,
    ODR: usize,
    BSRR: usize,
    LCKR: usize,
    AFRL: usize,
    AFRH: usize
}

pub struct Gpio(*mut GpioReg);

pub struct Pin {
    gpio: *mut GpioReg,
    pin: u8
}

#[non_exhaustive]
pub enum Mode {
    Input = 0b00,
    Output = 0b01,
    Alternate = 0b10
}

pub enum PullMode {
    Off = 0,
    Up = 1,
    Down = 2
}

impl Gpio {
    #[cold]
    pub const unsafe fn new(addr: usize) -> Gpio {
        Gpio(addr as *mut GpioReg)
    }

    #[cold]
    pub const fn pin(&self, pin: u8) -> Pin {
        if pin >= 16 {
            panic!();
        }
        Pin {
            gpio: self.0,
            pin
        }
    }
}

impl Pin {
    #[cold]
    pub fn mode(&mut self, mode: Mode) -> &mut Self {
        let mode = mode as usize;
        block_irq(|| {
            unsafe {
                let mut moder = volatile_read(&(*self.gpio).MODER);
                moder &= !(0b11 << self.pin * 2);
                moder |= mode << self.pin * 2;
                volatile_write(&mut (*self.gpio).MODER, moder);
            }
        });
        self
    }

    #[cold]
    pub fn alternative(&mut self, af: u8) -> &mut Self {
        let af = af as usize;
        let block = unsafe { if self.pin / 8 == 0 { &mut (*self.gpio).AFRL } else { &mut (*self.gpio).AFRH } };
        let val = *block & !(0b1111 << (self.pin % 8) * 4);
        *block = val | (af << (self.pin % 8) * 4);
        self
    }

    #[cold]
    pub fn pull(&mut self, pm: PullMode) -> &mut Self {
        unsafe {
            let val = volatile_read(&(*self.gpio).PUPDR) & !(0b11 << self.pin * 2);
            volatile_write(&mut (*self.gpio).PUPDR, val | (pm as usize) << self.pin * 2);
        }
        self
    }

    pub fn write(&mut self, state: bool) {
        let bsrr = unsafe { &mut (*self.gpio).BSRR };
        let pos = self.pin + if state { 0 } else { 16 };
        *bsrr = 1 << pos;
    }

    pub fn read(&self) -> bool {
        let idr = unsafe { volatile_read(&(*self.gpio).IDR ) };
        (idr >> self.pin) & 1 != 0
    }
}
