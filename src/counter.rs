#[derive(Default, Debug)]
pub struct Counter {
    value: u64,
}

impl Counter {
    pub fn increment(&mut self) {
        self.value += 1;
    }
}
