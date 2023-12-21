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

    fn solve_part_2(input: String) -> String {
        // 26501365 = 131 * 202300 + 65
        // (-202300, 0) => (0,202300): 4 * 202300 * reachable(65);
        // (-202299, 0) => remaining step: 196 => reachable 130;
        //              => 202299 * 4 * reachable (130);
        // (-202298, 0) => remaining step: 327 => reachable 129;
        //              => 202288 * 4 * reachable (129);
        // ..
        // 1 * reachable(129);

        // 1 * reachable(129)
        // + reachable(129) * 4 * (202298, 202296, .. , 2)
        // + reachable(130) * 4 * (202299, 202297, .. , 1)
        // + reachable(65) * 4 * 202300

        // 1 + 3 + 5 + .. + (2n - 1)
        // 2 * (1 + 2 + .. + n) - n
        // n * (n + 1) - n = n * n

        // 2 + 4 + 6 + .. + 2n
        // n * n + n
        //
        // reachable(129) * (202299)^2
        // + reachable(130) * (202300)^2
        // + reachable(65) * 4 * 202300

        let map: Map = input.parse().unwrap();
        map.max_step().to_string();
        let a = map.reachable(129);
        let b = map.reachable(130);
        let c = map.reachable(65);
        let n = 202300;
        let nsq = n * n;
        let mnsq = (n - 1) * (n - 1);
        let n4 = 4 * n;
        (a * mnsq + b * nsq + c * n4).to_string()
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

impl Map {
    fn max_step(&self) -> usize {
        let mut visited: Vec<Vec<bool>> = self
            .tiles
            .iter()
            .map(|row| row.iter().map(|_| false).collect())
            .collect();
        let mut queue = VecDeque::from([(self.start, 0)]);
        visited[self.start.0][self.start.1] = true;

        let mut ans = 0;

        while let Some((pos, dist)) = queue.pop_front() {
            ans = ans.max(dist);
            for n in self.next(pos) {
                if !visited[n.0][n.1] {
                    visited[n.0][n.1] = true;
                    queue.push_back((n, dist + 1))
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
        let input = String::from(
            ".................................
.....###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#....
.##...####..##...####..##...####.
.##..#...#..##..#...#..##..#...#.
.......##.........##.........##..
.##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##.
.................................
.................................
.....###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#....
.##...####..##..S####..##...####.
.##..#...#..##..#...#..##..#...#.
.......##.........##.........##..
.##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##.
.................................
.................................
.....###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#....
.##...####..##...####..##...####.
.##..#...#..##..#...#..##..#...#.
.......##.........##.........##..
.##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##.
.................................",
        );
        let map: Map = input.parse().unwrap();

        let ans = map.reachable(10);
        assert_eq!(ans, 50);
    }
}
