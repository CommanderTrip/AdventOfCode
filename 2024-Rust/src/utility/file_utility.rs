use std::{fs::File, io::BufRead, io::BufReader};

pub fn get_file_lines(
    file_name: &str,
    day: i32,
) -> std::iter::Peekable<std::io::Lines<BufReader<File>>> {
    let file = match File::open("src/day".to_owned() + &day.to_string() + "/" + file_name) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't read file: {}", why),
    };
    return BufReader::new(file).lines().peekable();
}
