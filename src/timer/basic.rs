use crate::Box;
use crate::volatile::*;
use crate::abstr::timer::Timer;

#[repr(C)]
#[allow(non_snake_case)]
struct Reg {
    CR1: usize,
    CR2: usize,
    _res0: usize,
    DIER: usize,
    SR: usize,
    EGR: usize,
    _res1: usize,
    _res2: usize,
    _res3: usize,
    CNT: usize,
    PSC: usize,
    ARR: usize
}

pub struct Basic {
    hw: *mut Reg,
    handler: Option<Box<dyn Fn()>>
}

impl Basic {
    pub const unsafe fn new(addr: usize) -> Basic {
        Basic {
            hw: addr as *mut Reg,
            handler: None
        }
    }

    pub fn one_pulse(&mut self, state: bool) {
        unsafe {
            let cr = volatile_read(&(*self.hw).CR1);
            volatile_write(&mut (*self.hw).CR1, 
                if state { cr | (1 << 3) }
                else { cr & !(1 << 3) });
        }
    }

    pub fn interrupt(&mut self, state: bool) {
        unsafe {
            let cr = volatile_read(&(*self.hw).DIER);
            volatile_write(&mut (*self.hw).DIER, 
                if state { cr | 1 }
                else { cr & !1 });
        }
    }

    pub fn reload(&mut self) {
        if let Some(h) = &self.handler {
            h();
        }
    }
}

impl Timer for Basic {
    fn start(&mut self) {
        unsafe {
            let cr = volatile_read(&(*self.hw).CR1);
            volatile_write(&mut (*self.hw).CR1, cr | 1);
        }
    }

    fn stop(&mut self) {
        unsafe {
            let cr = volatile_read(&(*self.hw).CR1);
            volatile_write(&mut (*self.hw).CR1, cr & !1);
        }
    }

    fn set_prescaller(&mut self, psc: usize) {
        unsafe {
            let dier = volatile_read(&(*self.hw).DIER);
            volatile_write(&mut (*self.hw).DIER, 0);
            volatile_write(&mut (*self.hw).PSC, psc & 0xffff);
            self.trigger();
            while !self.clear_flag() {}
            volatile_write(&mut (*self.hw).DIER, dier);
        }
    }

    fn set_reload(&mut self, arr: usize) {
        unsafe {
            volatile_write(&mut (*self.hw).ARR, arr & 0xffff);
        }
    }

    fn get_count(&self) -> usize {
        unsafe {
            volatile_read(&(*self.hw).CNT)
        }
    }

    fn on_reload<F: Fn() + 'static>(&mut self, handler: F) {
        self.handler = Some(Box::new(handler));
    }

    fn trigger(&mut self) {
        unsafe {
            volatile_write(&mut (*self.hw).EGR, 1);
        }
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
