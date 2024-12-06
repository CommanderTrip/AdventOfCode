use crate::utility::file_utility::get_file_lines;

pub fn solution_part1() -> i32 {
    let mut file_reader = get_file_lines("sample.txt", 12);

    while file_reader.peek().is_some() {
        // Parse the problem
        let line = file_reader.next().unwrap().unwrap();
        let split: Vec<&str> = line.split_ascii_whitespace().collect();
        let mut record: Vec<&str> = split[0]
            .split(".")
            .filter(|pattern| !pattern.is_empty())
            .collect();
        let mut pattern: Vec<i32> = split[1]
            .split(",")
            .map(|rule| rule.parse::<i32>().unwrap())
            .collect();

        // Simpliy the problem by removing trivial sections
        for sequence in &mut record {
            if !sequence.contains("?") {
                for size in &mut pattern {
                    if sequence.len() == *size as usize {
                        *sequence = "";
                        *size = 0;
                    }
                }
            }
        }
        let new_record: Vec<&&str> = record
            .iter()
            .filter(|val| !val.is_empty())
            .collect::<Vec<&&str>>();
        let new_pattern: Vec<&i32> = pattern
            .iter()
            .filter(|val| **val != 0)
            .collect::<Vec<&i32>>();

        let mut solutions = 0;
        print!("{:?} {:?} ", new_record, new_pattern);

        if new_record.len() == new_pattern.len() {
            // Combination equation
            for (index, sequence) in new_record.iter().enumerate() {
                let n = sequence.len() as usize;
                let r = *new_pattern[index] as usize;
                let mut combinations: usize = (1..=n).product::<usize>()
                    / ((1..=r).product::<usize>() * (1..=(n - r)).product::<usize>());

                if sequence.contains('#') && combinations == 1 {
                    combinations = 0
                }
                solutions += combinations;
            }
        } else {
            for sequence in new_record {
                let sum = new_pattern.clone().iter().fold(0, |acc, x| acc + **x);

                if sequence.len() - 1 < sum as usize + new_pattern.len() {
                    solutions += 1;
                    break;
                }

                // scan an fit
                for pattern in &new_pattern {
                    let pos = sequence.find(&"#".repeat(**pattern as usize));
                    if pos.is_some() {
                        let (_, rest) = sequence.split_at(pos.unwrap() + **pattern as usize);
                    }
                }
            }
        }
        println!("{solutions}");
    }

    return 1;
}
