#![no_std]
#![no_main]

mod volatile;
pub mod rcc;
pub mod mutex;
pub mod interrupt;

static RCC: mutex::Mutex<rcc::Rcc> = unsafe { mutex::Mutex::new(rcc::Rcc::new(0x4002_3800)) };
