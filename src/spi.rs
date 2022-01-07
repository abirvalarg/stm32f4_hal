use crate::abstr::comm::Communication;
use crate::gpio::{self, Pin};
use crate::volatile::*;

#[repr(C)]
#[allow(non_snake_case)]
struct Reg {
    CR1: usize,
    CR2: usize,
    SR: usize,
    DR: usize,
    CRCPR: usize,
    RXCRCR: usize,
    TXCRCR: usize,
    I2SCFGR: usize,
    I2SPR: usize
}

pub struct Spi {
    hw: *mut Reg,
    mosi: Pin,
    miso: Pin,
    sck: Pin,
    nss: Pin,
    af_num: u8,
}

pub struct SpiDev {
    spi: *mut Reg,
    nss: Option<Pin>,
    nss_mode: NssMode
}

pub enum NssMode {
    Hardware,
    Default,
    Reverse
}

pub enum ClockMode {
    Default = 0b00,
    ReversePolarity = 0b01,
    ReversePhase = 0b10,
    Reverse = 0b11,
}

pub enum Mode {
    Master {
        baudrate: u8,
    },
    Slave
}

impl Spi {
    pub const unsafe fn new(addr: usize, mosi: Pin, miso: Pin, sck: Pin, nss: Pin, af_num: u8) -> Spi {
        Spi {
            hw: addr as *mut Reg,
            mosi,
            miso,
            sck,
            nss,
            af_num,
        }
    }

    pub fn init(&mut self, hardware_nss: bool, mode: Mode, clk_mode: ClockMode, bits16: bool) -> &mut Self {
        self.mosi.mode(gpio::Mode::Alternate)
            .alternative(self.af_num);
        self.miso.mode(gpio::Mode::Alternate)
            .alternative(self.af_num);
        self.sck.mode(gpio::Mode::Alternate)
            .alternative(self.af_num);
        if hardware_nss {
            self.nss.mode(gpio::Mode::Alternate)
                .alternative(self.af_num);
        }

        let mut cr1 = unsafe { volatile_read(&(*self.hw).CR1) & !(0b111 << 3) };
        let mut cr2 = unsafe { volatile_read(&(*self.hw).CR2) };
        cr1 &= !0b11;
        cr1 |= clk_mode as usize;

        if bits16 {
            cr1 |= 1 << 11;
        } else {
            cr1 &= !(1 << 11);
        }

        match mode {
            Mode::Master { baudrate } => {
                cr1 &= !(0b111 << 3);
                cr1 |= (baudrate as usize & 0b111) << 3 | 1 << 2 | 1 << 8;

                cr2 |= 1 << 2;
            },
            Mode::Slave => { todo!(); }
        }

        unsafe {
            volatile_write(&mut (*self.hw).CR1, cr1);
            volatile_write(&mut (*self.hw).CR2, cr2);
        }

        self
    }

    pub const fn device(&self, nss: Option<Pin>, nss_mode: NssMode) -> SpiDev {
        SpiDev {
            spi: self.hw,
            nss,
            nss_mode
        }
    }
}

impl SpiDev {
    pub fn start(&mut self) {
        match self.nss_mode {
            NssMode::Hardware => {
                unsafe {
                    let cr1 = volatile_read(&(*self.spi).CR1);
                    volatile_write(&mut (*self.spi).CR1, cr1 | (1 << 6));
                }
            },
            NssMode::Default => self.nss.as_mut().unwrap().write(false),
            NssMode::Reverse => self.nss.as_mut().unwrap().write(true)
        }
    }

    pub fn stop(&mut self) {
        unsafe {
            while (*self.spi).SR & (1 << 7) != 0 {}
        }

        match self.nss_mode {
            NssMode::Hardware => {
                unsafe {
                    let cr1 = volatile_read(&(*self.spi).CR1);
                    volatile_write(&mut (*self.spi).CR1, cr1 & !(1 << 6));
                }
            },
            NssMode::Default => self.nss.as_mut().unwrap().write(true),
            NssMode::Reverse => self.nss.as_mut().unwrap().write(false)
        }
    }
}

impl Communication<u16> for SpiDev {
    fn send(&mut self, data: u16) {
        unsafe {
            while (*self.spi).SR & 0b10 == 0 {}
            volatile_write(&mut (*self.spi).DR, data as usize);
        }

    }

    fn recv(&mut self) -> u16 {
        unsafe {
            while (*self.spi).SR & 1 == 0 {}
            volatile_read(&(*self.spi).DR) as u16
        }
    }
}
