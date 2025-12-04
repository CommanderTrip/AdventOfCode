use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_reader(file_name: &str) -> std::iter::Peekable<std::io::Lines<BufReader<File>>> {
    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't read file: {}", why),
    };
    BufReader::new(file).lines().peekable()
}