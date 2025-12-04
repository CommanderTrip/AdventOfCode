use crate::utilities::file_reader::get_reader;

pub fn part1() -> i32 {
    let mut dial = 50;
    let mut password = 0;
    let mut reader = get_reader("src/day1/input.txt");

    println!("Dial is at {:?}", dial);
    while reader.peek().is_some() {
        let line = reader.next().unwrap().unwrap();
        let split = line.as_str().split_at(1);

        let mut turn = split.1.parse::<i32>().unwrap();
        while turn > 99 {
            turn -= 100;
        }

        match split.0 {
            "L" => {
                dial -= turn;
                if dial < 0 {
                    dial += 100;
                }
            }
            "R" => {
                dial += turn;
                if dial > 99 {
                    dial -= 100;
                }
            }
            _ => panic!("Split Error"),
        }
        if dial == 0 {
            password += 1;
        }
        println!("Dial is at {:?}", dial);
    }

    password
}

pub fn part2() -> i32 {
    let mut dial = 50;
    let mut password = 0;
    let mut reader = get_reader("src/day1/input.txt");

    println!("Dial is at {:?}", dial);
    while reader.peek().is_some() {
        let line = reader.next().unwrap().unwrap();
        let split = line.as_str().split_at(1);

        let mut turn = split.1.parse::<i32>().unwrap();
        while turn > 99 {
            turn -= 100;
            password += 1;
        }

        match split.0 {
            "L" => {
                let prev_dial = dial;
                dial -= turn;
                if dial < 0 {
                    dial += 100;

                    if prev_dial != 0 {
                        password += 1;
                    }
                }
            }
            "R" => {
                let prev_dial = dial;
                dial += turn;
                if dial > 99 {
                    dial -= 100;

                    if dial != 0 {
                        password += 1;
                    }
                }
            }
            _ => panic!("Split Error"),
        }
        if dial == 0 {
            password += 1;
        }

        println!(
            "Action is {:?}\t Dial is at {:?}\t Password is {:?}",
            line, dial, password
        );
    }

    password
}
