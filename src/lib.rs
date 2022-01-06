#![no_std]
#![no_main]

mod volatile;
pub mod rcc;
pub mod mutex;
pub mod interrupt;

#[cfg(feature = "default_panic")]
#[panic_handler]
fn __panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

static RCC: mutex::Mutex<rcc::Rcc> = unsafe { mutex::Mutex::new(rcc::Rcc::new(0x4002_3800)) };
