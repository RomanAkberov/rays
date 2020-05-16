pub struct Random {
    state: u32,
}

impl Random {
    pub fn new(state: u32) -> Self {
        assert_ne!(0, state);
        Self { state }
    }

    pub fn next_u32(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }

    pub fn next_f64(&mut self) -> f64 {
        self.next_u32() as f64 / 4294967296.0
    }
}
