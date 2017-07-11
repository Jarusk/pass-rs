extern crate rand;

mod parseargs;
mod constants;
mod genpass;


fn main() {
    // Set default config
    let mut config = parseargs::ConfigArgs::new();

    config.read_args();

    let mut abc = genpass::Alphabet::new(&config);

    let mut pass = String::new();

    for _ in 0..constants::DEFAULT_PASS_LEN {
        pass.push(abc.get_char());
    }

    println!("{:?}", pass);
}
