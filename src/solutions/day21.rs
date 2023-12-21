use std::{collections::VecDeque, str::FromStr};

use super::Solution;

pub struct Day21;

impl Solution for Day21 {
    fn test_input() -> String {
        String::from(
            "...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........",
        )
    }

    fn solve_part_1(input: String) -> String {
        let map: Map = input.parse().unwrap();

        let ans = map.reachable(64);
        ans.to_string()
    }

    fn solve_part_2(_input: String) -> String {
        String::from("0")
    }
}

type Tile = bool;

const PLOT: Tile = true;
const ROCK: Tile = false;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pos(usize, usize);

struct Map {
    tiles: Vec<Vec<Tile>>,
    start: Pos,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = Pos(0, 0);
        let tiles = s
            .lines()
            .enumerate()
            .map(|(r, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .map(|(c, char)| match char {
                        'S' => {
                            start = Pos(r, c);
                            PLOT
                        }
                        '.' => PLOT,
                        '#' => ROCK,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();
        Ok(Self { tiles, start })
    }
}

impl Map {
    fn next(&self, pos: Pos) -> Vec<Pos> {
        let mut ans = vec![];
        let (max_r, max_c) = (self.tiles.len(), self.tiles[0].len());
        let Pos(r, c) = pos;
        if r > 0 {
            let pos = Pos(r - 1, c);
            if self.tiles[pos.0][pos.1] == PLOT {
                ans.push(pos);
            }
        }
        if c > 0 {
            let pos = Pos(r, c - 1);
            if self.tiles[pos.0][pos.1] == PLOT {
                ans.push(pos);
            }
        }
        if r + 1 < max_r {
            let pos = Pos(r + 1, c);
            if self.tiles[pos.0][pos.1] == PLOT {
                ans.push(pos);
            }
        }
        if c + 1 < max_c {
            let pos = Pos(r, c + 1);
            if self.tiles[pos.0][pos.1] == PLOT {
                ans.push(pos);
            }
        }
        ans
    }
    fn reachable(&self, steps: usize) -> usize {
        let mut ans = 0;
        let start = (self.start, 0);
        let rem = steps % 2;
        let mut queue = VecDeque::from([start]);
        let mut visited: Vec<Vec<bool>> = self
            .tiles
            .iter()
            .map(|row| row.iter().map(|_| false).collect())
            .collect();
        visited[self.start.0][self.start.1] = true;
        while let Some((pos, dist)) = queue.pop_front() {
            if dist % 2 == rem {
                ans += 1;
            }
            if dist == steps {
                continue;
            }
            for n in self.next(pos) {
                if !visited[n.0][n.1] {
                    visited[n.0][n.1] = true;
                    queue.push_back((n, dist + 1));
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod day21_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day21::test_input();
        let map: Map = input.parse().unwrap();

        let ans = map.reachable(6);
        assert_eq!(ans, 16);
    }

    #[test]
    fn test_part_2() {
        let input = Day21::test_input();
        let ans = Day21::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
