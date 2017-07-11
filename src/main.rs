use std::env;

const DEFAULT_PASS_LEN: usize = 38;


struct ConfigArgs {
    passLength: usize,
    printHelp: bool,
    enableDigit: bool,
    enableUpper: bool,
    enableLower: bool
}

impl ConfigArgs {
    fn new() -> ConfigArgs {
        ConfigArgs {
            passLength: DEFAULT_PASS_LEN,
            printHelp: false,
            enableLower: true,
            enableUpper: true,
            enableDigit: true
        }
    }

    fn readArgs(&mut self) {
        unimplemented!();
    }
}

fn main() {
    // DUMMY code as placeholder
    println!("pass-rs");

    for a in env::args() {
        println!("{:?}", a);
    }
}
