use std::{collections::HashMap, str::FromStr};

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
        map.part_2().to_string()
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Pos(usize, usize);

impl Map {
    fn size(&self) -> (usize, usize) {
        (self.tiles.len(), self.tiles[0].len())
    }
    fn apply_slope(&self, candidates: Vec<(Pos, usize)>) -> Vec<(Pos, usize)> {
        candidates
            .into_iter()
            .filter_map(|(pos, dist)| {
                let tile = &self.tiles[pos.0][pos.1];
                match tile {
                    Forest => unreachable!(),
                    Path => Some((pos, dist)),
                    Slope(dir) => {
                        let new_pos = match dir {
                            South => Pos(pos.0 + 1, pos.1),
                            East => Pos(pos.0, pos.1 + 1),
                            North => Pos(pos.0 - 1, pos.1),
                            West => Pos(pos.0, pos.1 - 1),
                        };
                        Some((new_pos, dist + 1))
                    }
                }
            })
            .collect()
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
                    _ => Some((pos, 1)),
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

            for (next, add_d) in map.apply_slope(map.next(pos)) {
                dfs(next, dist + add_d, map, visited, distances);
            }

            visited[pos.0][pos.1] = false;
        }
        dfs(start, 0, &self, &mut visited, &mut distances);
        distances[end.0][end.1]
    }

    fn part_2(&self) -> usize {
        let mut nodes: HashMap<Pos, Node> = HashMap::new();

        fn add_node(pos: Pos, map: &Map, nodes: &mut HashMap<Pos, Node>) {
            let mut visited: Vec<Vec<bool>> = map
                .tiles
                .iter()
                .map(|row| row.iter().map(|_| false).collect())
                .collect();
            visited[pos.0][pos.1] = true;
            let mut stack = vec![(pos, 0)];

            let me = Node {
                pos,
                neighbors: vec![],
            };

            let original = pos;

            nodes.insert(me.pos, me);

            while let Some((pos, dist)) = stack.pop() {
                let next = map.next(pos);
                let is_branch = next.len() != 2;
                if pos == original && dist > 0 {
                    panic!("cannot solve with this");
                }
                if is_branch && dist > 0 {
                    nodes
                        .get_mut(&original)
                        .unwrap()
                        .neighbors
                        .push((pos, dist));
                    visited[pos.0][pos.1] = true;
                    if nodes.get(&pos).is_none() {
                        add_node(pos, map, nodes);
                    }
                } else {
                    for (next_pos, d) in next {
                        if !visited[next_pos.0][next_pos.1] {
                            visited[next_pos.0][next_pos.1] = true;
                            stack.push((next_pos, dist + d));
                        }
                    }
                }
            }
        }

        add_node(Pos(0, 1), &self, &mut nodes);

        let start = Pos(0, 1);
        let (max_r, max_c) = self.size();
        let end = Pos(max_r - 1, max_c - 2);

        let mut visited: HashMap<Pos, bool> = nodes.keys().map(|pos| (*pos, false)).collect();
        let mut distances: HashMap<Pos, usize> = nodes.keys().map(|pos| (*pos, 0)).collect();

        println!("nodes: {}", nodes.len());

        fn dfs(
            pos: Pos,
            dist: usize,

            nodes: &HashMap<Pos, Node>,
            visited: &mut HashMap<Pos, bool>,
            distances: &mut HashMap<Pos, usize>,
        ) {
            if *visited.get(&pos).unwrap() {
                return;
            }
            *visited.get_mut(&pos).unwrap() = true;

            if distances.get(&pos).unwrap() < &dist {
                *distances.get_mut(&pos).unwrap() = dist;
            }

            for (next, add_d) in nodes.get(&pos).unwrap().neighbors.iter() {
                dfs(*next, dist + add_d, nodes, visited, distances);
            }

            *visited.get_mut(&pos).unwrap() = false;
        }
        dfs(start, 0, &nodes, &mut visited, &mut distances);
        *distances.get(&end).unwrap()
    }
}

#[derive(Debug)]
struct Node {
    pos: Pos,
    neighbors: Vec<(Pos, usize)>,
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
        assert_eq!(ans, "154");
    }
}
