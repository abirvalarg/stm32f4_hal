#![no_std]
#![no_main]
#![feature(coerce_unsized)]
#![feature(unsize)]
#![feature(const_fn_trait_bound)]

mod volatile;
mod malloc;
pub mod syscfg;
pub mod rcc;
pub mod nvic;
pub mod mutex;
pub mod ptr;
pub mod interrupt;
pub mod abstr;
pub mod gpio;
pub mod timer;
pub mod spi;
pub mod analog;
pub mod usart;
pub mod dma;
pub mod flash;

pub mod lcd1602;

#[cfg(feature = "default_panic")]
#[panic_handler]
fn __panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub use mutex::Mutex;
pub use ptr::Box;
use gpio::Gpio;
use spi::Spi;
use dma::DMA;

pub static RCC: mutex::Mutex<rcc::Rcc> = unsafe { mutex::Mutex::new(rcc::Rcc::new(0x4002_3800)) };

pub static GPIOA: Mutex<Gpio> = unsafe { Mutex::new(Gpio::new(0x4002_0000)) };
pub static GPIOB: Mutex<Gpio> = unsafe { Mutex::new(Gpio::new(0x4002_0400)) };
pub static GPIOC: Mutex<Gpio> = unsafe { Mutex::new(Gpio::new(0x4002_0800)) };
pub static DMA1: Mutex<DMA> = unsafe { Mutex::new(DMA::new(0x4002_6000)) };
pub static DMA2: Mutex<DMA> = unsafe { Mutex::new(DMA::new(0x4002_6400)) };


pub static TIM3: Mutex<timer::GP34> = unsafe { Mutex::new(timer::GP34::new(0x4000_0400)) };
pub static TIM4: Mutex<timer::GP34> = unsafe { Mutex::new(timer::GP34::new(0x4000_0800)) };
pub static TIM6: Mutex<timer::Basic> = unsafe { Mutex::new(timer::Basic::new(0x4000_1000)) };
pub static TIM7: Mutex<timer::Basic> = unsafe { Mutex::new(timer::Basic::new(0x4000_1400)) };
pub static USART3: Mutex<usart::Usart> = unsafe { Mutex::new(usart::Usart::new(0x4000_4800)) };

// APB2
pub static TIM1: Mutex<timer::GP34> = unsafe { Mutex::new(timer::GP34::new(0x4001_0000)) };
pub static USART1: Mutex<usart::Usart> = unsafe { Mutex::new(usart::Usart::new(0x4001_1000)) };
pub static ADC: Mutex<analog::ADC> = unsafe { Mutex::new(analog::ADC::new(0x4001_2000)) };
pub static SYSCFG: Mutex<syscfg::Syscfg> = unsafe { Mutex::new(syscfg::Syscfg::new(0x4001_3800)) };

pub static SPI1: Mutex<Spi> = unsafe { Mutex::new(Spi::new(
    0x4001_3000,
    GPIOA.as_const().pin(7),
    GPIOA.as_const().pin(6),
    GPIOA.as_const().pin(5),
    GPIOA.as_const().pin(4),
    5)) };
