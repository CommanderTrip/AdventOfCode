use std::collections::VecDeque;
use std::ops::Div;

use crate::utility::file_utility::get_file_lines;

#[derive(Debug, Clone, PartialEq)]
enum Color {
    Black, // Unvisited
    White, // Visited
    Grey,  // Seen
}

#[derive(Debug, Clone, PartialEq)]
enum Heading {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Node {
    value: char,
    color: Color,
    position: [usize; 2],
    distance: i32,
    heading: Heading,
}

impl Node {
    pub fn new(value: char, column: usize, row: usize) -> Node {
        return Node {
            value: value,
            position: [column, row],
            color: Color::Black,
            distance: i32::MAX,
            heading: Heading::Up,
        };
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.position[0] == other.position[0] && self.position[1] == other.position[1]
    }
}

pub fn solution_part1() -> i32 {
    let mut file_reader = get_file_lines("input.txt", 10);

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

    let mut solution = 0;
    // BFS
    while queue.len() != 0 {
        // Dequeue node and change its color to visited
        let node = queue.pop_front().unwrap();
        solution = node.distance;
        graph[node.position[0]][node.position[1]].color = Color::White;

        // Look around and add unvisited nodes (that follow pipe) to queue, change color to seen, set to self.distance + 1
        if node.position[0] != 0 {
            // Look Left
            match graph[node.position[0] - 1][node.position[1]].value {
                '-' | 'L' | 'F' => {
                    if graph[node.position[0] - 1][node.position[1]].color == Color::Black {
                        graph[node.position[0] - 1][node.position[1]].distance = node.distance + 1;
                        graph[node.position[0] - 1][node.position[1]].color = Color::Grey;
                        graph[node.position[0] - 1][node.position[1]].heading = Heading::Left;
                        queue.push_back(graph[node.position[0] - 1][node.position[1]].clone());
                    }
                }
                _ => { /* Ignore */ }
            }
        }

        if node.position[0] + 1 != graph.len() {
            // Look Right
            match graph[node.position[0] + 1][node.position[1]].value {
                '-' | 'J' | '7' => {
                    if graph[node.position[0] + 1][node.position[1]].color == Color::Black {
                        graph[node.position[0] + 1][node.position[1]].distance = node.distance + 1;
                        graph[node.position[0] + 1][node.position[1]].color = Color::Grey;
                        graph[node.position[0] + 1][node.position[1]].heading = Heading::Right;
                        queue.push_back(graph[node.position[0] + 1][node.position[1]].clone());
                    }
                }
                _ => { /* Ignore */ }
            }
        }

        if node.position[1] != 0 {
            // Look Up
            match graph[node.position[0]][node.position[1] - 1].value {
                '|' | 'F' | '7' => {
                    if graph[node.position[0]][node.position[1] - 1].color == Color::Black {
                        graph[node.position[0]][node.position[1] - 1].distance = node.distance + 1;
                        graph[node.position[0]][node.position[1] - 1].color = Color::Grey;
                        graph[node.position[0]][node.position[1] - 1].heading = Heading::Up;
                        queue.push_back(graph[node.position[0]][node.position[1] - 1].clone());
                    }
                }
                _ => { /* Ignore */ }
            }
        }

        if node.position[1] + 1 != graph[0].len() {
            // Look Down
            match graph[node.position[0]][node.position[1] + 1].value {
                '|' | 'J' | 'L' => {
                    if graph[node.position[0]][node.position[1] + 1].color == Color::Black {
                        graph[node.position[0]][node.position[1] + 1].distance = node.distance + 1;
                        graph[node.position[0]][node.position[1] + 1].color = Color::Grey;
                        graph[node.position[0]][node.position[1] + 1].heading = Heading::Down;
                        queue.push_back(graph[node.position[0]][node.position[1] + 1].clone());
                    }
                }
                _ => { /* Ignore */ }
            }
        }

        // loop
    }

    return solution;
}

pub fn solution_part2() -> i32 {
    let mut file_reader = get_file_lines("input.txt", 10);

    let mut graph: Vec<Vec<Node>> = vec![]; // [column][row]
    let mut stack: VecDeque<Node> = VecDeque::new();
    let mut loop_vec: Vec<Node> = vec![];

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
                stack.push_back(node);
            }

