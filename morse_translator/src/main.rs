extern crate regex;

mod encode;
mod decode;

use std::io;
use regex::Regex;

fn main() {
    println!("Morse code encode v1");
    let valid_ascii_chars = Regex::new(r"(?i)[a-z0-9 .\\n]");
    let valid_morse_chars = Regex::new(r"[._\-\*/]");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            if valid_ascii_chars.unwrap().is_match(&input) {
                let output :String = encode::encode(input);
                println!("{}", output);
            } else if valid_morse_chars.unwrap().is_match(&input) {
                let output :String = decode::decode(input);
                println!("{}", output);
            } else {
                panic!("use of unsupported characters.");
            }
        }
        Err(error) => println!("Error: {}", error)
    }
}

