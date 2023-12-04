use std::{fs::File, io::BufReader, io::BufRead};

pub fn solution_part1() {
    // Get file and make sure it opens correctly
    let file = match File::open("src/day4/input.txt") {
        Ok(file) => file, 
        Err(why) => panic!("Couldn't read file: {}", why),
    };

    let mut scratch_cards_value = 0;

    for line in BufReader::new(file).lines() {
        if let Ok(card) = line {
            let split: Vec<&str> = card.split(":").collect();
            let numbers: Vec<&str> = split[1].split("|").collect();

            let mut winning_numbers: Vec<&str> = numbers[0].split_ascii_whitespace().collect();
            winning_numbers.sort();
            let mut my_numbers: Vec<&str> = numbers[1].split_ascii_whitespace().collect();
            my_numbers.sort();

            // Trying to do something that is better than an n*m algorithm. This would be n+m, right?
            let mut wn_iter = 0;
            let mut mn_iter = 0;

            // println!("{:?}", winning_numbers);
            // println!("{:?}", my_numbers);

            let mut score = 0;
            while wn_iter < winning_numbers.len() && mn_iter < my_numbers.len() {
                if winning_numbers[wn_iter] == my_numbers[mn_iter] {
                    if score == 0 {
                        score += 1;
                    } else {
                        score *= 2;
                    }
                    wn_iter += 1;
                    mn_iter += 1;
                    continue;
                }

                if winning_numbers[wn_iter] > my_numbers[mn_iter] {
                    mn_iter += 1;
                } else {
                    wn_iter += 1;
                }
            }
            scratch_cards_value += score;
        }
    }
    println!("{}", scratch_cards_value);
}

pub fn solution_part2() {
        // Get file and make sure it opens correctly
        let file = match File::open("src/day4/input.txt") {
            Ok(file) => file, 
            Err(why) => panic!("Couldn't read file: {}", why),
        };

        let mut card_instances: Vec<i32> = vec![];
        
        for line in BufReader::new(file).lines() {
            if let Ok(card) = line {
                let split: Vec<&str> = card.split(":").collect();
                let title_split: Vec<&str> = split[0].split_ascii_whitespace().collect();
                let card_number: usize = title_split[1].parse().unwrap();
                drop(title_split);

                if card_instances.len() < card_number {
                    card_instances.push(1);
                }

                let numbers: Vec<&str> = split[1].split("|").collect();
    
                let mut winning_numbers: Vec<&str> = numbers[0].split_ascii_whitespace().collect();
                winning_numbers.sort();
                let mut my_numbers: Vec<&str> = numbers[1].split_ascii_whitespace().collect();
                my_numbers.sort();

                let mut wn_iter = 0;
                let mut mn_iter = 0;

                let mut winnings = 0;
                while wn_iter < winning_numbers.len() && mn_iter < my_numbers.len() {
                    if winning_numbers[wn_iter] == my_numbers[mn_iter] {
    
                        winnings += 1;

                        wn_iter += 1;
                        mn_iter += 1;
                        continue;
                    }

                    if winning_numbers[wn_iter] > my_numbers[mn_iter] {
                        mn_iter += 1;
                    } else {
                        wn_iter += 1;
                    }
                }
                
                for i in 0..winnings {
                    if card_number + i >= card_instances.len() {
                        card_instances.push(1);
                    }

                    card_instances[card_number + i] += 1 * card_instances[card_number - 1];
                } 
            }
        }
        let mut sum = 0;
        for card in card_instances {
            sum += card;
        }
        println!("{}", sum);
}