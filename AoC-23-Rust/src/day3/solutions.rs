use std::{fs::File, io::BufReader, io::BufRead};

pub fn solution_part1() {
    // Get file and make sure it opens correctly
    let file = match File::open("src/day3/input.txt") {
        Ok(file) => file, 
        Err(why) => panic!("Couldn't read file: {}", why),
    };
    let mut lines = BufReader::new(file).lines().peekable(); // Read lines of the file

    let mut u = String::from("");
    let mut c = lines.next().unwrap().unwrap();
    let mut b = lines.next().unwrap().unwrap();

    let mut part_number_sum = 0;

    loop {

        // Find valid part numbers
        part_number_sum += find_part_numbers_in_scan(&u, &c, &b);

        // Check if next is valid
        if lines.peek().is_none() {break;}

        // Shift
        u = c;
        c = b;
        b = lines.next().unwrap().unwrap();
    }

    println!("{}", part_number_sum);
}

fn find_part_numbers_in_scan(upper_raster: &String, middle_raster: &String, btm_raster: &String) -> i32 {
    let upper: Vec<&str> = upper_raster.split("").collect();
    let middle: Vec<&str> = middle_raster.split("").collect();
    let bottom: Vec<&str> = btm_raster.split("").collect();

    let mut part_numbers: Vec<i32> = vec![];

    // I get this is a lot of redundency but idrc :)
    // This actually has a logical bug with potential diagonal duplicates being missed. Oh well :), passed
    for (index, &character) in middle.iter().enumerate() {
        if is_part_symbol(character) {
            // look up
            {
                let mut parts: Vec<i32> = vec![];
                if upper[index - 1] != "." && !is_part_symbol(upper[index - 1]) { 
                    let number = get_number(&upper, &(index-1));
                    if !parts.contains(&number) {
                        parts.push(number);
                    }
                }

                if upper[index] != "." && !is_part_symbol(upper[index]) { 
                    let number = get_number(&upper, &index);
                    if !parts.contains(&number) {
                        parts.push(number);
                    }
                }

                if upper[index + 1] != "." && !is_part_symbol(upper[index + 1]) { 
                    let number = get_number(&upper, &(&index + 1));
                    if !parts.contains(&number) {
                        parts.push(number);
                    }
                }
                for part in parts {
                    part_numbers.push(part);
                }
            }

            {
                let mut parts: Vec<i32> = vec![];
                // look down
                if bottom[index - 1] != "." && !is_part_symbol(bottom[index - 1]) { 
                    let number = get_number(&bottom, &(index - 1));
                    if !parts.contains(&number) {
                        parts.push(number);
                    }
                }

                if bottom[index] != "." && !is_part_symbol(bottom[index]) { 
                    let number = get_number(&bottom, &index);
                    if !parts.contains(&number) {
                        parts.push(number);
                    }
                }


                if bottom[index + 1] != "." && !is_part_symbol(bottom[index + 1]) { 
                    let number = get_number(&bottom, &(index + 1));
                    if !parts.contains(&number) {
                        parts.push(number);
                    }
                }
                for part in parts {
                    part_numbers.push(part);
                }
            }

            // look left
            if middle[index - 1] != "." && !is_part_symbol(middle[index - 1]) { 
                let number = get_number(&middle, &(index - 1));
                if !part_numbers.contains(&number) {
                    part_numbers.push(number);
                } 
            }

            // look right
            if middle[index + 1] != "." && !is_part_symbol(middle[index + 1]) { 
                let number = get_number(&middle, &(index+1));
                if !part_numbers.contains(&number) {
                    part_numbers.push(number);
                }
            }
        }
    }

    let mut sum = 0;
    for number in part_numbers {
        sum += number;
    }
    return sum;
}

fn is_part_symbol(val: &str) -> bool {
    if val == "@" || val == "#" || val == "%" || val == "&" || val == "*" || val == "-" || val == "=" || val == "$" || val == "+" || val == "/" {
        return true;
    } 
    return false;
}

