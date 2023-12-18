use std::str::FromStr;

use super::Solution;

pub struct Day16;

impl Solution for Day16 {
    fn test_input() -> String {
        String::from(
            r".|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....",
        )
    }

    fn solve_part_1(input: String) -> String {
        let contraption: Contraption = input.parse().unwrap();

        contraption
            .energize(Light {
                r: 0,
                c: 0,
                dir: Dir::Right,
            })
            .to_string()
    }

    fn solve_part_2(input: String) -> String {
        let contraption: Contraption = input.parse().unwrap();
        contraption.maximize().to_string()
    }
}

#[derive(Debug)]
struct Contraption {
    wall: Vec<Vec<Option<Mirror>>>,
}

impl Contraption {
    fn size(&self) -> (usize, usize) {
        (self.wall.len(), self.wall[0].len())
    }

    fn maximize(&self) -> usize {
        let (max_r, max_c) = self.size();
        let mut lights: Vec<Light> = vec![];
        lights.extend((0..max_r).map(|r| Light {
            r,
            c: 0,
            dir: Dir::Right,
        }));
        lights.extend((0..max_r).map(|r| Light {
            r,
            c: max_c - 1,
            dir: Dir::Left,
        }));
        lights.extend((0..max_c).map(|c| Light {
            r: 0,
            c,
            dir: Dir::Down,
        }));
        lights.extend((0..max_c).map(|c| Light {
            r: max_r - 1,
            c,
            dir: Dir::Up,
        }));
        lights
            .into_iter()
            .map(|light| self.energize(light))
            .max()
            .unwrap()
    }
    fn energize(&self, initial: Light) -> usize {
        // r, c, direction
        let mut visited: Vec<Vec<Vec<bool>>> = self
            .wall
            .iter()
            .map(|row| row.iter().map(|_| vec![false; 4]).collect())
            .collect();

        let mut lights: Vec<Light> = vec![initial];

        let (r, c, d) = lights[0].coord();
        visited[r][c][d] = true;

        while let Some(light) = lights.pop() {
            for next_light in self.tick(&light) {
                let (r, c, d) = next_light.coord();
                if !visited[r][c][d] {
                    visited[r][c][d] = true;
                    lights.push(next_light);
                }
            }
        }

        visited
            .iter()
            .map(|row| {
                row.iter()
                    .map(|col| {
                        if col.iter().fold(false, |acc, now| acc || *now) {
                            1
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
    fn tick(&self, light: &Light) -> Vec<Light> {
        self.reflect(light)
            .iter()
            .filter_map(|light| self.proceed(light))
            .collect()
    }
    fn proceed(&self, light: &Light) -> Option<Light> {
        let (max_r, max_c) = self.size();
        let r = light.r;
        let c = light.c;
        let dir = light.dir;
        match dir {
            Dir::Left => {
                if c > 0 {
                    return Some(Light { r, c: c - 1, dir });
                }
            }
            Dir::Right => {
                if c + 1 < max_c {
                    return Some(Light { r, c: c + 1, dir });
                }
            }
            Dir::Up => {
                if r > 0 {
                    return Some(Light { r: r - 1, c, dir });
                }
            }
            Dir::Down => {
                if r + 1 < max_r {
                    return Some(Light { r: r + 1, c, dir });
                }
            }
        }
        None
    }

    fn reflect(&self, light: &Light) -> Vec<Light> {
        let r = light.r;
        let c = light.c;
        let dir = light.dir;
        match &self.wall[r][c] {
            None => vec![Light { r, c, dir }],
            Some(mirror) => mirror
                .reflect(&dir)
                .into_iter()
                .map(|dir| Light { r, c, dir })
                .collect(),
        }
    }
}

impl FromStr for Contraption {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            wall: s
                .lines()
                .map(|line| line.trim().chars().map(|char| Mirror::from(char)).collect())
                .collect(),
        })
    }
}

#[derive(Debug)]
enum Mirror {
    Vertical,
    Horizontal,
    Slash,
    Backslash,
}

impl Mirror {
    fn from(char: char) -> Option<Self> {
        match char {
            '|' => Some(Self::Vertical),
            '-' => Some(Self::Horizontal),
            '/' => Some(Self::Slash),
            '\\' => Some(Self::Backslash),
            _ => None,
        }
    }

    fn reflect(&self, dir: &Dir) -> Vec<Dir> {
        match self {
            Self::Vertical => match dir {
                Dir::Left | Dir::Right => vec![Dir::Up, Dir::Down],
                d => vec![*d],
            },
            Self::Horizontal => match dir {
                Dir::Up | Dir::Down => vec![Dir::Left, Dir::Right],
                d => vec![*d],
            },
            Self::Slash => match dir {
                Dir::Up => vec![Dir::Right],
                Dir::Right => vec![Dir::Up],
                Dir::Left => vec![Dir::Down],
                Dir::Down => vec![Dir::Left],
            },
            Self::Backslash => match dir {
                Dir::Up => vec![Dir::Left],
                Dir::Right => vec![Dir::Down],
                Dir::Left => vec![Dir::Up],
                Dir::Down => vec![Dir::Right],
            },
        }
    }
}

#[derive(Debug)]
struct Light {
    r: usize,
    c: usize,
    dir: Dir,
}

impl Light {
    fn coord(&self) -> (usize, usize, usize) {
        (self.r, self.c, self.dir as usize)
    }
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[cfg(test)]
mod day16_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day16::test_input();
        let ans = Day16::solve_part_1(input);
        assert_eq!(ans, "46");
    }

    #[test]
    fn test_part_2() {
        let input = Day16::test_input();
        let ans = Day16::solve_part_2(input);
        assert_eq!(ans, "51");
    }
}
