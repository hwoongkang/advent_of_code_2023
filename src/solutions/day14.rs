use std::str::FromStr;

use super::Solution;

pub struct Day14;

impl Solution for Day14 {
    fn test_input() -> String {
        String::from(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        )
    }

    fn solve_part_1(input: String) -> String {
        let mut platform: Platform = input.parse().unwrap();
        platform.tilt_to_north();
        // platform.print();
        platform.total_load().to_string()
    }

    fn solve_part_2(_input: String) -> String {
        String::from("0")
    }
}

enum Rock {
    Empty,
    Rounded,
    Cube,
}

impl Rock {
    fn from(char: char) -> Self {
        match char {
            '.' => Self::Empty,
            '#' => Self::Cube,
            'O' => Self::Rounded,
            _ => unreachable!(),
        }
    }
}

struct Platform {
    rocks: Vec<Vec<Rock>>,
}

impl Platform {
    fn total_load(&self) -> usize {
        let mut ans = 0;
        let (max_r, max_c) = self.size();
        for r in 0..max_r {
            for c in 0..max_c {
                if let Rock::Rounded = &self.rocks[r][c] {
                    ans += max_r - r;
                }
            }
        }
        ans
    }
    fn print(&self) {
        for row in self.rocks.iter() {
            for rock in row.iter() {
                print!(
                    "{}",
                    match rock {
                        Rock::Empty => '.',
                        Rock::Rounded => 'O',
                        Rock::Cube => '#',
                    }
                );
            }
            println!("");
        }
    }
    fn size(&self) -> (usize, usize) {
        (self.rocks.len(), self.rocks[0].len())
    }
    fn tilt_to_north(&mut self) {
        let (max_r, max_c) = self.size();
        for r in 1..max_r {
            for c in 0..max_c {
                let rock = &self.rocks[r][c];
                if let Rock::Rounded = rock {
                    for prev in (0..r).rev() {
                        let rock_on_north = &self.rocks[prev][c];
                        if let Rock::Empty = rock_on_north {
                            self.rocks[prev][c] = Rock::Rounded;
                            self.rocks[prev + 1][c] = Rock::Empty;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
}

impl FromStr for Platform {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rocks = s
            .lines()
            .map(|line| line.chars().map(|ch| Rock::from(ch)).collect())
            .collect();
        Ok(Self { rocks })
    }
}

#[cfg(test)]
mod day14_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day14::test_input();
        let ans = Day14::solve_part_1(input);
        assert_eq!(ans, "136");
    }

    #[test]
    fn test_part_2() {
        let input = Day14::test_input();
        let ans = Day14::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
