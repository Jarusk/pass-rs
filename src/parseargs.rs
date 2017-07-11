use std::env;
use constants;


struct ConfigArgs {
    passLength: usize,
    printHelp: bool,
    enableDigit: bool,
    enableUpper: bool,
    enableLower: bool,
    enableSpecial: bool
}

impl ConfigArgs {
    pub fn new() -> ConfigArgs {
        ConfigArgs {
            passLength: constants::DEFAULT_PASS_LEN,
            printHelp: false,
            enableLower: true,
            enableUpper: true,
            enableDigit: true,
            enableSpecial: true
        }
    }

    pub fn readArgs(&mut self) {
        unimplemented!();
    }
}
