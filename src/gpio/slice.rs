use super::Pin;

pub struct Slice<const SIZE: usize>([Pin; SIZE]);

impl<const SIZE: usize> From<[Pin; SIZE]> for Slice<SIZE> {
    fn from(s: [Pin; SIZE]) -> Self {
        Slice(s)
    }
}

impl<const SIZE: usize> Slice<SIZE> {
    pub const fn new(s: [Pin; SIZE]) -> Self {
        Slice(s)
    }

    pub fn write(&mut self, data: usize) {
        for i in 0..SIZE {
            self.0[i].write(data & (1 << i) != 0);
        }
    }

    pub fn read(&mut self) -> usize {
        self.0.iter().enumerate().fold(0, |acc, (idx, pin)| acc | ((pin.read() as usize) << idx))
    }
}
