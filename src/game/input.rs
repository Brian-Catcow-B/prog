pub struct Input {
    pub esc: bool,
    pub num_key_bitwise: u16,
    pub run_prog: bool,
    pub stop_prog: bool,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            esc: false,
            num_key_bitwise: 0,
            run_prog: false,
            stop_prog: false,
        }
    }
}

impl Input {
    pub fn reset(&mut self) {
        self.esc = false;
        self.num_key_bitwise = 0;
        self.run_prog = false;
        self.stop_prog = false;
    }
}
