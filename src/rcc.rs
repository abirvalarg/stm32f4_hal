use crate::volatile::*;
use crate::mutex::block_irq;

#[repr(C)]
#[allow(non_snake_case)]
struct RccReg {
    CR: usize,
    PLLCFGR: usize,
    CFGR: usize,
    CIR: usize,
    AHB1RSTR: usize,
    AHB2RSTR: usize,
    AHB3RSTR: usize,
    _res0: usize,
    APB1RSTR: usize,
    APB2RSTR: usize,
    _res1: usize,
    _res2: usize,
    AHB1ENR: usize,
    AHB2ENR: usize,
    AHB3ENR: usize,
    _res3: usize,
    APB1ENR: usize,
    APB2ENR: usize,
    _res4: usize,
    _res5: usize
}

pub struct Rcc(*mut RccReg);

impl Rcc {
    pub const unsafe fn new(addr: usize) -> Rcc {
        Rcc(addr as *mut RccReg)
    }

    pub fn ahb1_enable(&mut self, pos: Ahb1Module) -> &mut Self {
        let mut enr = unsafe { volatile_read(&(*self.0).AHB1ENR) };
        enr |= 1 << (pos as usize);
        unsafe {
            volatile_write(&mut (*self.0).AHB1ENR, enr);
        }
        self
    }

    pub fn apb1_enable(&mut self, pos: Apb1Module) -> &mut Self {
        unsafe {
            let enr = volatile_read(&(*self.0).APB1ENR);
            volatile_write(&mut (*self.0).APB1ENR, enr | (1 << pos as usize));
        }
        self
    }

    pub fn apb2_enable(&mut self, pos: Apb2Module) -> &mut Self {
        unsafe {
            let enr = volatile_read(&(*self.0).APB2ENR);
            volatile_write(&mut (*self.0).APB2ENR, enr | (1 << pos as usize));
        }
        self
    }

    pub fn set_pll(&mut self, m: usize, n: usize, p: usize, q: usize, src: PLLSource) -> &mut Self {
        block_irq(|| {
            unsafe {
                volatile_write(&mut (*self.0).PLLCFGR,
                    (m & 0b111111) |
                    ((n & ((1 << 10) - 1)) << 6) |
                    ((p & 0b11) << 16) |
                    (q & 0b1111) << 24 |
                    (src as usize & 1) << 22);
            }
        });
        self
    }

    pub fn set_prescallers(&mut self, ahb: usize, apb1: usize, apb2: usize) -> &mut Self {
        block_irq(|| {
            unsafe {
                volatile_write(&mut (*self.0).CFGR,
                    ((ahb & 0b1111) << 4) |
                    ((apb1 & 0b111) << 10) |
                    ((apb2 & 0b111) << 13)
                );
            }
        });
        self
    }

    pub fn enable_hse(&mut self) -> &mut Self {
        unsafe {
            let cir = volatile_read(&mut (*self.0).CIR);
            volatile_write(&mut (*self.0).CIR, cir | (1 << 11));
            let cr = volatile_read(&(*self.0).CR);
            volatile_write(&mut (*self.0).CR, cr | (1 << 16));
            while volatile_read(&(*self.0).CIR) & (1 << 3) == 0 {}
            volatile_write(&mut (*self.0).CIR, 1 << 19);
        }
        self
    }

    pub fn enable_pll(&mut self) -> &mut Self {
        unsafe {
            let cir = volatile_read(&mut (*self.0).CIR);
            volatile_write(&mut (*self.0).CIR, cir | (1 << 12));
            let cr = volatile_read(&(*self.0).CR);
            volatile_write(&mut (*self.0).CR, cr | (1 << 24));
            while volatile_read(&(*self.0).CIR) & (1 << 4) == 0 {}
            volatile_write(&mut (*self.0).CIR, 1 << 20);
        }
        self
    }

    pub fn set_clock_source(&mut self, clock_source: ClockSource) -> &mut Self {
        unsafe {
            let cfgr = volatile_read(&(*self.0).CFGR) & !(0b11);
            volatile_write(&mut (*self.0).CFGR, cfgr | (clock_source as usize));
        }
        self
    }
}

#[non_exhaustive]
pub enum Ahb1Module {
    GPIOA = 0,
    GPIOB = 1,
    GPIOC = 2,
    DMA1 = 21,
    DMA2 = 22,
}

#[non_exhaustive]
pub enum Apb1Module {
    TIM3 = 1,
    TIM4 = 2,
    TIM6 = 4,
    TIM7 = 5,
    USART3 = 18
}

#[non_exhaustive]
pub enum Apb2Module {
    TIM1 = 0,
    USART1 = 4,
    ADC1 = 8,
    ADC2 = 9,
    ADC3 = 10,
    SPI1 = 12,
    SYSCFG = 14
}

#[derive(Copy, Clone)]
pub enum PLLSource {
    HSI = 0,
    HSE = 1
}

#[derive(Copy, Clone)]
pub enum ClockSource {
    HSI = 0b00,
    HSE = 0b01,
    PLL = 0b10
}
