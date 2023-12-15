use std::str::FromStr;

use super::Solution;

pub struct Day15;

impl Solution for Day15 {
    fn test_input() -> String {
        String::from("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7")
    }

    fn solve_part_1(input: String) -> String {
        input
            .split(",")
            .map(|step| hash(step))
            .sum::<usize>()
            .to_string()
    }

    fn solve_part_2(input: String) -> String {
        let mut machine = Machine::new();
        for cmd in input.split(",").map(|word| word.parse().unwrap()) {
            machine.exec(cmd);
        }
        machine.focusing_power().to_string()
    }
}

fn hash(s: &str) -> usize {
    s.chars()
        .map(|ch| ch as u8)
        .fold(0u8, |acc, now| acc.wrapping_add(now).wrapping_mul(17)) as usize
}

struct Machine {
    boxes: Vec<Vec<Lens>>,
}

impl Machine {
    fn new() -> Self {
        Self {
            boxes: (0..256).map(|_| vec![]).collect(),
        }
    }

    fn exec(&mut self, cmd: Command) {
        match cmd {
            Command::Dash(label) => {
                let box_index = hash(&label);
                self.boxes[box_index] = self.boxes[box_index]
                    .drain(..)
                    .filter_map(|lens| if lens.0 == label { None } else { Some(lens) })
                    .collect();
            }
            Command::Equal(label, focal_str) => {
                let box_index = hash(&label);
                let mut seen = false;
                self.boxes[box_index] = self.boxes[box_index]
                    .drain(..)
                    .map(|lens| {
                        if lens.0 == label {
                            seen = true;
                            Lens(lens.0, focal_str)
                        } else {
                            lens
                        }
                    })
                    .collect();
                if !seen {
                    self.boxes[box_index].push(Lens(label, focal_str));
                }
            }
        }
    }

    fn focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .map(|(i, lenses)| {
                lenses
                    .iter()
                    .enumerate()
                    .map(|(j, lens)| (i + 1) * (j + 1) * lens.1)
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
}

#[derive(Debug)]
struct Lens(String, usize);

#[derive(Debug)]
enum Command {
    Dash(String),
    Equal(String, usize),
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with("-") {
            Ok(Self::Dash(s[..s.len() - 1].to_string()))
        } else {
            let mut words = s.split("=");
            Ok(Self::Equal(
                words.next().unwrap().to_string(),
                words.next().unwrap().parse().unwrap(),
            ))
        }
    }
}

#[cfg(test)]
mod day15_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day15::test_input();
        let ans = Day15::solve_part_1(input);
        assert_eq!(ans, "1320");
    }

    #[test]
    fn test_part_2() {
        let input = Day15::test_input();
        let ans = Day15::solve_part_2(input);
        assert_eq!(ans, "145");
    }
}
