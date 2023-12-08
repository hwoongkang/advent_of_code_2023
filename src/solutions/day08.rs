use std::collections::HashMap;

use super::Solution;

pub struct Day08;

impl Solution for Day08 {
    fn test_input() -> String {
        String::from(
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        )
    }

    fn solve_part_1(input: String) -> String {
        let lines = &mut input.lines();
        let instructions: Vec<char> = lines.next().unwrap().chars().collect();
        lines.next();
        let mut graph = Graph::from(lines);
        graph.find_zzz(&instructions).to_string()
    }

    fn solve_part_2(input: String) -> String {
        let lines = &mut input.lines();
        let instructions: Vec<char> = lines.next().unwrap().chars().collect();
        lines.next();
        let mut graph = Graph::from(lines);
        let nodes: Vec<String> = graph
            .nodes
            .keys()
            .filter_map(|k| {
                if k.ends_with("A") {
                    Some(k.clone())
                } else {
                    None
                }
            })
            .collect();
        nodes
            .into_iter()
            .map(|node| {
                graph.head = node;
                graph.find_z(&instructions)
            })
            .fold(1, |a, b| lcm(a, b))
            .to_string()
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b < a {
        gcd(b, a)
    } else {
        if b % a == 0 {
            a
        } else {
            gcd(b % a, a)
        }
    }
}

fn lcm(a: usize, b: usize) -> usize {
    let d = gcd(a, b);
    a * b / d
}

fn parse_node_line(line: &str) -> (String, String, String) {
    //AAA = (BBB, CCC)
    let mut line = line.split("=");
    let first = line.next().unwrap().trim().to_string();
    let mut line = line.next().unwrap().trim().split(",");
    let second = line
        .next()
        .unwrap()
        .trim()
        .trim_start_matches('(')
        .to_string();
    let third = line
        .next()
        .unwrap()
        .trim()
        .trim_end_matches(')')
        .to_string();
    (first, second, third)
}

struct Graph {
    head: String,
    nodes: HashMap<String, Node>,
}

impl Graph {
    fn find_z(&mut self, instructions: &[char]) -> usize {
        let l = instructions.len();
        let mut i = 0;
        loop {
            let j = i % l;
            let instruction = instructions[j];
            self.handle_instruction(instruction);
            if self.head.ends_with("Z") {
                break i + 1;
            }
            i += 1;
        }
    }
    fn find_zzz(&mut self, instructions: &[char]) -> usize {
        let l = instructions.len();
        let mut i = 0;
        loop {
            let j = i % l;
            let instruction = instructions[j];
            self.handle_instruction(instruction);
            if self.head == "ZZZ" {
                break i + 1;
            }
            i += 1;
        }
    }
    fn handle_instruction(&mut self, instruction: char) {
        match instruction {
            'L' => {
                self.head = self.nodes.get(&self.head).unwrap().left.clone();
            }
            'R' => {
                self.head = self.nodes.get(&self.head).unwrap().right.clone();
            }
            _ => unreachable!(),
        }
    }
    fn from(lines: &mut std::str::Lines) -> Self {
        let mut nodes = HashMap::new();
        for line in lines {
            let (head, left, right) = parse_node_line(line);
            nodes.insert(head, Node { left, right });
        }
        Self {
            head: "AAA".to_string(),
            nodes,
        }
    }
}

#[derive(Clone)]
struct Node {
    left: String,
    right: String,
}

#[cfg(test)]
mod day08_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day08::test_input();
        let ans = Day08::solve_part_1(input);
        assert_eq!(ans, "2");
    }

    #[test]
    fn test_part_2() {
        let input = String::from(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        let ans = Day08::solve_part_2(input);
        assert_eq!(ans, "6");
    }
}
