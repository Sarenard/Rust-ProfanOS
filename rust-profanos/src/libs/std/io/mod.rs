pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Option<usize>;
}