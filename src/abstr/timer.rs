pub trait Timer<'a> {
    fn start(&mut self);
    fn stop(&mut self);
    fn set_prescaller(&mut self, pcs: usize);
    fn set_reload(&mut self, arr: usize);
    fn get_count(&self) -> usize;
    fn on_reload(&mut self, handler: &'a mut dyn FnMut());
}
