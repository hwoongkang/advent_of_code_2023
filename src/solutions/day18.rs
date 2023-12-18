use std::str::FromStr;

use super::Solution;

use super::utils::Pos;

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
        let mut points = vec![(0, 0)];
        for line in input.lines() {
            let (dir, dist) = parse(line);
            let p = *points.last().unwrap();

            match dir {
                Dir::Down => {
                    for i in 1..=dist {
                        points.push((p.0 + i, p.1));
                    }
                }
                Dir::Up => {
                    for i in 1..=dist {
                        points.push((p.0 - i, p.1));
                    }
                }
                Dir::Right => {
                    for i in 1..=dist {
                        points.push((p.0, p.1 + i));
                    }
                }
                Dir::Left => {
                    for i in 1..=dist {
                        points.push((p.0, p.1 - i));
                    }
                }
            }
        }
        let min_x = points.iter().map(|(x, _)| x).min().unwrap();
        let max_x = points.iter().map(|(x, _)| x).max().unwrap();
        let min_y = points.iter().map(|(_, y)| y).min().unwrap();
        let max_y = points.iter().map(|(_, y)| y).max().unwrap();
        let mut grid: Vec<Vec<Option<bool>>> = (0..=(max_x - min_x + 3))
            .map(|_| (0..(max_y - min_y + 3)).map(|_| Some(false)).collect())
            .collect();

        for point in points.iter() {
            let (x, y) = point;
            let x = x - min_x + 1;
            let y = y - min_y + 1;
            let x = x as usize;
            let y = y as usize;
            grid[x][y] = Some(true);
        }

        let mut stack = vec![Pos(0, 0)];
        grid[0][0] = None;
        let clamp = Pos(grid.len(), grid[0].len());
        while let Some(pos) = stack.pop() {
            for np in pos.next(&clamp) {
                if let Some(false) = grid[np.0][np.1] {
                    grid[np.0][np.1] = None;
                    stack.push(np);
                }
            }
        }
        grid.iter()
            .map(|row| row.iter().filter_map(|a| *a).count())
            .sum::<usize>()
            .to_string()
    }

    fn solve_part_2(input: String) -> String {
        String::from("0")
    }
}

fn parse(line: &str) -> (Dir, i64) {
    let mut words = line.trim().split_whitespace();
    (
        words.next().unwrap().parse().unwrap(),
        words.next().unwrap().parse().unwrap(),
    )
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
        assert_eq!(ans, "");
    }
}
