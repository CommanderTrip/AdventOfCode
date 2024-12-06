use crate::utility::file_utility::get_file_lines;

pub fn solution_part1() -> i32 {
    let mut file_reader = get_file_lines("sample.txt", 13);

    let mut first_row: Vec<char> = vec![];
    let mut first_col: Vec<char> = vec![];
    let mut summarize = 0;

    while file_reader.peek().is_some() {
        let line: Vec<char> = file_reader.next().unwrap().unwrap().chars().collect();

        if line.is_empty() || file_reader.peek().is_none() {
            match find_reflection(first_row) {
                Some(v_reflection) => summarize += v_reflection + 1,
                None => match find_reflection(first_col) {
                    Some(h_reflection) => summarize += 100 * (h_reflection + 1),
                    None => {
                        panic!("Failed to find reflection")
                    }
                },
            }

            first_row = vec![];
            first_col = vec![];
            continue;
        }

        if first_row.is_empty() {
            first_row = line.clone();
        }

        first_col.push(line[0].clone());
    }

    return summarize;
}

fn find_reflection(array: Vec<char>) -> Option<i32> {
    println!("{:?}", array);
    let mut end_ptr = array.len() - 1;
    let mut matching = false;

    for (index, char) in array.iter().enumerate() {
        if end_ptr < index {
            match matching {
                true => return Some(end_ptr as i32),
                false => return None,
            }
        }

        if *char == array[end_ptr] {
            end_ptr -= 1;
            matching = true;
        } else {
            end_ptr = array.len() - 1;
            // recheck last itteration
            if *char == array[end_ptr] {
                matching = true;
                end_ptr -= 1;
            } else {
                matching = false;
            }
        }
    }
    return None;
}
