mod exti;

pub use exti::Edge;

use crate::Box;
use crate::mutex::Mutex;
use crate::abstr::timer::Timer;
use crate::{TIM3, TIM4, TIM6, TIM7, USART1, USART3, SYSCFG};

type Handler = Box<dyn FnMut()>;

static EXTI: crate::Mutex<exti::Exti> = unsafe { crate::Mutex::new(exti::Exti::new(0x4001_4000)) };

const NO_HANDLER: Option<Handler> = None;

static NMI: Mutex<Option<Handler>> = Mutex::new(NO_HANDLER);
static HARDFAULT: Mutex<Option<Handler>> = Mutex::new(NO_HANDLER);
static EXTI_INT: Mutex<[Option<Handler>; 16]> = Mutex::new([NO_HANDLER; 16]);

pub fn on_nmi(h: Handler) {
    let mut nmi = NMI.lock();
    *nmi = Some(h);
}

pub fn on_hardfault<F: FnMut() + 'static>(h: F) {
    let mut hf = HARDFAULT.lock();
    *hf = Some(Box::new(h));
}

pub fn on_exti<F: FnMut() + 'static>(ctl: u8, channel: u8, mode: Edge, hnd: F) {
    EXTI_INT.lock()[channel as usize] = Some(Box::new(hnd));
    SYSCFG.lock().set_channel(ctl, channel);
    EXTI.lock().set_channel(channel, mode);
}

#[no_mangle]
extern "C" fn _nmi() {
    match &mut *NMI.lock() {
        Some(handler) => handler(),
        None => panic!()
    }
}

#[no_mangle]
extern "C" fn _hardfault() {
    match &mut *HARDFAULT.lock() {
        Some(handler) => handler(),
        None => panic!()
    }
}

#[no_mangle]
extern "C" fn _exti() {
    let pending = EXTI.lock().get_pending();
    let mut handlers = EXTI_INT.lock();
    for pos in 0..16 {
        if pending & (1 << pos) != 0 {
            if let Some(h) = &mut handlers[pos] {
                h();
            }
        }
    }
}

#[no_mangle]
extern "C" fn _tim3() {
    let mut tim = TIM3.lock();
    if tim.clear_flag() {
        tim.reload();
    }
}

#[no_mangle]
extern "C" fn _tim4() {
    let mut tim = TIM4.lock();
    if tim.clear_flag() {
        tim.reload();
    }
}

#[no_mangle]
extern "C" fn _usart1() {
    USART1.lock().check_state();
}

#[no_mangle]
extern "C" fn _usart3() {
    USART3.lock().check_state();
}

#[no_mangle]
extern "C" fn _tim6_dac() {
    let mut tim6 = TIM6.lock();
    if tim6.clear_flag() {
        tim6.reload();
    }
}

#[no_mangle]
extern "C" fn _tim7() {
    let mut tim7 = TIM7.lock();
    if tim7.clear_flag() {
        tim7.reload();
    }
}
