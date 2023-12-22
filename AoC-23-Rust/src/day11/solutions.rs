use crate::utility::file_utility::get_file_lines;

#[derive(Debug)]
struct Galaxy {
    x: usize,
    y: usize,
    x_adj: usize,
    y_adj: usize,
}

pub fn solution_part1() -> usize {
    let mut file_reader = get_file_lines("sample.txt", 11);

    let mut universe: Vec<Vec<char>> = vec![];

    while file_reader.peek().is_some() {
        let line: Vec<char> = file_reader.next().unwrap().unwrap().chars().collect();
        let clone = line.clone();

        universe.push(line);

        // Account for universe expansion rows
        let has_galaxy = clone.iter().any(|&x| x == '#');
        if !has_galaxy {
            universe.push(clone)
        }
    }

    // Account for universe expansion in columns
    let mut i = 0;
    'a: while i < universe[0].len() {
        for row in &universe {
            if row[i] == '#' {
                i += 1;
                continue 'a;
            }
        }
        for row in &mut universe {
            row.insert(i, '.');
        }
        i += 2;
    }

    // Collect all the galaxies
    let mut galaxies: Vec<Galaxy> = vec![];
    for (y, row) in universe.iter().enumerate() {
        for (x, symbol) in row.iter().enumerate() {
            if *symbol == '#' {
                galaxies.push(Galaxy {
                    x,
                    x_adj: 0,
                    y,
                    y_adj: 0,
                })
            }
        }
    }

    // Find distances between each one
    let mut sum_distance = 0;
    for (first, galaxy) in galaxies.iter().enumerate() {
        for (second, galaxy2) in galaxies.iter().enumerate() {
            if second <= first {
                continue;
            }
            sum_distance +=
                (galaxy2.y - galaxy.y) + (galaxy2.x as i32 - galaxy.x as i32).abs() as usize
        }
    }

    return sum_distance;
}

pub fn solution_part2() -> usize {
    let mut file_reader = get_file_lines("sample.txt", 11);

    let mut universe: Vec<Vec<char>> = vec![];
    let mut galaxies: Vec<Galaxy> = vec![];

    while file_reader.peek().is_some() {
        let line: Vec<char> = file_reader.next().unwrap().unwrap().chars().collect();

        universe.push(line);
    }

    let multiplier = 10;
    let mut y_expansion = 0;
    let mut x_expansion = 0;

    // get every galaxy from the initial image with y expansion
    for (y, row) in universe.iter().enumerate() {
        let mut blank_row = true;
        for (x, symbol) in row.iter().enumerate() {
            if *symbol == '#' {
                galaxies.push(Galaxy {
                    x,
                    x_adj: x,
                    y,
                    y_adj: y + y_expansion,
                });
                blank_row = false;
            }
        }
        if blank_row {
            y_expansion += multiplier
        }
    }

    // Account for x expansion
    let mut i = 0;
    'a: while i < universe[0].len() {
        for row in &universe {
            if row[i] == '#' {
                i += 1;
                continue 'a;
            }
        }
        x_expansion += multiplier;
        for galaxy in &mut galaxies {
            if galaxy.x > i {
                galaxy.x_adj += x_expansion as usize;
            }
        }
        i += 1
    }

    println!("{:?}", galaxies);

    // Find distances between each one
    let mut sum_distance = 0;
    for (first, galaxy) in galaxies.iter().enumerate() {
        for (second, galaxy2) in galaxies.iter().enumerate() {
            if second <= first {
                continue;
            }
            sum_distance += (galaxy2.y_adj - galaxy.y_adj)
                + (galaxy2.x_adj as i32 - galaxy.x_adj as i32).abs() as usize
        }
    }

    return sum_distance;
}
