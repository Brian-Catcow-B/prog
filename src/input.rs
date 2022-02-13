pub struct Input {
    pub num_key_bitwise: u16,
    pub run_prog: bool,
    pub stop_prog: bool,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            num_key_bitwise: 0,
            run_prog: false,
            stop_prog: false,
        }
    }
}
