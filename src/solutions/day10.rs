use std::{collections::VecDeque, str::FromStr};

use super::Solution;

pub struct Day10;

impl Solution for Day10 {
    fn test_input() -> String {
        String::from(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        )
    }

    fn solve_part_1(input: String) -> String {
        let diagram = Diagram::from_str(&input).unwrap();

        diagram.part_1().to_string()
    }

    fn solve_part_2(input: String) -> String {
        let diagram = Diagram::from_str(&input).unwrap();

        diagram.part_2().to_string()
    }
}

/*
| is a vertical pipe connecting north and south.
- is a horizontal pipe connecting east and west.
L is a 90-degree bend connecting north and east.
J is a 90-degree bend connecting north and west.
7 is a 90-degree bend connecting south and west.
F is a 90-degree bend connecting south and east.
. is ground; there is no pipe in this tile.
S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
 */

struct Diagram {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Copy, Clone, Debug)]
struct Pos(usize, usize);

impl Diagram {
    fn get_size(&self) -> Pos {
        Pos(self.tiles.len(), self.tiles[0].len())
    }
    fn get_animal(&self) -> Pos {
        let Pos(max_r, max_c) = self.get_size();
        for r in 0..max_r {
            for c in 0..max_c {
                match self.tiles[r][c] {
                    Tile::Animal => return Pos(r, c),
                    _ => {}
                }
            }
        }
        Pos(0, 0)
    }
    fn get_animal_type(&self) -> Pipe {
        let pos = self.get_animal();
        let north = self.connected_to(pos, Dir::North).is_some();
        let south = self.connected_to(pos, Dir::South).is_some();
        let east = self.connected_to(pos, Dir::East).is_some();
        let west = self.connected_to(pos, Dir::West).is_some();

        if north && south {
            Pipe::Vertical
        } else if north && east {
            Pipe::NorthEast
        } else if north && west {
            Pipe::NorthWest
        } else if south && east {
            Pipe::SouthEast
        } else if south && west {
            Pipe::SouthWest
        } else if east && west {
            Pipe::Horizontal
        } else {
            unreachable!()
        }
    }

    fn part_2(&self) -> usize {
        // sketch:
        let Pos(r, c) = self.get_size();
        let max_r = 3 * r;
        let max_c = 3 * c;
        let mut tripled = vec![vec![false; max_c]; max_r];
        for pos in self.get_loop() {
            let tile = &self.tiles[pos.0][pos.1];
            if let Tile::Pipe(pipe) = tile {
                let (sr, sc) = (pos.0 * 3, pos.1 * 3);
                let tr = pipe.tripled();
                for r in 0..3 {
                    for c in 0..3 {
                        tripled[sr + r][sc + c] = tr[r][c];
                    }
                }
            }
        }
        let pipe = self.get_animal_type();
        let pos = self.get_animal();
        let (sr, sc) = (pos.0 * 3, pos.1 * 3);
        let tr = pipe.tripled();
        for r in 0..3 {
            for c in 0..3 {
                tripled[sr + r][sc + c] = tr[r][c];
            }
        }
        let mut stack = vec![Pos(0, 0)];

        tripled[0][0] = true;

        while let Some(pos) = stack.pop() {
            let mut next = vec![];
            if pos.0 > 0 {
                next.push(Pos(pos.0 - 1, pos.1));
            }
            if pos.1 > 0 {
                next.push(Pos(pos.0, pos.1 - 1));
            }
            if pos.0 + 1 < max_r {
                next.push(Pos(pos.0 + 1, pos.1));
            }
            if pos.1 + 1 < max_c {
                next.push(Pos(pos.0, pos.1 + 1));
            }
            for n in next.into_iter() {
                if !tripled[n.0][n.1] {
                    tripled[n.0][n.1] = true;

                    stack.push(n);
                }
            }
        }

        let Pos(max_r, max_c) = self.get_size();

        let mut count = 0;

        for r in 0..max_r {
            for c in 0..max_c {
                let sr = 3 * r;
                let sc = 3 * c;
                let mut local_count = 0;
                for x in 0..3 {
                    for y in 0..3 {
                        if tripled[sr + x][sc + y] {
                            local_count += 1;
                        }
                    }
                }
                if local_count == 0 {
                    count += 1;
                };
            }
        }

        count
    }

    fn get_loop(&self) -> Vec<Pos> {
        // DFS, unlike part 1
        let mut ans = vec![];
        let mut stack = vec![];
        let start = self.get_animal();
        stack.push(start);
        let mut visited: Vec<Vec<bool>> = self
            .tiles
            .iter()
            .map(|row| row.iter().map(|_| false).collect())
            .collect();
        visited[start.0][start.1] = true;
        while let Some(now) = stack.pop() {
            for next in self.connections(now) {
                if !visited[next.0][next.1] {
                    visited[next.0][next.1] = true;
                    stack.push(next);
                    ans.push(next);
                }
            }
        }
        ans
    }

    fn part_1(&self) -> usize {
        let now = self.get_animal();
        let mut queue: VecDeque<(Pos, usize)> = VecDeque::from([(now, 0)]);
        let mut visited: Vec<Vec<bool>> = self
            .tiles
            .iter()
            .map(|row| row.iter().map(|_| false).collect())
            .collect();
        let mut ans = 0;
        while let Some((pos, dist)) = queue.pop_front() {
            ans = ans.max(dist);
            for next_pos in self.connections(pos) {
                if !visited[next_pos.0][next_pos.1] {
                    visited[next_pos.0][next_pos.1] = true;
                    queue.push_back((next_pos, dist + 1));
                }
            }
        }

        ans
    }

