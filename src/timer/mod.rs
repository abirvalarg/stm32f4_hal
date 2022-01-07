mod basic;
mod gp34;
pub use basic::Basic;
pub use gp34::GP34;
use crate::volatile::*;

#[repr(C)]
#[allow(non_snake_case)]
struct Reg {
    CR1: usize,
    CR2: usize,
    SMCR: usize,
    DIER: usize,
    SR: usize,
    EGR: usize,
    CCMR1: usize,
    CCMR2: usize,
    CCER: usize,
    CNT: usize,
    PSC: usize,
    ARR: usize,
    _res0: usize,
    CCR1: usize,
    CCR2: usize,
    CCR3: usize,
    CCR4: usize,
    _res1: usize,
    DCR: usize,
    DMAR: usize,
    OR: usize
}

pub struct PwmChannel {
    cr: *mut usize,
    cr_high: bool,
    counter: *mut usize
}

impl PwmChannel {
    pub fn start(&mut self) -> &mut Self {
        unsafe {
            let mut cr = volatile_read(self.cr);
            cr |= (0b110 << 4) << if self.cr_high { 8 } else { 0 };
            volatile_write(self.cr, cr);
        }
        self
    }

    pub fn value(&mut self, value: u32) -> &mut Self {
        unsafe {
            volatile_write(self.counter, value as usize);
        }
        self
    }
}

macro_rules! impl_timer {
    ($name:ident, $rel_hnd:ident) => {
        use crate::abstr::timer::Timer;
        use crate::volatile::*;
        impl Timer for $name {
            #[cold]
            fn one_pulse(&mut self, state: bool) -> &mut Self{
                unsafe {
                    let cr = volatile_read(&(*self.hw).CR1);
                    volatile_write(&mut (*self.hw).CR1, 
                        if state { cr | (1 << 3) }
                        else { cr & !(1 << 3) });
                }
                self
            }

            fn interrupt(&mut self, state: bool) -> &mut Self {
                unsafe {
                    let cr = volatile_read(&(*self.hw).DIER);
                    volatile_write(&mut (*self.hw).DIER, 
                        if state { cr | 1 }
                        else { cr & !1 });
                }
                self
            }

            fn reload(&mut self) -> &mut Self{
                if let Some(h) = &mut self.$rel_hnd {
                    h();
                }
                self
            }

            fn start(&mut self) -> &mut Self {
                unsafe {
                    let cr = volatile_read(&(*self.hw).CR1);
                    volatile_write(&mut (*self.hw).CR1, cr | 1);
                }
                self
            }

            fn stop(&mut self) -> &mut Self {
                unsafe {
                    let cr = volatile_read(&(*self.hw).CR1);
                    volatile_write(&mut (*self.hw).CR1, cr & !1);
                }
                self
            }

            fn set_prescaller(&mut self, psc: usize) -> &mut Self{
                unsafe {
                    let dier = volatile_read(&(*self.hw).DIER);
                    volatile_write(&mut (*self.hw).DIER, 0);
                    volatile_write(&mut (*self.hw).PSC, psc & 0xffff);
                    self.trigger();
                    while !self.clear_flag() {}
                    volatile_write(&mut (*self.hw).DIER, dier);
                }
                self
            }

            fn set_reload(&mut self, arr: usize) -> &mut Self {
                unsafe {
                    volatile_write(&mut (*self.hw).ARR, arr & 0xffff);
                }
                self
            }

            fn get_count(&self) -> usize {
                unsafe {
                    volatile_read(&(*self.hw).CNT)
                }
            }

            fn on_reload<F: FnMut() + 'static>(&mut self, handler: F) -> &mut Self {
                self.$rel_hnd = Some(Box::new(handler));
                self
            }

            fn trigger(&mut self) -> &mut Self {
                unsafe {
                    volatile_write(&mut (*self.hw).EGR, 1);
                }
                self
            }

            fn clear_flag(&mut self) -> bool {
                let sr = unsafe { volatile_read(&(*self.hw).SR) };
                if sr == 1 {
                    unsafe {
                        volatile_write(&mut (*self.hw).SR, 0);
                    }
                }
                sr == 1
            }
        }
    };
}
use impl_timer;
