use super::Reg;
use crate::Box;

pub struct GP34 {
    hw: *mut Reg,
    reload_handler: Option<Box<dyn FnMut()>>
}

impl GP34 {
    pub const unsafe fn new(addr: usize) -> GP34 {
        GP34 {
            hw: addr as *mut Reg,
            reload_handler: None
        }
    }

    pub fn pwm(&mut self, channel: i32) -> super::PwmChannel {
        unsafe {
            let cnt = match channel {
                1 => &mut (*self.hw).CCR1,
                2 => &mut (*self.hw).CCR2,
                3 => &mut (*self.hw).CCR3,
                4 => &mut (*self.hw).CCR4,
                _ => panic!()
            };
            let channel = channel - 1;
            let ccer = volatile_read(&(*self.hw).CCER);
            volatile_write(&mut (*self.hw).CCER, ccer | 1 << (4 * channel));
            let cr_addr = if channel & 0b10 == 0 { &mut (*self.hw).CCMR1 } else { &mut (*self.hw).CCMR2 };
            super::PwmChannel {
                cr: cr_addr,
                cr_high: channel & 1 == 1,
                counter: cnt
            }
        }
    }
}

super::impl_timer!(GP34, reload_handler);