    fn adj(&self, pos: Pos, dir: Dir) -> Option<Pos> {
        match dir {
            Dir::North => self.north(pos),
            Dir::South => self.south(pos),
            Dir::East => self.east(pos),
            Dir::West => self.west(pos),
        }
    }

    fn north(&self, pos: Pos) -> Option<Pos> {
        if pos.0 > 0 {
            Some(Pos(pos.0 - 1, pos.1))
        } else {
            None
        }
    }

    fn south(&self, pos: Pos) -> Option<Pos> {
        if pos.0 + 1 < self.get_size().0 {
            Some(Pos(pos.0 + 1, pos.1))
        } else {
            None
        }
    }
    fn west(&self, pos: Pos) -> Option<Pos> {
        if pos.1 > 0 {
            Some(Pos(pos.0, pos.1 - 1))
        } else {
            None
        }
    }

    fn east(&self, pos: Pos) -> Option<Pos> {
        if pos.1 + 1 < self.get_size().1 {
            Some(Pos(pos.0, pos.1 + 1))
        } else {
            None
        }
    }

    fn connected_to(&self, pos: Pos, dir: Dir) -> Option<Pos> {
        let now = &self.tiles[pos.0][pos.1];
        if let Some(next_pos) = self.adj(pos, dir) {
            let next = &self.tiles[next_pos.0][next_pos.1];
            match dir {
                Dir::East => {
                    if now.heads_east() && next.heads_west() {
                        return Some(next_pos);
                    }
                }
                Dir::West => {
                    if now.heads_west() && next.heads_east() {
                        return Some(next_pos);
                    }
                }
                Dir::North => {
                    if now.heads_north() && next.heads_south() {
                        return Some(next_pos);
                    }
                }
                Dir::South => {
                    if now.heads_south() && next.heads_north() {
                        return Some(next_pos);
                    }
                }
            }
        }
        None
    }

    fn connections(&self, pos: Pos) -> Vec<Pos> {
        [Dir::North, Dir::South, Dir::East, Dir::West]
            .into_iter()
            .filter_map(|dir| self.connected_to(pos, dir))
            .collect()
    }
}

impl FromStr for Diagram {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .map(|line| line.chars().map(|c| Tile::from(c)).collect())
            .collect();
        Ok(Self { tiles })
    }
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    East,
    West,
    South,
    North,
}

#[derive(Debug)]
enum Tile {
    Pipe(Pipe),
    Ground,
    Animal,
}

impl Tile {
    fn heads_north(&self) -> bool {
        match self {
            Tile::Pipe(Pipe::NorthEast)
            | Tile::Pipe(Pipe::NorthWest)
            | Tile::Pipe(Pipe::Vertical)
            | Tile::Animal => true,
            _ => false,
        }
    }

    fn heads_south(&self) -> bool {
        match self {
            Tile::Pipe(Pipe::SouthEast)
            | Tile::Pipe(Pipe::SouthWest)
            | Tile::Pipe(Pipe::Vertical)
            | Tile::Animal => true,
            _ => false,
        }
    }

    fn heads_east(&self) -> bool {
        match self {
            Tile::Pipe(Pipe::SouthEast)
            | Tile::Pipe(Pipe::NorthEast)
            | Tile::Pipe(Pipe::Horizontal)
            | Tile::Animal => true,
            _ => false,
        }
    }

    fn heads_west(&self) -> bool {
        match self {
            Tile::Pipe(Pipe::NorthWest)
            | Tile::Pipe(Pipe::SouthWest)
            | Tile::Pipe(Pipe::Horizontal)
            | Tile::Animal => true,
            _ => false,
        }
    }

    fn from(char: char) -> Self {
        match char {
            'S' => Self::Animal,
            '.' => Self::Ground,
            c => Self::Pipe(Pipe::from(c)),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl Pipe {
    fn tripled(&self) -> [[bool; 3]; 3] {
        match self {
            Pipe::Vertical => [
                [false, true, false],
                [false, true, false],
                [false, true, false],
            ],
            Pipe::Horizontal => [
                [false, false, false],
                [true, true, true],
                [false, false, false],
            ],
            Pipe::NorthEast => [
                [false, true, false],
                [false, true, true],
                [false, false, false],
            ],
            Pipe::NorthWest => [
                [false, true, false],
                [true, true, false],
                [false, false, false],
            ],
            Pipe::SouthEast => [
                [false, false, false],
                [false, true, true],
                [false, true, false],
            ],
            Pipe::SouthWest => [
                [false, false, false],
                [true, true, false],
                [false, true, false],
            ],
        }
    }
    fn from(char: char) -> Self {
        use Pipe::*;
        match char {
            '|' => Vertical,
            '-' => Horizontal,
            'L' => NorthEast,
            'J' => NorthWest,
            '7' => SouthWest,
            'F' => SouthEast,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day10::test_input();
        let ans = Day10::solve_part_1(input);
        assert_eq!(ans, "8");
    }

    #[test]
    fn test_part_2() {
        let input = String::from(
            "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        );
        let ans = Day10::solve_part_2(input);
        assert_eq!(ans, "10");
    }

    #[test]
    fn test_animal_type() {
        let input = Day10::test_input();
        let diagram: Diagram = input.as_str().parse().unwrap();
        assert_eq!(diagram.get_animal_type(), Pipe::SouthEast)
    }
}
