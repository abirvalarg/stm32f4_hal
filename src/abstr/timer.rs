pub trait Timer {
    fn one_pulse(&mut self, state: bool);
    fn interrupt(&mut self, state: bool);
    fn reload(&mut self);
    fn start(&mut self);
    fn stop(&mut self);
    fn trigger(&mut self);
    fn set_prescaller(&mut self, pcs: usize);
    fn set_reload(&mut self, arr: usize);
    fn get_count(&self) -> usize;
    fn clear_flag(&mut self) -> bool;
}
