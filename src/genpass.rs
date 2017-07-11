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
        let mut chars = Vec::new();

        if config.enable_special {
            for c in constants::SPECIAL_CHARS.into_iter() {
                chars.push(c.clone());
            }
        }

        if config.enable_upper {
            for c in 65u8..90u8 + 1 {
                chars.push(c as char);
            }
        }

        if config.enable_lower {
            for c in 97u8..122u8 + 1 {
                chars.push(c as char);
            }
        }

        if config.enable_digit {
            for _ in 0..2 {
                for c in 48u8..57u8 + 1 {
                    chars.push(c as char);
                }
            }
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
