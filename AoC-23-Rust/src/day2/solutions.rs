use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn solution_part1() {
        // Get file and make sure it opens correctly
        let file = match File::open("src/day2/input.txt") {
            Ok(file) => file, 
            Err(why) => panic!("Couldn't read file: {}", why),
        };

        // Rules
        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;

        let mut possible_games: Vec<i32> = vec![];
    
        for line in BufReader::new(file).lines() {
            if let Ok(game) = line {
                let game_split: Vec<&str> = game.split(":").collect();
                let mut is_possible = true;

                let game_title: Vec<&str> = game_split[0].split(" ").collect();
                let game_id = game_title[1].parse::<i32>().unwrap();

                let sets: Vec<&str> = game_split[1].split(";").collect();
                for set in sets {
                    if !is_possible { break; };
                    let colored_cubes: Vec<&str> = set.split(",").collect();

                    for cube in colored_cubes {
                        let cube_split: Vec<&str> = cube.split_whitespace().collect();
                        let number = cube_split[0].parse::<i32>().unwrap();
                        let color = cube_split[1];

                        match color {
                            "red" => {if number > max_red {is_possible = false; break;}},
                            "green" => {if number > max_green {is_possible = false; break;}},
                            "blue" => {if number > max_blue {is_possible = false; break;}},
                            _ => {}
                        }
                    }
                }
                if is_possible {possible_games.push(game_id);}
            }
        }

        let mut sum = 0;
        for game in possible_games {
            sum += game;
        }
        println!("{}", sum);
}

pub fn solution_part2() {
      // Get file and make sure it opens correctly
      let file = match File::open("src/day2/input.txt") {
        Ok(file) => file, 
        Err(why) => panic!("Couldn't read file: {}", why),
    };

    let mut min_sum: Vec<i32> = vec![];

    for line in BufReader::new(file).lines() {
        if let Ok(game) = line {
            let game_split: Vec<&str> = game.split(":").collect();

            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;

            let sets: Vec<&str> = game_split[1].split(";").collect();
            for set in sets {
                let colored_cubes: Vec<&str> = set.split(",").collect();

                for cube in colored_cubes {
                    let cube_split: Vec<&str> = cube.split_whitespace().collect();
                    let number = cube_split[0].parse::<i32>().unwrap();
                    let color = cube_split[1];

                    match color {
                        "red" => {if number > min_red {min_red = number}},
                        "green" => {if number > min_green {min_green = number}},
                        "blue" => {if number > min_blue {min_blue = number}},
                        _ => {}
                    }
                }
            }
            min_sum.push(min_red * min_green * min_blue);
        }
    }

    let mut sum = 0;
    for game in min_sum {
        sum += game;
    }
    println!("{}", sum);
}