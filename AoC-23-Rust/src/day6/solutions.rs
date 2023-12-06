use std::num;

use crate::file_utility::file_utility::get_file_lines;

/// Ay this was great for part 2! Just remove the white space in the input and ezpz
///
pub fn solution_part1() {
    let mut file_reader = get_file_lines("sample.txt", 6);

    let times: Vec<f64> = file_reader
        .next()
        .unwrap()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split_ascii_whitespace()
        .map(|num| num.parse::<f64>().unwrap())
        .collect();
    let distances: Vec<f64> = file_reader
        .next()
        .unwrap()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split_ascii_whitespace()
        .map(|num| num.parse::<f64>().unwrap())
        .collect();

    let mut possible_winnings: Vec<f64> = vec![];
    /*
       x = time we need to hold the boat.
       T = record time
       D = record distance

       x(T - x) > D
       Tx - x^2 - D > 0

       a = -1
       b = T
       c = -D

       Quadradic formula time, baby!
       + is lower bound
       - is upper bound
    */

    for (index, &time) in times.iter().enumerate() {
        let a: f64 = -1.0;
        let b = time;
        let c = -distances[index];

        let mut lower_bound =
            (-b + f64::sqrt(b.powf(2.0) - 4 as f64 * a * c) as f64) / (2 as f64 * a);
        if lower_bound - lower_bound.floor() > 0.0 {
            lower_bound = lower_bound.ceil();
        } else {
            lower_bound += 1.0;
        }

        let mut upper_bound =
            (-b - f64::sqrt(b.powf(2.0) - 4 as f64 * a * c) as f64) / (2 as f64 * a);
        if upper_bound - upper_bound.floor() > 0.0 {
            upper_bound = upper_bound.floor();
        } else {
            upper_bound -= 1.0;
        }

        possible_winnings.push(upper_bound - lower_bound + 1 as f64);
    }

    let mut product = 1.0;
    for winning in possible_winnings {
        product *= winning;
    }
    println!("{}", product);
}
