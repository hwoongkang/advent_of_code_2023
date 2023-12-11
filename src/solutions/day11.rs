use std::{collections::HashSet, str::FromStr};

use super::Solution;

pub struct Day11;

impl Solution for Day11 {
    fn test_input() -> String {
        String::from(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        )
    }

    fn solve_part_1(input: String) -> String {
        let galaxy = Galaxy::from_str(&input).unwrap();
        galaxy.dist(2, 1).to_string()
    }

    fn solve_part_2(input: String) -> String {
        let galaxy = Galaxy::from_str(&input).unwrap();
        galaxy.dist(1_000_000, 1).to_string()
    }
}

#[derive(Copy, Clone, Debug)]
struct Star(usize, usize);

#[derive(Debug)]
struct Galaxy {
    stars: Vec<Star>,
    empty_rows: HashSet<usize>,
    empty_cols: HashSet<usize>,
}

impl Galaxy {
    fn dist_btw(&self, from: Star, to: Star, expansion_rate: usize, num_expansion: u32) -> usize {
        let Star(sr, sc) = from;
        let Star(er, ec) = to;

        let (sr, er) = if sr < er { (sr, er) } else { (er, sr) };
        let (sc, ec) = if sc < ec { (sc, ec) } else { (ec, sc) };

        let mut count = 0;

        for r in sr..er {
            if let Some(_) = self.empty_rows.get(&r) {
                count += expansion_rate.pow(num_expansion);
            } else {
                count += 1;
            }
        }
        for c in sc..ec {
            if let Some(_) = self.empty_cols.get(&c) {
                count += expansion_rate.pow(num_expansion);
            } else {
                count += 1;
            }
        }

        count
    }
    fn dist(&self, expansion_rate: usize, num_expansion: u32) -> usize {
        let mut count = 0;
        let num_stars = self.stars.len();

        for i in 0..num_stars {
            let from = self.stars[i];
            for j in (i + 1)..num_stars {
                let to = self.stars[j];
                count += self.dist_btw(from, to, expansion_rate, num_expansion);
            }
        }

        count
    }
}

impl FromStr for Galaxy {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stars = vec![];
        let mut empty_rows = HashSet::new();
        let mut empty_cols = HashSet::new();

        let chars: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();

        let size = Star(chars.len(), chars[1].len());
        for r in 0..size.0 {
            let mut had_star = false;
            for c in 0..size.1 {
                if chars[r][c] == '#' {
                    stars.push(Star(r, c));
                    had_star = true;
                }
            }
            if !had_star {
                empty_rows.insert(r);
            }
        }
        for c in 0..size.1 {
            let mut had_star = false;
            for r in 0..size.0 {
                if chars[r][c] == '#' {
                    had_star = true;
                }
            }
            if !had_star {
                empty_cols.insert(c);
            }
        }

        Ok(Self {
            stars,
            empty_cols,
            empty_rows,
        })
    }
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day11::test_input();
        let ans = Day11::solve_part_1(input);
        assert_eq!(ans, "374");
    }

    #[test]
    fn test_part_2() {
        let input = Day11::test_input();
        let galaxy = Galaxy::from_str(&input).unwrap();

        assert_eq!(galaxy.dist(10, 1).to_string(), "1030");
    }
}
