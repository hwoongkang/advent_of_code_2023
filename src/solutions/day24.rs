use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Div, DivAssign, Sub, SubAssign},
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
        let hails: Vec<Hail> = input
            .lines()
            .map(|line| line.trim().parse().unwrap())
            .collect();

        // reddit idea: iterate 하면서 같은 곳에서 만나게 하기

        let mut candidates: HashMap<Vec3D, Vec3D> = HashMap::new();
        for u in -200i128..200i128 {
            if u % 50 == 0 {
                println!("u={}", u);
            }
            for v in -200i128..20064 {
                let vel = Vec3D { x: u, y: v, z: 0 };
                let hails: Vec<Hail> = hails.iter().take(2).map(|hail| hail.rel_vel(vel)).collect();
                if vel == (Vec3D { x: -3, y: 1, z: 0 }) {
                    // println!("{:?}", hails[0].collides_at(&hails[1]));
                    // println!("{:?}", hails[0]);
                }
                if let Some(pos) = hails[0].collides_at(&hails[1]) {
                    candidates.insert(vel, pos);
                }
            }
        }

        let filtered: HashMap<Vec3D, Vec3D> = candidates
            .into_iter()
            .filter_map(|(vel, pos)| {
                let hails: Vec<Hail> = hails.iter().map(|hail| hail.rel_vel(vel)).collect();
                //for i in 2..(hails.len()) {
                for i in 2..(hails.len()) {
                    let lhs = hails[i - 1];
                    let rhs = hails[i];
                    if vel == (Vec3D { x: -3, y: 1, z: 0 }) {
                        // println!("{:?}, {:?}", lhs, rhs);
                        // println!("{:?}", lhs.collides_at(&rhs));
                    }
                    if let Some(p) = lhs.collides_at(&rhs) {
                        if !(p.x == pos.x && p.y == pos.y) {
                            return None;
                        }
                    }
                }
                Some((vel, pos))
            })
            .collect();

        for (vel, pos) in filtered.into_iter() {
            for w in -500i128..500i128 {
                if w % 50 == 0 {
                    println!("w={}", w);
                }
                let vel = Vec3D {
                    x: vel.x,
                    y: vel.y,
                    z: w,
                };
                let hails: Vec<Hail> = hails.iter().map(|hail| hail.rel_vel(vel)).collect();

                // x0 + t * u = x;
                // y0 + t * v = y;

                let t0 = hails[0].t(pos);
                if t0 < 0 {
                    continue;
                }
                let p0 = hails[0].at_t(t0);

                let mut found = true;

                for i in 1..(hails.len()) {
                    let hail = hails[i];
                    let t = hail.t(pos);
                    if t < 0 {
                        found = false;
                        break;
                    }
                    let p = hail.at_t(t);
                    if p != p0 {
                        found = false;
                        break;
                    }
                }
                if found {
                    println!("!!");
                    println!("pos: {:?}", p0);
                    println!("vel: {:?}", vel);
                    return (p0.x + p0.y + p0.z).to_string();
                }
            }
        }

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

fn gcd(a: i128, b: i128) -> i128 {
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Vec3D {
    x: i128,
    y: i128,
    z: i128,
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

impl DivAssign<i128> for Vec3D {
    fn div_assign(&mut self, rhs: i128) {
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

impl SubAssign for Vec3D {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Div for Vec3D {
    type Output = i128;
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
        let nums: Vec<i128> = s.split(",").map(|s| s.trim().parse().unwrap()).collect();
        Ok(Self {
            x: nums[0],
            y: nums[1],
            z: nums[2],
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
    fn rel_vel(&self, vel: Vec3D) -> Self {
        Hail {
            pos: self.pos,
            vel: self.vel - vel,
        }
    }
    fn collides_at(&self, rhs: &Self) -> Option<Vec3D> {
        let Hail { pos: p0, vel: v0 } = self;
        let Hail { pos: p1, vel: v1 } = rhs;
        let a = v0.x;
        let b = -v1.x;
        let c = v0.y;
        let d = -v1.y;
        let e = p1.x - p0.x;
        let f = p1.y - p0.y;
        // a= 2, b = -1, c = -2, d = 3, e = 2, f = 6
        // det = 6 -2 = 4;
        // de = 6 bf = -6
        //
        let det = a * d - b * c;
        if det == 0 {
            return None;
        }
        // t0= e * d / det - f * b /det;
        let de = d * e;
        let bf = b * f;
        let t = de - bf;
        if t % det == 0 {
            let t = t / det;

            Some(Vec3D {
                x: self.pos.x + t * self.vel.x,
                y: self.pos.y + t * self.vel.y,
                z: self.pos.z + t * self.vel.z,
            })
        } else {
            None
        }
    }
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

    fn t(&self, pos: Vec3D) -> i128 {
        if self.vel.x != 0 {
            (pos.x - self.pos.x) / self.vel.x
        } else if self.vel.y != 0 {
            (pos.y - self.pos.y) / self.vel.y
        } else if self.vel.z != 0 {
            (pos.z - self.pos.z) / self.vel.z
        } else {
            0
        }
        // self.pos.x + t * self.vel.x = pos.x
    }

    fn at_t(&self, t: i128) -> Vec3D {
        Vec3D {
            x: self.pos.x + t * self.vel.x,
            y: self.pos.y + t * self.vel.y,
            z: self.pos.z + t * self.vel.z,
        }
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
        assert_eq!(ans, "47");
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
