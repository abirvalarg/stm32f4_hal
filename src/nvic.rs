use crate::volatile::*;

#[cold]
pub fn enable_irq(pos: Irq) {
    let pos = pos as usize;
    let block = pos / 32;
    let pos = pos % 32;
    unsafe {
        volatile_write((0xe000_e100 + 4 * block) as *mut _, 1 << pos);
    }
}


#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Irq {
    TIM6_DAC = 54,
    TIM7 = 55
}
