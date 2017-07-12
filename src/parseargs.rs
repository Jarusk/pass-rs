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
            print_help: constants::DEFAULT_PRINT_HELP,
            enable_lower: constants::DEFAULT_ENABLE_LOWER,
            enable_upper: constants::DEFAULT_ENABLE_UPPER,
            enable_digit: constants::DEFAULT_ENABLE_DIGIT,
            enable_special: constants::DEFAULT_ENABLE_SPECIAL
        }
    }

    pub fn read_args(&mut self) {
        for _ in env::args() {
            //println!("{:?}", a);
        }
    }
}
