use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn solution_part1() {
    // Get file and make sure it opens correctly
    let file = match File::open("src/day1/input.txt") {
        Ok(file) => file, 
        Err(why) => panic!("Couldn't read file: {}", why),
    };

    let mut calibration_sum: i32 = 0; // This will be the solution

    // Loop over every line and get the calibration values
    for line in BufReader::new(file).lines() {
        if let Ok(calibration_input) = line {
            let mut calibration_value = [-1, -1];

            for c in calibration_input.chars() {
                if c.is_ascii_digit() {

                    // Always set the second value
                    calibration_value[1] = c.to_digit(10).unwrap() as i32;

                    // Only set the first value once -- "-1" will never be set again
                    if calibration_value[0] == -1 {
                        calibration_value[0] = c.to_digit(10).unwrap() as i32 * 10;
                    }
                }
            }

            calibration_sum += calibration_value[0] + calibration_value[1];
        }
    }

    println!("{}", calibration_sum);
}

pub fn solution_part2() {
    // Get file and make sure it opens correctly
    let file = match File::open("src/day1/input.txt") {
        Ok(file) => file, 
        Err(why) => panic!("Couldn't read file: {}", why),
    };

    let mut calibration_sum: i32 = 0; // This will be the solution

     // Loop over every line and get the calibration values
     for line in BufReader::new(file).lines() {
        if let Ok(calibration_input) = line {
            let mut calibration_value = [-1, -1];

            for (i, c) in calibration_input.chars().enumerate() {
                if c.is_ascii_digit() {
                    add_calibration_value(&mut calibration_value, c);
                } else {
                    match c {
                        'o' => {
                            if i + 2 < calibration_input.len() &&
                            calibration_input.chars().nth(i + 1) == Some('n') &&
                            calibration_input.chars().nth(i + 2) == Some('e') {
                                add_calibration_value(&mut calibration_value, '1');
                            }
                        },
                        't' => {
                            if i + 2 < calibration_input.len() &&
                            calibration_input.chars().nth(i + 1) == Some('w') &&
                            calibration_input.chars().nth(i + 2) == Some('o') {
                                add_calibration_value(&mut calibration_value, '2');
                            } else if i + 4 < calibration_input.len() &&
                            calibration_input.chars().nth(i + 1) == Some('h') &&
                            calibration_input.chars().nth(i + 2) == Some('r') &&
                            calibration_input.chars().nth(i + 3) == Some('e') &&
                            calibration_input.chars().nth(i + 4) == Some('e') {
                                add_calibration_value(&mut calibration_value, '3');
                            }
                        },
                        'f' => {
                            if i + 3 < calibration_input.len() &&
                            calibration_input.chars().nth(i + 1) == Some('o') &&
                            calibration_input.chars().nth(i + 2) == Some('u') &&
                            calibration_input.chars().nth(i + 3) == Some('r') {
                                add_calibration_value(&mut calibration_value, '4');
                            } else if i + 3 < calibration_input.len() &&
                            calibration_input.chars().nth(i + 1) == Some('i') &&
                            calibration_input.chars().nth(i + 2) == Some('v') &&
                            calibration_input.chars().nth(i + 3) == Some('e') {
                                add_calibration_value(&mut calibration_value, '5');
                            }
                        },
                        's' => {
                            if i + 2 < calibration_input.len() &&
                            calibration_input.chars().nth(i + 1) == Some('i') &&
                            calibration_input.chars().nth(i + 2) == Some('x') {
                                add_calibration_value(&mut calibration_value, '6');
                            } else if i + 4 < calibration_input.len() &&
                            calibration_input.chars().nth(i + 1) == Some('e') &&
                            calibration_input.chars().nth(i + 2) == Some('v') &&
                            calibration_input.chars().nth(i + 3) == Some('e') &&
                            calibration_input.chars().nth(i + 4) == Some('n') {
                                add_calibration_value(&mut calibration_value, '7');
                            }
                        },
                        'e' => {
                            if i + 4 < calibration_input.len() &&
                            calibration_input.chars().nth(i + 1) == Some('i') &&
                            calibration_input.chars().nth(i + 2) == Some('g') &&
                            calibration_input.chars().nth(i + 3) == Some('h') &&
                            calibration_input.chars().nth(i + 4) == Some('t') {
                                add_calibration_value(&mut calibration_value, '8');
                            }
                        },
                        'n' => {
                            if i + 3 < calibration_input.len() &&
                            calibration_input.chars().nth(i + 1) == Some('i') &&
                            calibration_input.chars().nth(i + 2) == Some('n') &&
                            calibration_input.chars().nth(i + 3) == Some('e') {
                                add_calibration_value(&mut calibration_value, '9');
                            }
                        },
                        _ => continue
                    }
                }
            }

            calibration_sum += calibration_value[0] + calibration_value[1];
        }
    }
    println!("{}", calibration_sum);
}

fn add_calibration_value(calibration_value: &mut [i32; 2], value: char) {
    // Always set the second value
    calibration_value[1] = value.to_digit(10).unwrap() as i32;

    // Only set the first value once -- "-1" will never be set again
    if calibration_value[0] == -1 {
        calibration_value[0] = value.to_digit(10).unwrap() as i32 * 10;
    }
}