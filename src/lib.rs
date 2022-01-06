#![no_std]
#![no_main]

mod volatile;
pub mod rcc;
pub mod mutex;
pub mod interrupt;
pub mod abstr;
pub mod gpio;
pub mod timer;

#[cfg(feature = "default_panic")]
#[panic_handler]
fn __panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

use mutex::Mutex;
use gpio::Gpio;

pub static RCC: mutex::Mutex<rcc::Rcc> = unsafe { mutex::Mutex::new(rcc::Rcc::new(0x4002_3800)) };

pub static GPIOA: Mutex<Gpio> = unsafe { Mutex::new(Gpio::new(0x4002_0000)) };
pub static GPIOB: Mutex<Gpio> = unsafe { Mutex::new(Gpio::new(0x4002_0400)) };
pub static GPIOC: Mutex<Gpio> = unsafe { Mutex::new(Gpio::new(0x4002_0800)) };
