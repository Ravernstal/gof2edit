pub trait Index {
    fn index(&self) -> u32;
    fn set_index(&mut self, index: u32);
}
