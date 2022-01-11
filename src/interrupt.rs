use crate::mutex::Mutex;
use crate::abstr::timer::Timer;
use crate::{TIM3, TIM4, TIM6, TIM7, USART1, USART3};

type Handler = &'static mut dyn FnMut();

static NMI: Mutex<Option<Handler>> = Mutex::new(None);
static HARDFAULT: Mutex<Option<Handler>> = Mutex::new(None);

pub fn on_nmi(h: Handler) {
    let mut nmi = NMI.lock();
    *nmi = Some(h);
}

pub fn on_hardfault(h: Handler) {
    let mut hf = HARDFAULT.lock();
    *hf = Some(h);
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
