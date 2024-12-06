use std::collections::HashMap;

use crate::utility::file_utility::get_file_lines;

pub fn part1(file_name: &str) -> i32 {
    let mut file_reader = get_file_lines(file_name, 1);

    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];

    while file_reader.peek().is_some() {
        let line = file_reader.next().unwrap().unwrap();
        let split: Vec<&str> = line.split(" ").collect();
        left_list.push(split[0].parse().unwrap());
        right_list.push(split[split.len() - 1].parse().unwrap());
    }

    if left_list.len() != right_list.len() {
        println!("List sizses did not match");
        return -1;
    }

    left_list.sort();
    right_list.sort();

    let mut running_total = 0;
    for (i, num) in left_list.iter().enumerate() {
        running_total += (num - right_list[i]).abs();
    }

    return running_total;
}

pub fn part2(file_name: &str) -> i32 {
    let mut file_reader = get_file_lines(file_name, 1);

    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];

    while file_reader.peek().is_some() {
        let line = file_reader.next().unwrap().unwrap();
        let split: Vec<&str> = line.split(" ").collect();
        left_list.push(split[0].parse().unwrap());
        right_list.push(split[split.len() - 1].parse().unwrap());
    }

    let similarity_map: HashMap<i32, i32> = HashMap::new();
    let mut running_total = 0;
    for num in left_list.iter() {
        match similarity_map.get(num) {
            Some(similarity) => running_total += similarity,
            _ => {
                let mut similarity_count = 0;
                for val in (&mut right_list).iter() {
                    if num == val {
                        similarity_count += 1;
                    }
                }
                running_total += num * similarity_count;
            }
        }
    }
    return running_total;
}

#[test]
fn part1_solution() {
    assert_eq!(part1("sample.txt"), 11);
}

#[test]
fn part2_solution() {
    assert_eq!(part2("sample.txt"), 31);
}
