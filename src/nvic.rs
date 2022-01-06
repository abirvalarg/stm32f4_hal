use crate::volatile::*;

pub fn enable_irq(pos: usize) {
    let block = pos / 32;
    let pos = pos % 32;
    unsafe {
        volatile_write((0xe000_e100 + 4 * block) as *mut _, 1 << pos);
    }
}

