pub trait Timer {
    fn one_pulse(&mut self, state: bool) -> &mut Self;
    fn interrupt(&mut self, state: bool) -> &mut Self;
    fn reload(&mut self) -> &mut Self;
    fn start(&mut self) -> &mut Self;
    fn stop(&mut self) -> &mut Self;
    fn trigger(&mut self) -> &mut Self;
    fn set_prescaller(&mut self, pcs: usize) -> &mut Self;
    fn set_reload(&mut self, arr: usize) -> &mut Self;
    fn get_count(&self) -> usize;
    fn on_reload<F: FnMut() + 'static>(&mut self, handler: F) -> &mut Self;
    fn clear_flag(&mut self) -> bool;
}
