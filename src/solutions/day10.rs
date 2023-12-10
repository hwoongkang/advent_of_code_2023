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
        String::from("0")
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

#[derive(Copy, Clone)]
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

    fn part_2(&self) -> usize {
        // sketch:
        0
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

    fn connections(&self, pos: Pos) -> Vec<Pos> {
        let now = &self.tiles[pos.0][pos.1];
        let mut ans = vec![];

        if let Some(next_pos) = self.north(pos) {
            let next = &self.tiles[next_pos.0][next_pos.1];
            if now.heads_north() && next.heads_south() {
                ans.push(next_pos);
            }
        }
        if let Some(next_pos) = self.south(pos) {
            let next = &self.tiles[next_pos.0][next_pos.1];
            if now.heads_south() && next.heads_north() {
                ans.push(next_pos);
            }
        }
        if let Some(next_pos) = self.east(pos) {
            let next = &self.tiles[next_pos.0][next_pos.1];
            if now.heads_east() && next.heads_west() {
                ans.push(next_pos);
            }
        }
        if let Some(next_pos) = self.west(pos) {
            let next = &self.tiles[next_pos.0][next_pos.1];
            if now.heads_west() && next.heads_east() {
                ans.push(next_pos);
            }
        }
        ans
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

enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl Pipe {
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
            ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        );
        let ans = Day10::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
