use crate::mutex::Mutex;

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
fn _nmi() {
    match &mut *NMI.lock() {
        Some(handler) => handler(),
        None => panic!()
    }
}

#[no_mangle]
fn _hardfault() {
    match &mut *HARDFAULT.lock() {
        Some(handler) => handler(),
        None => panic!()
    }
}
