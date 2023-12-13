use std::str::FromStr;

use super::Solution;

pub struct Day13;

impl Solution for Day13 {
    fn test_input() -> String {
        String::from(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        )
    }

    fn solve_part_1(input: String) -> String {
        let mirrors: Vec<Mirror> = input
            .split("\n\n")
            .map(|str| str.parse().unwrap())
            .collect();
        mirrors
            .iter()
            .map(|mirror| {
                let mut ans = 0;
                if let Some(r) = mirror.row_symmetry() {
                    ans += 100 * r;
                }
                if let Some(c) = mirror.col_symmetry() {
                    ans += c;
                }
                ans
            })
            .sum::<usize>()
            .to_string()
    }

    fn solve_part_2(input: String) -> String {
        let mirrors: Vec<Mirror> = input
            .split("\n\n")
            .map(|str| str.parse().unwrap())
            .collect();
        mirrors
            .iter()
            .map(|mirror| {
                let mut ans = 0;
                if let Some(r) = mirror.row_smudged_symmetry() {
                    ans += 100 * r;
                }
                if let Some(c) = mirror.col_smudged_symmetry() {
                    ans += c;
                }
                ans
            })
            .sum::<usize>()
            .to_string()
    }
}

#[derive(Debug)]
struct Mirror {
    chars: Vec<Vec<char>>,
}

impl Mirror {
    fn transposed(&self) -> Mirror {
        Self {
            chars: (0..self.chars[0].len())
                .map(|c| self.chars.iter().map(|row| row[c]).collect())
                .collect(),
        }
    }

    fn row_smudged_symmetry(&self) -> Option<usize> {
        for r in 1..self.chars.len() {
            let mut smudges = 0;
            for (row1, row2) in self.chars[r..].iter().zip(self.chars[0..r].iter().rev()) {
                for (c1, c2) in row1.iter().zip(row2.iter()) {
                    if c1 != c2 {
                        smudges += 1;
                    }
                }
            }
            if smudges == 1 {
                return Some(r);
            }
        }
        None
    }

    fn col_smudged_symmetry(&self) -> Option<usize> {
        self.transposed().row_smudged_symmetry()
    }

    fn row_symmetry(&self) -> Option<usize> {
        for r in 1..self.chars.len() {
            let mut symm = true;
            for (row1, row2) in self.chars[r..].iter().zip(self.chars[0..r].iter().rev()) {
                if row1 != row2 {
                    symm = false;
                    break;
                }
            }
            if symm {
                return Some(r);
            }
        }
        None
    }

    fn col_symmetry(&self) -> Option<usize> {
        self.transposed().row_symmetry()
    }
}

impl FromStr for Mirror {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.lines().map(|line| line.chars().collect()).collect();
        Ok(Self { chars })
    }
}

#[cfg(test)]
mod day13_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day13::test_input();
        let ans = Day13::solve_part_1(input);
        assert_eq!(ans, "405");
    }

    #[test]
    fn test_part_2() {
        let input = Day13::test_input();
        let ans = Day13::solve_part_2(input);
        assert_eq!(ans, "400");
    }
}
