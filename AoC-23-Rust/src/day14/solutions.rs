use crate::utility::file_utility::get_file_lines;

pub fn solution_part1() -> i32 {
    let mut file_reader = get_file_lines("input.txt", 14);

    let mut m: Vec<Vec<char>> = vec![];

    let mut i = 0;
    while file_reader.peek().is_some() {
        m.push(file_reader.next().unwrap().unwrap().chars().collect());

        let line = m[i].clone();
        for (index, char) in line.iter().enumerate() {
            let mut j = i.clone();

            if *char != 'O' {
                continue;
            }

            while j > 0 {
                if m[j - 1][index] != '.' {
                    break;
                }
                m[j][index] = '.';
                j -= 1;
                m[j][index] = 'O';
            }
        }

        i += 1;
    }

    let mut solution = 0;
    for line in m {
        let mut line_sum = 0;
        for char in line {
            if char == 'O' {
                line_sum += 1;
            }
        }
        solution += line_sum * i;
        i -= 1;
    }

    return solution as i32;
}

pub fn solution_part2() -> i32 {
    let mut file_reader = get_file_lines("sample.txt", 14);

    let mut m: Vec<Vec<char>> = vec![];

    while file_reader.peek().is_some() {
        m.push(file_reader.next().unwrap().unwrap().chars().collect());
    }

    rotate_clockwise(m);
    return 1;
}

fn rotate_clockwise(dish: Vec<Vec<char>>) {
    let mut rotated_dish = dish.clone();

    for line in &dish {
        println!("{:?}", line);
    }

    for (y, line) in dish.iter().enumerate() {
        for (x, value) in line.iter().enumerate() {
            // Map
            let mut new_x = x as i32 - (line.len() - 1) as i32 / 2;
            let mut new_y = y as i32 - (line.len() - 1) as i32 / 2;

            // Rotate
            new_x = new_y;
            new_y = -new_x;

            // Unmap
            new_x = new_x + (line.len() - 1) as i32 / 2;
            new_y = new_y + (line.len() - 1) as i32 / 2;

            println!("{},{} to {},{}", x, y, new_x, new_y);
            rotated_dish[new_x as usize][new_y as usize] = *value;
        }
    }
    println!("----------");

    for line in rotated_dish {
        println!("{:?}", line);
    }
}
