pub trait Timer {
    fn start(&mut self);
    fn stop(&mut self);
    fn trigger(&mut self);
    fn set_prescaller(&mut self, pcs: usize);
    fn set_reload(&mut self, arr: usize);
    fn get_count(&self) -> usize;
    fn on_reload<F: Fn() + 'static>(&mut self, handler: F);
    fn clear_flag(&mut self) -> bool;
}
