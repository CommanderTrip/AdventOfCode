use std::collections::HashMap;

use crate::utility::{custom_math::gcd, file_utility::get_file_lines};

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

impl Node {
    fn new(left: String, right: String) -> Node {
        return Node {
            left: left,
            right: right,
        };
    }
}

pub fn solution_part1() -> i32 {
    let mut file_reader = get_file_lines("input.txt", 8);
    let mut node_map: HashMap<String, Node> = HashMap::new();
    let mut current_node = String::from("AAA");
    let mut moves = 0;

    let binding = file_reader.next().unwrap().unwrap();
    let instructions: Vec<char> = binding.chars().collect(); // Spliting creates blanks on the ends.
    let mut instruction_pointer = 0;

    file_reader.next(); // Skip the next line

    while current_node != String::from("ZZZ") {
        while !node_map.contains_key(&current_node) {
            // Add nodes until we find the rule for the current node
            let line = file_reader.next().unwrap().unwrap();
            let line_split: Vec<&str> = line.split("=").collect();
            let node = line_split[0].trim().to_string();

            let route_trim: &[_] = &['(', ')'];
            let routes: Vec<String> = line_split[1]
                .trim()
                .trim_matches(route_trim)
                .split(", ")
                .map(|route| route.to_string())
                .collect();

            node_map.insert(node, Node::new(routes[0].clone(), routes[1].clone()));
        }

        // Our node is in the map. Move to the next instruction
        current_node = match instructions[instruction_pointer] {
            'L' => node_map.get(&*current_node).unwrap().left.clone(),
            'R' => node_map.get(&*current_node).unwrap().right.clone(),
            _ => panic!(),
        };

        if instruction_pointer + 1 == instructions.len() {
            instruction_pointer = 0;
        } else {
            instruction_pointer += 1;
        }
        moves += 1;
    }

    println!("{}", moves);
    return moves;
}

pub fn solution_part2() -> u128 {
    let mut file_reader = get_file_lines("input.txt", 8);
    let mut node_map: HashMap<String, Node> = HashMap::new();
    let mut current_node: Vec<String> = vec![];
    let mut current_node_moves: Vec<u128> = vec![];

    let binding = file_reader.next().unwrap().unwrap();
    let instructions: Vec<char> = binding.chars().collect(); // Spliting creates blanks on the ends.
    let mut instruction_pointer = 0;

    file_reader.next(); // Skip the next line

    // Create the map
    while file_reader.peek().is_some() {
        let line = file_reader.next().unwrap().unwrap();
        let line_split: Vec<&str> = line.split("=").collect();
        let node = line_split[0].trim().to_string();

        if node.ends_with("A") {
            current_node.push(node.clone());
            current_node_moves.push(0);
        }

        let route_trim: &[_] = &['(', ')'];
        let routes: Vec<String> = line_split[1]
            .trim()
            .trim_matches(route_trim)
            .split(", ")
            .map(|route| route.to_string())
            .collect();

        node_map.insert(node, Node::new(routes[0].clone(), routes[1].clone()));
    }

    // Follow each path until they are ZZZ and track how many moves each one took
    for (index, _) in current_node.clone().iter().enumerate() {
        while !current_node[index].ends_with("Z") {
            current_node[index] = match instructions[instruction_pointer] {
                'L' => {
                    current_node_moves[index] += 1;
                    node_map.get(&*current_node[index]).unwrap().left.clone()
                }
                'R' => {
                    current_node_moves[index] += 1;
                    node_map.get(&*current_node[index]).unwrap().right.clone()
                }
                _ => panic!(),
            };

            if instruction_pointer + 1 == instructions.len() {
                instruction_pointer = 0;
            } else {
                instruction_pointer += 1;
            }
        }
    }

    // Find the LCM for them all
    while current_node_moves.len() != 1 {
        println!("{:?}", current_node_moves);

        let a = current_node_moves.pop().unwrap();
        let b = current_node_moves.pop().unwrap();
        current_node_moves.push(a * b / gcd(a, b));
    }

    let lcm = current_node_moves.pop().unwrap();
    println!("{}", lcm);

    return lcm;
}

#[test]
fn test() {
    assert_eq!(solution_part1(), 19099);
    assert_eq!(solution_part2(), 17099847107071)
}
