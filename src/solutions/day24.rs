use std::{
    ops::{Add, Div, DivAssign, Sub},
    str::FromStr,
};

use super::Solution;

pub struct Day24;

impl Solution for Day24 {
    fn test_input() -> String {
        String::from(
            "19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3",
        )
    }

    fn solve_part_1(input: String) -> String {
        part_1(input, 200000000000000.0, 400000000000000.0).to_string()
    }

    fn solve_part_2(input: String) -> String {
        String::from("0")
    }
}

fn part_1(input: String, min: f64, max: f64) -> usize {
    let mut ans = 0;
    let hails: Vec<Hail> = input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();
    let len = hails.len();
    for i in 0..(len - 1) {
        for j in (i + 1)..len {
            let lhs = &hails[i];
            let rhs = &hails[j];
            if let Some((t0, t1)) = lhs.collides(&rhs) {
                if !(t0 >= 0.0 && t1 >= 0.0) {
                    continue;
                }
                let (x, y, _) = lhs.at(t0);
                if min <= x && x <= max && min <= y && y <= max {
                    ans += 1;
                }
            }
        }
    }
    ans
}

fn gcd(a: i64, b: i64) -> i64 {
    let a = a.abs();
    let b = b.abs();
    if a * b == 0 {
        return 1;
    }
    if a > b {
        gcd(b, a)
    } else {
        if b % a == 0 {
            a
        } else {
            gcd(b % a, a)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Vec3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3D {
    fn is_parallel(&self, rhs: &Self) -> bool {
        self.normalized() == rhs.normalized()
    }
}

impl Vec3D {
    fn normalize(&mut self) {
        let d = gcd(self.z, gcd(self.y, self.x));

        *self /= d;
    }

    fn normalized(&self) -> Self {
        let mut ans = self.clone();
        ans.normalize();
        ans
    }
}

impl DivAssign<i64> for Vec3D {
    fn div_assign(&mut self, rhs: i64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Add for Vec3D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Div for Vec3D {
    type Output = i64;
    fn div(self, rhs: Self) -> Self::Output {
        if !self.is_parallel(&rhs) {
            panic!("cannot divide: {:?} and {:?}", self, rhs);
        } else {
            let (divisor, divider) = if rhs.x != 0 {
                (self.x, rhs.x)
            } else if rhs.y != 0 {
                (self.y, rhs.y)
            } else if rhs.z != 0 {
                (self.z, rhs.z)
            } else {
                unreachable!()
            };
            if divisor % divider != 0 {
                panic!("noninteger time for : {:?} and {:?}", self, rhs);
            }
            divisor / divider
        }
    }
}

impl FromStr for Vec3D {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<i64> = s.split(",").map(|s| s.trim().parse().unwrap()).collect();
        Ok(Self {
            x: nums[0],
            y: nums[1],
            z: nums[2],
        })
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Hail {
    pos: Vec3D,
    vel: Vec3D,
}

impl FromStr for Hail {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vecs: Vec<Vec3D> = s.split("@").map(|s| s.trim().parse().unwrap()).collect();
        Ok(Self {
            pos: vecs[0],
            vel: vecs[1],
        })
    }
}

impl Hail {
    fn collides(&self, rhs: &Self) -> Option<(f64, f64)> {
        let Hail { pos: p0, vel: v0 } = self;
        let Hail { pos: p1, vel: v1 } = rhs;

        // p0 + v0 * t0 = p1 + v1 * t1
        // v0 * t0 - v1 * t1 = p1 - p0;
        // v0x -v1x t0 = p1x - p0x
        // v0y -v1y t1 = p1y - p0y

        let a = v0.x as f64;
        let b = -v1.x as f64;
        let c = v0.y as f64;
        let d = -v1.y as f64;
        let e = p1.x as f64 - p0.x as f64;
        let f = p1.y as f64 - p0.y as f64;

        let det = a * d - b * c;
        if det == 0.0 {
            // cannot solve
            None
        } else {
            let (a, b, c, d) = (d / det, -b / det, -c / det, a / det);
            let t0 = a * e + b * f;
            let t1 = c * e + d * f;
            Some((t0, t1))
        }
    }

    fn at(&self, t: f64) -> (f64, f64, f64) {
        let x = (self.pos.x as f64) + (self.vel.x as f64) * t;
        let y = (self.pos.y as f64) + (self.vel.y as f64) * t;
        let z = (self.pos.z as f64) + (self.vel.z as f64) * t;
        (x, y, z)
    }
}

#[cfg(test)]
mod day24_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day24::test_input();
        let ans = part_1(input, 7.0, 27.0);
        assert_eq!(ans, 2);
    }

    #[test]
    fn test_part_2() {
        let input = Day24::test_input();
        let ans = Day24::solve_part_2(input);
        assert_eq!(ans, "");
    }

    #[test]
    fn test_parallel() {
        let lhs = "-2, -2, -4".parse::<Vec3D>().unwrap();
        let rhs = "-1, -1, -2".parse::<Vec3D>().unwrap();
        assert!(lhs.is_parallel(&rhs));
        assert_eq!(
            lhs,
            Vec3D {
                x: -2,
                y: -2,
                z: -4
            }
        )
    }
}
