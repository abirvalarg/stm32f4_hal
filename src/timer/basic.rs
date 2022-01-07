use crate::Box;
use super::Reg;

pub struct Basic {
    hw: *mut Reg,
    handler: Option<Box<dyn FnMut()>>
}

impl Basic {
    pub const unsafe fn new(hw: usize) -> Basic {
        Basic {
            hw: hw as *mut Reg,
            handler: None
        }
    }
}

super::impl_timer!(Basic, handler);
