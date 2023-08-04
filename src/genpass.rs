use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng};

use crate::constants;
use crate::parseargs::ConfigArgs;

#[cfg(test)]
const MAX_ALPHABET_LENGTH: usize = 86;

pub struct Alphabet {
    chars: Vec<char>,
    range: Uniform<usize>,
    rng: StdRng,
}

impl Alphabet {
    pub fn new(config: &ConfigArgs) -> Alphabet {
        // Enough space to hold all the alphanumeric characters (upper and lower case)
        // as well as the 14 special characters defined in constants.rs
        let mut chars: Vec<char> = Vec::with_capacity(86);

        if config.enable_special {
            chars.append(&mut constants::SPECIAL_CHARS.to_vec());
        }

        if config.enable_upper {
            for i in b'A'..b'Z' + 1 {
                chars.push(i as char);
            }
        }

        if config.enable_lower {
            for i in b'a'..b'z' + 1 {
                chars.push(i as char);
            }
        }

        if config.enable_digit {
            for i in b'0'..b'9' + 1 {
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

    #[cfg(test)]
    pub fn set_seed(&mut self, seed: [u8; 32]) {
        self.rng = StdRng::from_seed(seed);
    }
}

#[cfg(test)]
mod tests {
    use crate::{constants, genpass::MAX_ALPHABET_LENGTH, parseargs::ConfigArgs};

    use super::*;

    #[test]
    fn it_makes_digit_only_alphabet() {
        let mut cfg = ConfigArgs::new();
        cfg.enable_digit = true;
        cfg.enable_lower = false;
        cfg.enable_upper = false;
        cfg.enable_special = false;

        let alpha = Alphabet::new(&cfg);

        for elem in alpha.chars.iter() {
            assert_eq!(elem.is_ascii_digit(), true);
        }
    }

    #[test]
    fn it_makes_lowercase_only_alphabet() {
        let mut cfg = ConfigArgs::new();
        cfg.enable_digit = false;
        cfg.enable_lower = true;
        cfg.enable_upper = false;
        cfg.enable_special = false;

        let alpha = Alphabet::new(&cfg);

        for elem in alpha.chars.iter() {
            assert_eq!(elem.is_ascii_lowercase(), true);
        }
    }

    #[test]
    fn it_makes_uppercase_only_alphabet() {
        let mut cfg = ConfigArgs::new();
        cfg.enable_digit = false;
        cfg.enable_lower = false;
        cfg.enable_upper = true;
        cfg.enable_special = false;

        let alpha = Alphabet::new(&cfg);

        for elem in alpha.chars.iter() {
            assert_eq!(elem.is_ascii_uppercase(), true);
        }
    }

    #[test]
    fn it_makes_special_only_alphabet() {
        let mut cfg = ConfigArgs::new();
        cfg.enable_digit = false;
        cfg.enable_lower = false;
        cfg.enable_upper = false;
        cfg.enable_special = true;

        let alpha = Alphabet::new(&cfg);

        for elem in alpha.chars.iter() {
            assert_eq!(constants::SPECIAL_CHARS.contains(elem), true);
        }
    }

    #[test]
    fn it_makes_alphabet_with_all() {
        let mut cfg = ConfigArgs::new();
        cfg.enable_digit = true;
        cfg.enable_lower = true;
        cfg.enable_upper = true;
        cfg.enable_special = true;

        let alpha = Alphabet::new(&cfg);

        assert_eq!(alpha.chars.len(), MAX_ALPHABET_LENGTH);
    }

    #[test]
    fn it_should_produce_known_chars() {
        let cfg = ConfigArgs::new();
        let mut alpha = Alphabet::new(&cfg);

        let seed = [0; 32];
        alpha.set_seed(seed);

        let expected = ['X', '4', 'F', 'a', '1', 'i', 'H', '0', '9', 'R'];

        let mut result = Vec::new();

        for _ in 0..10 {
            result.push(alpha.get_char());
        }

        assert_eq!(result, expected);
    }
}
