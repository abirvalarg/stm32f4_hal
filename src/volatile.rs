extern "C" {
    pub fn volatile_read(addr: *const usize) -> usize;
    pub fn volatile_write(addr: *mut usize, value: usize);
}
