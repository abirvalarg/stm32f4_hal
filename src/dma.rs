#[repr(C)]
#[allow(non_snake_case, dead_code)]
struct DmaReg {
    LISR: usize,
    HISR: usize,
    LIFCR: usize,
    HIFCR: usize,
    S: [StreamReg; 8]
}

#[repr(C)]
#[allow(non_snake_case, dead_code)]
struct StreamReg {
    CR: usize,
    NDTR: usize,
    PAR: usize,
    M0AR: usize,
    M1AR: usize,
    FCR: usize
}

pub struct DMA(*mut DmaReg);

impl DMA {
    pub const unsafe fn new(addr: usize) -> DMA {
        DMA(addr as *mut DmaReg)
    }
}
