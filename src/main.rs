extern crate rand;

use std::process;

mod parseargs;
mod constants;
mod genpass;


fn main() {
    // Set default config
    let mut config = parseargs::ConfigArgs::new();

    // Read command line args
    config.read_args();

    if config.print_help {
        print_usage();
        process::exit(0);
    }

    let mut abc = genpass::Alphabet::new(&config);

    let mut pass = String::new();

    for _ in 0..constants::DEFAULT_PASS_LEN {
        pass.push(abc.get_char());
    }

    println!("{}", pass);
}

fn print_usage() {
    unimplemented!()
}
