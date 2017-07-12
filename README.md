# pass-rs

*A simple static password generator for rust*

## Motivation
I recently picked up a [Yubikey 4 nano](https://www.yubico.com/products/yubikey-hardware/yubikey4/). After setting it up as a GPG smart card and using the first slot for Yubico OTP, I wanted to set a strong, static password to be emitted from the second slot.

Curiously, the [Yubikey Personalization Tool](https://github.com/Yubico/yubikey-personalization) doesn't include a tool to auto-generate a strong password. So, I decided to write pass-rs as a simple tool for generating strong static passwords to be stored in password managers.

## Design
This tool uses a custom RNG based on the [KISS RNG ](http://www0.cs.ucl.ac.uk/staff/D.Jones/GoodPracticeRNG.pdf) originally proposed by G.Marsaglia. The tool uses this to emit a password based solely on ASCII characters for broad compatibility.

By default, these passwords are 38 characters long, the max supported by Yubikeys with firmware >= 2.2. For lesser firmware versions , the limit is 16 characters so a different length can be specified.

## Installation
Make sure you've got the standard rust toolchain installed (see [Rustup](https://www.rustup.rs/) for help).


Then, simply clone this repo and install via `cargo`:

```
git clone https://github.com/Jarusk/pass-rs.git
cd pass-rs/
cargo install
```

## Usage
```
Usage: pass-rs: [options] [len]

By default, the password is 38 characters long.
For a custom length, simply specify a numeric length as an argument.

Options:
-nl            Disable lowercase characters
-nu            Disable uppercase characters
-nd            Disable numeric characters
-s             Enable special characters in generations (!, @, #, $, etc)
-h, --help     Print this help dialogue
```
