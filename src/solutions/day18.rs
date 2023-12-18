use std::str::FromStr;

use super::Solution;

pub struct Day18;

impl Solution for Day18 {
    fn test_input() -> String {
        String::from(
            "R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)",
        )
    }

    fn solve_part_1(input: String) -> String {
        let mut prev = Pos(0, 0);
        let mut points = vec![prev];
        let mut perimeter = 0;
        for line in input.lines() {
            let (dir, dist) = parse(line);
            perimeter += dist;
            let pos = match dir {
                Dir::Right => Pos(prev.0, prev.1 + dist),
                Dir::Left => Pos(prev.0, prev.1 - dist),
                Dir::Up => Pos(prev.0 - dist, prev.1),
                Dir::Down => Pos(prev.0 + dist, prev.1),
            };
            points.push(pos);
            prev = pos;
        }

        let area = get_area(&points);

        let internal = area + 1 - perimeter / 2;

        (internal + perimeter).to_string()
    }

    fn solve_part_2(input: String) -> String {
        let mut prev = Pos(0, 0);
        let mut points = vec![prev];
        let mut perimeter = 0;
        for line in input.lines() {
            let (dir, dist) = parse_line_2(line);
            perimeter += dist;
            let pos = match dir {
                Dir::Right => Pos(prev.0, prev.1 + dist),
                Dir::Left => Pos(prev.0, prev.1 - dist),
                Dir::Up => Pos(prev.0 - dist, prev.1),
                Dir::Down => Pos(prev.0 + dist, prev.1),
            };
            points.push(pos);
            prev = pos;
        }

        let area = get_area(&points);

        let internal = area + 1 - perimeter / 2;

        (internal + perimeter).to_string()
    }
}

fn get_area(points: &[Pos]) -> i64 {
    let mut ans = 0;
    for j in 1..points.len() {
        let i = j - 1;
        let Pos(x1, y1) = points[i];
        let Pos(x2, y2) = points[j];
        ans += x1 * y2 - x2 * y1;
    }
    ans /= 2;
    ans.abs()
}

#[derive(Clone, Copy)]
struct Pos(i64, i64);

fn parse(line: &str) -> (Dir, i64) {
    let mut words = line.trim().split_whitespace();
    (
        words.next().unwrap().parse().unwrap(),
        words.next().unwrap().parse().unwrap(),
    )
}

fn parse_line_2(line: &str) -> (Dir, i64) {
    let hex = line.trim().split_whitespace().nth(2).unwrap();
    let dist = i64::from_str_radix(&hex[2..7], 16).unwrap();
    let dir = match hex.chars().nth(7).unwrap() {
        //RDLU
        '0' => Dir::Right,
        '1' => Dir::Down,
        '2' => Dir::Left,
        '3' => Dir::Up,
        _ => panic!("sth wrong"),
    };
    (dir, dist)
}

enum Dir {
    Down,
    Up,
    Left,
    Right,
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "D" => Ok(Self::Down),
            "U" => Ok(Self::Up),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod day18_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day18::test_input();
        let ans = Day18::solve_part_1(input);
        assert_eq!(ans, "62");
    }

    #[test]
    fn test_part_2() {
        let input = Day18::test_input();
        let ans = Day18::solve_part_2(input);
        assert_eq!(ans, "952408144115");
    }
}