            column += 1;
        }
        row += 1;
    }

    // DFS to traverse the loop and save every tile on the loop
    while stack.len() != 0 {
        // Dequeue node and change its color to visited
        let node = stack.pop_back().unwrap();
        loop_vec.push(node.clone());
        graph[node.position[0]][node.position[1]].color = Color::White;

        // Look around and add unvisited nodes (that follow pipe) to queue, change color to seen, set to self.distance + 1
        if node.position[0] != 0
            && (node.value == '-' || node.value == 'J' || node.value == '7' || node.value == 'S')
        {
            // Look Left
            match graph[node.position[0] - 1][node.position[1]].value {
                '-' | 'L' | 'F' => {
                    if graph[node.position[0] - 1][node.position[1]].color == Color::Black {
                        graph[node.position[0] - 1][node.position[1]].distance = node.distance + 1;
                        graph[node.position[0] - 1][node.position[1]].color = Color::Grey;
                        graph[node.position[0] - 1][node.position[1]].heading = Heading::Left;
                        stack.push_back(graph[node.position[0] - 1][node.position[1]].clone());
                        continue;
                    }
                }
                _ => { /* Ignore */ }
            }
        }

        if node.position[1] + 1 != graph[0].len()
            && (node.value == '|' || node.value == '7' || node.value == 'F' || node.value == 'S')
        {
            // Look Down
            match graph[node.position[0]][node.position[1] + 1].value {
                '|' | 'J' | 'L' => {
                    if graph[node.position[0]][node.position[1] + 1].color == Color::Black {
                        graph[node.position[0]][node.position[1] + 1].distance = node.distance + 1;
                        graph[node.position[0]][node.position[1] + 1].color = Color::Grey;
                        graph[node.position[0]][node.position[1] + 1].heading = Heading::Down;
                        stack.push_back(graph[node.position[0]][node.position[1] + 1].clone());
                        continue;
                    }
                }
                _ => { /* Ignore */ }
            }
        }

        if node.position[0] + 1 != graph.len()
            && (node.value == '-' || node.value == 'F' || node.value == 'L' || node.value == 'S')
        {
            // Look Right
            match graph[node.position[0] + 1][node.position[1]].value {
                '-' | 'J' | '7' => {
                    if graph[node.position[0] + 1][node.position[1]].color == Color::Black {
                        graph[node.position[0] + 1][node.position[1]].distance = node.distance + 1;
                        graph[node.position[0] + 1][node.position[1]].color = Color::Grey;
                        graph[node.position[0] + 1][node.position[1]].heading = Heading::Right;
                        stack.push_back(graph[node.position[0] + 1][node.position[1]].clone());
                        continue;
                    }
                }
                _ => { /* Ignore */ }
            }
        }

        if node.position[1] != 0
            && (node.value == '|' || node.value == 'J' || node.value == 'L' || node.value == 'S')
        {
            // Look Up
            match graph[node.position[0]][node.position[1] - 1].value {
                '|' | 'F' | '7' => {
                    if graph[node.position[0]][node.position[1] - 1].color == Color::Black {
                        graph[node.position[0]][node.position[1] - 1].distance = node.distance + 1;
                        graph[node.position[0]][node.position[1] - 1].color = Color::Grey;
                        graph[node.position[0]][node.position[1] - 1].heading = Heading::Up;
                        stack.push_back(graph[node.position[0]][node.position[1] - 1].clone());
                        continue;
                    }
                }
                _ => { /* Ignore */ }
            }
        }

        // loop
    }

    // Shoelace formula
    // A = (1/2) * Sum{(y + y')(x - x')}
    let area = loop_vec
        .iter()
        .enumerate()
        .fold(0, |acc, (i, node)| {
            let l = (i + 1) % loop_vec.len();
            acc + (node.position[1] + loop_vec[l].position[1]) as i32
                * (node.position[0] as i32 - loop_vec[l].position[0] as i32)
        })
        .abs()
        .div(2);

    // Picks formula
    // A = i + (b / 2) - 1
    let solution = area + 1 - (loop_vec.len() / 2) as i32;

    return solution;
}
