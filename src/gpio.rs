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
    Output = 0b01
}

impl Gpio {
    pub const unsafe fn new(addr: usize) -> Gpio {
        Gpio(addr as *mut GpioReg)
    }

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

    pub fn write(&mut self, state: bool) {
        block_irq(|| {
            unsafe {
                let odr = volatile_read(&(*self.gpio).ODR);
                volatile_write(
                    &mut (*self.gpio).ODR,
                    if state { odr | 1 << self.pin}
                        else { odr & !(1 << self.pin)});
            }
        })
    }

    pub fn read(&self) -> bool {
        let idr = unsafe { volatile_read(&(*self.gpio).IDR ) };
        (idr >> self.pin) & 1 != 0
    }
}
