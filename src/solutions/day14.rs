use std::{collections::HashMap, hash::Hash, str::FromStr};

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
        // println!("{}", platform.to_string());
        platform.total_load().to_string()
    }

    fn solve_part_2(input: String) -> String {
        let mut pattern: HashMap<Platform, usize> = HashMap::new();
        let mut platform: Platform = input.parse().unwrap();

        let max_cycle = 1_000_000_000;

        for i in 1..=max_cycle {
            platform.cycle();
            if let Some(n) = pattern.get(&platform) {
                let cycle = i - n;
                let rem = (max_cycle - n) % cycle;

                let platform_at_rem = pattern.iter().find(|(_, v)| **v == n + rem).unwrap().0;

                return platform_at_rem.total_load().to_string();
            } else {
                pattern.insert(platform.clone(), i);
            }
        }
        String::from("0")
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
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

#[derive(Debug, Clone)]
struct Platform {
    rocks: Vec<Vec<Rock>>,
}

impl Hash for Platform {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_string().hash(state)
    }
}

impl PartialEq for Platform {
    fn eq(&self, other: &Self) -> bool {
        self.rocks == other.rocks
    }
}
impl Eq for Platform {}

impl ToString for Platform {
    fn to_string(&self) -> String {
        self.rocks
            .iter()
            .map(|row| {
                row.iter()
                    .map(|rock| match rock {
                        Rock::Cube => '#',
                        Rock::Rounded => 'O',
                        Rock::Empty => '.',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
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
    fn size(&self) -> (usize, usize) {
        (self.rocks.len(), self.rocks[0].len())
    }
    fn cycle(&mut self) {
        // nwse
        self.tilt_to_north();
        self.tilt_to_west();
        self.tilt_to_south();
        self.tilt_to_east();
    }

    fn tilt_to_north(&mut self) {
        let (max_r, max_c) = self.size();
        for r in 1..max_r {
            for c in 0..max_c {
                let rock = &self.rocks[r][c];
                if let Rock::Rounded = rock {
                    for north in (0..r).rev() {
                        let rock_north = &self.rocks[north][c];
                        if let Rock::Empty = rock_north {
                            self.rocks[north][c] = Rock::Rounded;
                            self.rocks[north + 1][c] = Rock::Empty;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
    fn tilt_to_south(&mut self) {
        let (max_r, max_c) = self.size();
        for r in (0..max_r - 1).rev() {
            for c in 0..max_c {
                let rock = &self.rocks[r][c];
                if let Rock::Rounded = rock {
                    for south in (r + 1)..max_r {
                        let rock_south = &self.rocks[south][c];
                        if let Rock::Empty = rock_south {
                            self.rocks[south][c] = Rock::Rounded;
                            self.rocks[south - 1][c] = Rock::Empty;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
    fn tilt_to_west(&mut self) {
        let (max_r, max_c) = self.size();
        for c in 1..max_c {
            for r in 0..max_r {
                let rock = &self.rocks[r][c];
                if let Rock::Rounded = rock {
                    for west in (0..c).rev() {
                        let rock_west = &self.rocks[r][west];
                        if let Rock::Empty = rock_west {
                            self.rocks[r][west] = Rock::Rounded;
                            self.rocks[r][west + 1] = Rock::Empty;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    fn tilt_to_east(&mut self) {
        let (max_r, max_c) = self.size();
        for c in (0..max_c - 1).rev() {
            for r in 0..max_r {
                let rock = &self.rocks[r][c];
                if let Rock::Rounded = rock {
                    for east in c + 1..max_c {
                        let rock_south = &self.rocks[r][east];
                        if let Rock::Empty = rock_south {
                            self.rocks[r][east] = Rock::Rounded;
                            self.rocks[r][east - 1] = Rock::Empty;
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
            .map(|line| line.trim().chars().map(|ch| Rock::from(ch)).collect())
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
        assert_eq!(ans, "64");
    }

    #[test]
    fn test_cycles() {
        let input = Day14::test_input();
        let mut platform: Platform = input.parse().unwrap();
        platform.cycle();
        let ans: Platform = ".....#....
        ....#...O#
        ...OO##...
        .OO#......
        .....OOO#.
        .O#...O#.#
        ....O#....
        ......OOOO
        #...O###..
        #..OO#...."
            .parse()
            .unwrap();

        assert_eq!(platform, ans);
        platform.cycle();
        let ans: Platform = ".....#....
        ....#...O#
        .....##...
        ..O#......
        .....OOO#.
        .O#...O#.#
        ....O#...O
        .......OOO
        #..OO###..
        #.OOO#...O"
            .parse()
            .unwrap();
        assert_eq!(platform, ans);
    }
}
