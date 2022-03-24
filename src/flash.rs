use crate::volatile::*;

#[repr(C)]
#[allow(non_snake_case)]
struct Reg {
    ACR: usize,
    KEYR: usize,
    OPTKEYR: usize,
    SR: usize,
    CR: usize,
    OPTCR: usize
}

pub struct Flash(*mut Reg);

impl Flash {
    pub const unsafe fn new(addr: usize) -> Flash {
        Flash(addr as *mut Reg)
    }

    pub fn set_latency(&mut self, latency: usize) {
        unsafe {
            let val = volatile_read(&(*self.0).ACR) & !(0b111);
            volatile_write(&mut (*self.0).KEYR, val | (latency & 0b111));
        }
    }
}
