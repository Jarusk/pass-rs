use std::env;
use constants;


pub struct ConfigArgs {
    pub pass_length: usize,
    pub print_help: bool,
    pub enable_digit: bool,
    pub enable_upper: bool,
    pub enable_lower: bool,
    pub enable_special: bool,
}

impl ConfigArgs {
    pub fn new() -> ConfigArgs {
        ConfigArgs {
            pass_length: constants::DEFAULT_PASS_LEN,
            print_help: false,
            enable_lower: true,
            enable_upper: true,
            enable_digit: true,
            enable_special: false,
        }
    }

    pub fn read_args(&mut self) {
        for a in env::args() {
            //println!("{:?}", a);
        }
    }
}