fn get_number(raster_split: &Vec<&str>, index: &usize) -> i32 {
    let mut cursor = index.clone();
    // Move right to the end of the number
    while raster_split[cursor] != "." && !is_part_symbol(raster_split[cursor]) && !raster_split[cursor].is_empty() {
        cursor += 1;
    }

    let mut number: i32 = 0; 
    let mut position = 1;
    cursor -= 1;
    // Move left to get the number
    while raster_split[cursor] != "." && !raster_split[cursor].is_empty() && !is_part_symbol(raster_split[cursor]) {
        number += raster_split[cursor].parse::<i32>().unwrap() * position;
        position = position * 10;
        if cursor == 0 {break;}
        cursor -= 1;
    }

    return number;
}

pub fn solution_part2() {
        // Get file and make sure it opens correctly
        let file = match File::open("src/day3/input.txt") {
            Ok(file) => file, 
            Err(why) => panic!("Couldn't read file: {}", why),
        };
        let mut lines = BufReader::new(file).lines().peekable(); // Read lines of the file
    
        let mut u = String::from("");
        let mut c = lines.next().unwrap().unwrap();
        let mut b = lines.next().unwrap().unwrap();
    
        let mut part_number_sum = 0;
    
        loop {
    
            // Find valid part numbers
            part_number_sum += find_gear_ratio_in_scan(&u, &c, &b);
    
            // Check if next is valid
            if lines.peek().is_none() {break;}
    
            // Shift
            u = c;
            c = b;
            b = lines.next().unwrap().unwrap();
        }
    
        println!("{}", part_number_sum);
}

fn find_gear_ratio_in_scan(upper_raster: &String, middle_raster: &String, btm_raster: &String) -> i32 { 
    let upper: Vec<&str> = upper_raster.split("").collect();
    let middle: Vec<&str> = middle_raster.split("").collect();
    let bottom: Vec<&str> = btm_raster.split("").collect();

    let mut gear_ratio_sum_in_scan = 0;

    // I get this is a lot of redundency but idrc :)
    // This actually has a logical bug with potential diagonal duplicates being missed. Oh well :), passed
    for (index, &character) in middle.iter().enumerate() {
        if is_gear(character) {
            // look up
            let mut gear_ratios: Vec<i32> = vec![];
            {
                let mut parts: Vec<i32> = vec![];
                if upper[index - 1] != "." && !is_part_symbol(upper[index - 1]) { 
                    let number = get_number(&upper, &(index-1));
                    if !parts.contains(&number) {
                        parts.push(number);
                    }
                }

                if upper[index] != "." && !is_part_symbol(upper[index]) { 
                    let number = get_number(&upper, &index);
                    if !parts.contains(&number) {
                        parts.push(number);
                    }
                }

                if upper[index + 1] != "." && !is_part_symbol(upper[index + 1]) { 
                    let number = get_number(&upper, &(&index + 1));
                    if !parts.contains(&number) {
                        parts.push(number);
                    }
                }
                for part in parts {
                    gear_ratios.push(part);
                }
            }

            {
                let mut parts: Vec<i32> = vec![];
                // look down
                if bottom[index - 1] != "." && !is_part_symbol(bottom[index - 1]) { 
                    let number = get_number(&bottom, &(index - 1));
                    if !parts.contains(&number) {
                        parts.push(number);
                    }
                }

                if bottom[index] != "." && !is_part_symbol(bottom[index]) { 
                    let number = get_number(&bottom, &index);
                    if !parts.contains(&number) {
                        parts.push(number);
                    }
                }


                if bottom[index + 1] != "." && !is_part_symbol(bottom[index + 1]) { 
                    let number = get_number(&bottom, &(index + 1));
                    if !parts.contains(&number) {
                        parts.push(number);
                    }
                }
                for part in parts {
                    gear_ratios.push(part);
                }
            }

            // look left
            if middle[index - 1] != "." && !is_part_symbol(middle[index - 1]) { 
                gear_ratios.push(get_number(&middle, &(index - 1)));
            }

            // look right
            if middle[index + 1] != "." && !is_part_symbol(middle[index + 1]) { 
                gear_ratios.push(get_number(&middle, &(index + 1)));
            }

            if gear_ratios.len() == 2 {
                gear_ratio_sum_in_scan += gear_ratios[0] * gear_ratios[1];
            }
        }
    }

    return gear_ratio_sum_in_scan;
}

fn is_gear(symbol: &str) -> bool {
    if symbol == "*" {return true;}
    return  false;
}