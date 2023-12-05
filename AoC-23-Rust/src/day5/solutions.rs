use crate::file_utility::file_utility::get_file_lines;

pub fn solution_part1() {
    let mut file_reader = get_file_lines("input.txt", 5);

    let seeds_line = file_reader.next().unwrap().unwrap();
    let line_split: Vec<&str> = seeds_line.split(":").collect();
    let seeds: Vec<&str> = line_split[1].split_ascii_whitespace().collect();
    let mut seed_map: Vec<Vec<i64>> = vec![];

    // Init the solution arrays
    for seed in seeds {
        seed_map.push(vec![seed.parse::<i64>().unwrap()]);
    }
    let mut map_iter = 0; // Which map we are currently processing

    while file_reader.peek().is_some() {
        let line = file_reader.next().unwrap().unwrap();

        if line.is_empty() {
            continue;
        }

        // Get all the ranges
        let mut ranges: Vec<String> = vec![];
        if line.ends_with(":") {
            while file_reader
                .peek()
                .is_some_and(|line| !line.as_ref().unwrap().is_empty())
            {
                ranges.push(file_reader.next().unwrap().unwrap());
            }
        }

        // Copy for the loop
        let map_copy: Vec<Vec<i64>> = seed_map.clone();
        let mut seed_number = 0;

        // Compare each seed to the list of ranges and map as needed
        for seed in map_copy {
            let mut mapped = false;
            for range in &ranges {
                let range_split: Vec<&str> = range.split_ascii_whitespace().collect();
                let destination = range_split[0].parse::<i64>().unwrap();
                let source = range_split[1].parse::<i64>().unwrap();
                let range_size = range_split[2].parse::<i64>().unwrap() - 1;

                if seed[map_iter] >= source && seed[map_iter] <= source + range_size {
                    // Seed is in range to map
                    let distance = seed[map_iter] - source;
                    seed_map[seed_number].push(destination + distance);
                    mapped = true;
                    break;
                }
            }
            if !mapped {
                let push_val = seed_map[seed_number][map_iter]; // copy for borrow
                seed_map[seed_number].push(push_val);
            }
            seed_number += 1;
        }
        map_iter += 1;
    }

    let mut closest = i64::MAX;
    for seed in seed_map {
        if seed.last().unwrap() < &closest {
            closest = *seed.last().unwrap();
        }
    }
    println!("{}", closest);
}

pub fn solution_part2() {
    let mut file_reader = get_file_lines("input.txt", 5);

    let seeds_line: String = file_reader.next().unwrap().unwrap();
    let line_split: Vec<&str> = seeds_line.split(":").collect();
    let seed_ranges: Vec<i64> = line_split[1]
        .split_ascii_whitespace()
        .map(|r| r.parse::<i64>().unwrap())
        .collect();

    let mut ranges: Vec<Vec<String>> = vec![];
    let mut range_iter = 0;
    while file_reader.peek().is_some() {
        let line = file_reader.next().unwrap().unwrap();

        if line.is_empty() {
            continue;
        }

        // Get all the ranges
        if line.ends_with(":") {
            ranges.push(vec![]);
            while file_reader
                .peek()
                .is_some_and(|line| !line.as_ref().unwrap().is_empty())
            {
                ranges[range_iter].push(file_reader.next().unwrap().unwrap());
            }
        }
        range_iter += 1;
    }
    ranges.reverse();

    // Brute force this bitch :)
    // Location from part one can be the upper bound location 218513636, 109256818
    let start = 81973084;
    'search: for location in 81080815..start {
        let mut current_seed = location;

        // Reverse location to get seed.
        for map_details in &ranges {
            for map in map_details {
                let split: Vec<i64> = map
                    .split_ascii_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect();
                let dest = split[0];
                let src = split[1];
                let range = split[2];

                if current_seed >= dest && current_seed < dest + range {
                    // seed was mapped
                    current_seed = src + (current_seed - dest);
                    break;
                }
            }
        }

        let mut i = 0;
        while i < seed_ranges.len() {
            if current_seed >= seed_ranges[i] && current_seed <= seed_ranges[i] + seed_ranges[i + 1]
            {
                // 81956384 - FUCK THIS ONE! :) i am a terrible developer
                println!("Location {} maps to seed {}.", location, current_seed);
                break 'search;
            }
            i += 2;
        }
    }
}
