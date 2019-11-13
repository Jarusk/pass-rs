use std::env;
use std::str::FromStr;

use crate::constants;

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
            enable_special: constants::DEFAULT_ENABLE_SPECIAL,
        }
    }

    pub fn read_args(&mut self) {
        for (count, arg) in env::args().enumerate() {
            match arg.as_ref() {
                "-nl" => self.enable_lower = false,
                "-nu" => self.enable_upper = false,
                "-nd" => self.enable_digit = false,
                "-s" => self.enable_special = true,
                "-h" | "--help" => self.print_help = true,
                _ => {
                    if count > 0 {
                        self.validate_possible_numeric(arg.as_ref())
                    }
                }
            }
        }

        if !self.enable_digit && !self.enable_lower && !self.enable_upper && !self.enable_special {
            println!("ERROR: can't disable entire alphabet!\n");
            self.print_help = true;
        }

        if self.pass_length < 1 {
            println!("ERROR: password length must be greater than 0\n");
            self.print_help = true;
        }
    }

    fn validate_possible_numeric(&mut self, arg: &str) {

        if let Ok(x) = usize::from_str(arg) {
            self.pass_length = x;
            return;
        }

        println!("ERROR: invalid argument \"{}\"\n", &arg);
        self.print_help = true;
    }
}
