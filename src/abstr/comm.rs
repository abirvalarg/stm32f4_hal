pub trait Communication<M> {
    fn send(&mut self, data: M);
    fn recv(&mut self) -> M;
}
