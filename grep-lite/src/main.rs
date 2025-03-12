use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{App, Arg};

fn process_lines<T: BufRead + Sized>(reader: T, regex_pattern: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match regex_pattern.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        };
    }
}

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("input")
            .help("File to search")
            .takes_value(true)
            .required(false))
        .get_matches();

    let pattern = match args.value_of("pattern") {
        Some(value) => value,
        None => {
            eprintln!("Error: No pattern provided.");
            return;
        }
    };

    let regex_pattern = match Regex::new(pattern) {
        Ok(regex) => regex,
        Err(e) => {
            eprintln!("Failed to compile regex: {}", e);
            return;
        }
    };

    let input = match args.value_of("input") {
        Some(value) => value,
        None => "--",
    };

    if input == "--" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, regex_pattern)
    } else {
        let f = match File::open(input) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("Could not find supplied file path: {}", input);
                return;
            }
        };

        let reader = BufReader::new(f);
        process_lines(reader, regex_pattern);
    }
}
