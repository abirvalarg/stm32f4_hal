extern "C" {
    pub fn malloc(size: usize) -> *mut ();
    pub fn free(mem: *mut ());
}
