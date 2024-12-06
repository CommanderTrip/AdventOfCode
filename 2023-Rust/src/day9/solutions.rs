use std::collections::btree_map::Values;

use crate::utility::file_utility::get_file_lines;

pub fn solution_part1() -> i32 {
    let mut file_reader = get_file_lines("input.txt", 9);

    let mut extrapolated_sum = 0;
    while file_reader.peek().is_some() {
        let mut history: Vec<i32> = file_reader
            .next()
            .unwrap()
            .unwrap()
            .split_ascii_whitespace()
            .map(|number| number.parse::<i32>().unwrap())
            .collect();
        history.reverse(); // Just adding this solved part 2
        extrapolated_sum += extrapolated(history);
    }

    return extrapolated_sum;
}

fn extrapolated(values: Vec<i32>) -> i32 {
    let mut last = true;
    let mut next_layer = vec![];

    for (index, value) in values.iter().enumerate() {
        if index == values.len() - 1 {
            break;
        }
        let next_layer_value = values[index + 1] - value;

        if next_layer_value != 0 {
            last = false;
        }

        next_layer.push(next_layer_value);
    }
    if last {
        return *values.last().unwrap();
    }

    return *values.last().unwrap() + extrapolated(next_layer);
}

#[test]
fn test() {
    // assert_eq!(solution_part1(), 114); // Sample
    // assert_eq!(solution_part1(), 1898776583); // No reverse
    assert_eq!(solution_part1(), 1100);
}
