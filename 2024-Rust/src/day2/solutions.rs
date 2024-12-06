use crate::utility::file_utility::get_file_lines;

pub fn part1(file_name: &str) -> i32 {
    let mut file_reader = get_file_lines(file_name, 2);
    let mut safe_reports = 0;

    while file_reader.peek().is_some() {
        let line = file_reader.next().unwrap().unwrap();
        let split = line.split(" ");
        println!("{:?}", split);
    }

    return 0;
}

// pub fn part2(file_name: &str) -> i32 {
//     let mut file_reader = get_file_lines(file_name, 2);

//     return 0;
// }

// #[test]
// fn part1_solution() {
//     assert_eq!(part1("sample.txt"), 0);
// }

// #[test]
// fn part2_solution() {
//     assert_eq!(part2("sample.txt"), 0);
// }
