use std::str::FromStr;

use super::Solution;

pub struct Day23;

use Dir::*;
use Tile::*;

impl Solution for Day23 {
    fn test_input() -> String {
        String::from(
            "#.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#",
        )
    }

    fn solve_part_1(input: String) -> String {
        let map: Map = input.parse().unwrap();
        map.part_1().to_string()
    }

    fn solve_part_2(input: String) -> String {
        let map: Map = input.parse().unwrap();
        String::from("0")
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq, Eq, Debug)]
enum Tile {
    Path,
    Forest,
    Slope(Dir),
}

impl Tile {
    fn from(char: char) -> Self {
        match char {
            '.' => Path,
            '#' => Forest,
            '>' => Slope(East),
            '<' => Slope(West),
            'v' => Slope(South),
            '^' => Slope(North),
            _ => unreachable!(),
        }
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            tiles: s
                .lines()
                .map(|line| line.trim().chars().map(|char| Tile::from(char)).collect())
                .collect(),
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Pos(usize, usize);

impl Map {
    fn size(&self) -> (usize, usize) {
        (self.tiles.len(), self.tiles[0].len())
    }
    fn next(&self, from: Pos) -> Vec<(Pos, usize)> {
        let Pos(r, c) = from;
        let mut ans = vec![];
        let (max_r, max_c) = self.size();
        if r > 0 {
            ans.push(Pos(r - 1, c));
        }
        if c > 0 {
            ans.push(Pos(r, c - 1));
        }
        if r + 1 < max_r {
            ans.push(Pos(r + 1, c));
        }
        if c + 1 < max_c {
            ans.push(Pos(r, c + 1))
        }
        ans.into_iter()
            .filter_map(|pos| {
                let tile = &self.tiles[pos.0][pos.1];
                match tile {
                    Forest => None,
                    Path => Some((pos, 1)),
                    Slope(dir) => {
                        let new_pos = match dir {
                            South => Pos(pos.0 + 1, pos.1),
                            East => Pos(pos.0, pos.1 + 1),
                            North => Pos(pos.0 - 1, pos.1),
                            West => Pos(pos.0, pos.1 - 1),
                        };
                        Some((new_pos, 2))
                    }
                }
            })
            .collect()
    }

    fn part_1(&self) -> usize {
        let mut distances: Vec<Vec<usize>> = self
            .tiles
            .iter()
            .map(|row| row.iter().map(|_| 0).collect())
            .collect();
        let mut visited: Vec<Vec<bool>> = self
            .tiles
            .iter()
            .map(|row| row.iter().map(|_| false).collect())
            .collect();

        let start = Pos(0, 1);
        let (max_r, max_c) = self.size();
        let end = Pos(max_r - 1, max_c - 2);

        let map = &self;

        fn dfs(
            pos: Pos,
            dist: usize,
            map: &Map,
            visited: &mut Vec<Vec<bool>>,
            distances: &mut Vec<Vec<usize>>,
        ) {
            if visited[pos.0][pos.1] {
                return;
            }
            visited[pos.0][pos.1] = true;

            if distances[pos.0][pos.1] < dist {
                distances[pos.0][pos.1] = dist;
            }

            for (next, add_d) in map.next(pos) {
                dfs(next, dist + add_d, map, visited, distances);
            }

            visited[pos.0][pos.1] = false;
        }
        dfs(start, 0, &self, &mut visited, &mut distances);
        distances[end.0][end.1]
    }
}

#[cfg(test)]
mod day23_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day23::test_input();
        let ans = Day23::solve_part_1(input);
        assert_eq!(ans, "94");
    }

    #[test]
    fn test_part_2() {
        let input = Day23::test_input();
        let ans = Day23::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
