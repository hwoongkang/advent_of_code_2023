use std::{collections::BinaryHeap, str::FromStr};

use super::Solution;

pub struct Day17;

impl Solution for Day17 {
    fn test_input() -> String {
        String::from(
            "2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533",
        )
    }

    fn solve_part_1(input: String) -> String {
        let map: Map = input.parse().unwrap();
        map.minimize().to_string()
    }

    fn solve_part_2(input: String) -> String {
        String::from("0")
    }
}

#[derive(Debug)]
struct Map {
    heat_loss: Vec<Vec<usize>>,
}

impl Map {
    fn size(&self) -> Pos {
        Pos(self.heat_loss.len(), self.heat_loss[0].len())
    }
    fn minimize(&self) -> usize {
        let dest = self.size();
        let mut heap: BinaryHeap<State> = BinaryHeap::new();
        let mut visited: Vec<Vec<Vec<usize>>> = self
            .heat_loss
            .iter()
            .map(|row| row.iter().map(|_| vec![usize::MAX; 4]).collect())
            .collect();
        heap.push(State {
            pos: Pos(0, 0),
            loss: 0,
            dir: Dir::Down,
        });
        heap.push(State {
            pos: Pos(0, 0),
            loss: 0,
            dir: Dir::Right,
        });
        for state in heap.iter() {
            let (x, y, z) = state.coord();
            visited[x][y][z] = 0;
        }
        let dest = self.size();

        while let Some(state) = heap.pop() {
            for next_state in self.next(&state) {
                let (x, y, z) = next_state.coord();

                if next_state.loss < visited[x][y][z] {
                    visited[x][y][z] = next_state.loss;
                    heap.push(next_state);
                }
            }
        }

        *visited[dest.0 - 1][dest.1 - 1].iter().min().unwrap()
    }
    fn next(&self, state: &State) -> Vec<State> {
        match state.dir {
            Dir::Down | Dir::Up => self
                .next_in(state, Dir::Left)
                .into_iter()
                .chain(self.next_in(state, Dir::Right).into_iter())
                .collect(),

            Dir::Left | Dir::Right => self
                .next_in(state, Dir::Up)
                .into_iter()
                .chain(self.next_in(state, Dir::Down).into_iter())
                .collect(),
        }
    }

    fn next_in(&self, state: &State, dir: Dir) -> Vec<State> {
        let Pos(r, c) = state.pos;
        let Pos(max_r, max_c) = self.size();
        match dir {
            Dir::Down => (1..=3)
                .filter_map(|i| {
                    let pos = Pos(r + i, c);
                    if pos.0 >= max_r {
                        None
                    } else {
                        let loss =
                            state.loss + (1..=i).map(|j| self.heat_loss[r + j][c]).sum::<usize>();
                        Some(State { pos, loss, dir })
                    }
                })
                .collect(),
            Dir::Up => (1..=3)
                .filter_map(|i| {
                    if i > r {
                        None
                    } else {
                        let pos = Pos(r - i, c);
                        let loss =
                            state.loss + (1..=i).map(|j| self.heat_loss[r - j][c]).sum::<usize>();
                        Some(State { pos, loss, dir })
                    }
                })
                .collect(),
            Dir::Left => (1..=3)
                .filter_map(|i| {
                    if i > c {
                        None
                    } else {
                        let pos = Pos(r, c - i);
                        let loss =
                            state.loss + (1..=i).map(|j| self.heat_loss[r][c - j]).sum::<usize>();
                        Some(State { pos, loss, dir })
                    }
                })
                .collect(),
            Dir::Right => (1..=3)
                .filter_map(|i| {
                    let pos = Pos(r, c + i);
                    if pos.1 >= max_c {
                        None
                    } else {
                        let loss =
                            state.loss + (1..=i).map(|j| self.heat_loss[r][c + j]).sum::<usize>();
                        Some(State { pos, loss, dir })
                    }
                })
                .collect(),
        }
    }
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            heat_loss: s
                .lines()
                .map(|line| {
                    line.trim()
                        .chars()
                        .map(|char| char.to_digit(10).unwrap() as usize)
                        .collect()
                })
                .collect(),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Pos(usize, usize);

#[derive(Debug, PartialEq, Eq)]
struct State {
    pos: Pos,
    loss: usize,
    dir: Dir,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.loss.partial_cmp(&self.loss)
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.loss.cmp(&self.loss)
    }
}

impl State {
    fn coord(&self) -> (usize, usize, usize) {
        (self.pos.0, self.pos.1, self.dir as usize)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[cfg(test)]
mod day17_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day17::test_input();
        let ans = Day17::solve_part_1(input);
        assert_eq!(ans, "102");
    }

    #[test]
    fn test_part_2() {
        let input = Day17::test_input();
        let ans = Day17::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
