use rand::distributions::{IndependentSample, Range};
use rand;

use parseargs::ConfigArgs;
use constants;

pub struct Alphabet {
    chars: Vec<char>,
    range: Range<usize>,
    rng: rand::ThreadRng,
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
            chars.append(&mut constants::TWICE_DIGITS.to_vec());
        }

        let size = chars.len();

        Alphabet {
            chars: chars,
            range: Range::new(0, size),
            rng: rand::thread_rng(),
        }
    }

    pub fn get_char(&mut self) -> char {
        self.chars[self.range.ind_sample(&mut self.rng)]
    }
}
