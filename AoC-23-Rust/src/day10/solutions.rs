use std::collections::VecDeque;

use crate::utility::file_utility::get_file_lines;

#[derive(Debug, Clone)]
enum Color {
    Black, // Unvisited
    White, // Visited
    Grey,  // Seen
}

#[derive(Debug, Clone)]
struct Node {
    value: char,
    color: Color,
    position: [usize; 2],
    distance: i32,
}

impl Node {
    pub fn new(value: char, column: usize, row: usize) -> Node {
        return Node {
            value: value,
            position: [column, row],
            color: Color::Black,
            distance: i32::MAX,
        };
    }
}

pub fn solution_part1() -> i32 {
    let mut file_reader = get_file_lines("sample.txt", 10);

    let mut graph: Vec<Vec<Node>> = vec![]; // [column][row]
    let mut queue: VecDeque<Node> = VecDeque::new();

    // Build graph
    let mut row = 0;
    while file_reader.peek().is_some() {
        let mut column = 0;
        let line: Vec<char> = file_reader.next().unwrap().unwrap().chars().collect();
        for char in line {
            if column >= graph.len() {
                graph.push(vec![]);
            }
            graph[column].push(Node::new(char, column, row));

            if char == 'S' {
                graph[column][row].color = Color::Grey;
                graph[column][row].distance = 0;
                let mut node = graph[column][row].clone();
                node.color = Color::Grey;
                queue.push_back(node);
            }

            column += 1;
        }
        row += 1;
    }

    // BFS
    while queue.len() != 0 {
        // Dequeue node and change its color to visited
        // Look around and add unvisited nodes (that follow pipe) to queue, change color to seen, set to self.distance + 1
        // loop
    }

    return 1;
}
