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
pub struct DmaStream(*mut StreamReg);

impl DMA {
    pub const unsafe fn new(addr: usize) -> DMA {
        DMA(addr as *mut DmaReg)
    }

    pub fn stream(&self, stream: usize) -> DmaStream {
        unsafe {
            DmaStream(&mut (*self.0).S[stream])
        }
    }
}

enum Mode {
    PeripheralToMemory = 0b00,
    MemoryToPeripheral = 0b01,
    MemoryToMemory = 0b10
}
