use rand::distributions::{IndependentSample, Range};
use rand::{self, Isaac64Rng, SeedableRng, Rng};

use parseargs::ConfigArgs;
use constants;

pub struct Alphabet {
    chars: Vec<char>,
    range: Range<usize>,
    rng: Isaac64Rng,
}

impl Alphabet {
    pub fn new(config: &ConfigArgs) -> Alphabet {
        let mut chars: Vec<char> = Vec::new();

        if config.enable_special {
            chars.append(&mut constants::SPECIAL_CHARS.to_vec());
        }

        if config.enable_upper {
            chars.append(&mut constants::UPPERCASE.to_vec());
        }

        if config.enable_lower {
            chars.append(&mut constants::LOWERCASE.to_vec());
        }

        if config.enable_digit {
            let mut digits = constants::DIGITS.to_vec();
            chars.append(&mut digits.clone());
            chars.append(&mut digits);
        }

        let size = chars.len();
        let mut seed = [0u64; 256];
        let mut tmp_rng = rand::thread_rng();

        for i in 0..256 {
            seed[i] = tmp_rng.next_u64();
        }

        Alphabet {
            chars: chars,
            range: Range::new(0, size),
            rng: Isaac64Rng::from_seed(&seed),
        }
    }

    pub fn get_char(&mut self) -> char {
        self.chars[self.range.ind_sample(&mut self.rng)]
    }
}
