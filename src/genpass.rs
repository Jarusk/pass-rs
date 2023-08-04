use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng};

use crate::constants;
use crate::parseargs::ConfigArgs;

pub struct Alphabet {
    chars: Vec<char>,
    range: Uniform<usize>,
    rng: StdRng,
}

impl Alphabet {
    pub fn new(config: &ConfigArgs) -> Alphabet {
        // Enough space to hold all the alphanumeric characters (upper and lower case)
        // as well as the 14 special characters defined in constants.rs
        let mut chars: Vec<char> = Vec::with_capacity(76);

        if config.enable_special {
            chars.append(&mut constants::SPECIAL_CHARS.to_vec());
        }

        if config.enable_upper {
            for i in 'A' as u8..'Z' as u8 + 1 {
                chars.push(i as char);
            }
        }

        if config.enable_lower {
            for i in 'a' as u8..'z' as u8 + 1 {
                chars.push(i as char);
            }
        }

        if config.enable_digit {
            for i in '0' as u8..'9' as u8 + 1 {
                chars.push(i as char);
                chars.push(i as char);
            }
        }

        let alphabet_length = chars.len();
        let mut seed = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut seed);

        Alphabet {
            chars,
            range: Uniform::new(0, alphabet_length),
            rng: StdRng::from_seed(seed),
        }
    }

    pub fn get_char(&mut self) -> char {
        self.chars[self.rng.sample(self.range)]
    }
}
