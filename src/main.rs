extern crate rand;

use std::process;

mod constants;
mod genpass;
mod parseargs;

fn main() {
    // Set default config
    let mut config = parseargs::ConfigArgs::new();

    // Read command line args
    config.read_args();

    // If invalid arguments or -h or --help, print usage dialogue and die
    if config.print_help {
        print_usage();
        process::exit(0);
    }

    // Generate our alphabet to choose chars from
    let mut abc = genpass::Alphabet::new(&config);

    let mut pass = String::new();

    // Build our password
    for _ in 0..config.pass_length {
        pass.push(abc.get_char());
    }

    //emit password to user
    match config.disable_newline {
        true => print!("{}", pass),
        false => println!("{}", pass),
    }
}

// Print usage data
fn print_usage() {
    let mut message = String::new();
    // Cargo sets this env var based on the value in the toml file.
    // We want to error out if not defined, as the env macro does
    message += &format!("Version {}\n\n", env!("CARGO_PKG_VERSION"));
    message += "Usage: pass-rs: [options] [len]\n\n";
    message += "By default, the password is 38 characters long.\n";
    message += "For a custom length, simply specify a numeric length as an argument.\n\n";
    message += "Options:\n";
    message += "-n             Disable newline when printing password\n";
    message += "-nd            Disable numeric characters\n";
    message += "-nl            Disable lowercase characters\n";
    message += "-nu            Disable uppercase characters\n";
    message += "-s             Enable special characters in generations (!, @, #, $, etc)\n";
    message += "-h, --help     Print this help dialogue\n\n";

    println!("{}", &message);
}
