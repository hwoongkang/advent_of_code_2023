use std::collections::VecDeque;
use std::{cmp::Ordering, collections::HashMap, ops::SubAssign, str::FromStr};

use std::ops::Range as StdRange;

use super::Solution;

pub struct Day22;

impl Solution for Day22 {
    fn test_input() -> String {
        String::from(
            "1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9",
        )
    }

    fn solve_part_1(input: String) -> String {
        let mut bricks: Vec<Brick> = input.lines().map(|line| line.parse().unwrap()).collect();

        let (edges, topo) = topological_sort(&bricks);

        println!("{:?}", edges.iter().map(|e| e.len()).collect::<Vec<_>>());

        String::from("0")
    }

    fn solve_part_2(_input: String) -> String {
        String::from("0")
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Pos(usize, usize, usize);

impl SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Brick {
    start: Pos,
    end: Pos,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Dir {
    X,
    Y,
    Z,
}

impl SubAssign<Pos> for Brick {
    fn sub_assign(&mut self, rhs: Pos) {
        self.start -= rhs;
        self.end -= rhs;
    }
}

impl FromStr for Pos {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.trim().split(",");
        Ok(Self(
            words.next().unwrap().parse().unwrap(),
            words.next().unwrap().parse().unwrap(),
            words.next().unwrap().parse().unwrap(),
        ))
    }
}

impl FromStr for Brick {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.trim().split("~");
        Ok(Self {
            start: words.next().unwrap().parse().unwrap(),
            end: words.next().unwrap().parse().unwrap(),
        })
    }
}

impl Brick {
    fn dir(&self) -> Dir {
        let Pos(x0, y0, z0) = self.start;
        let Pos(x1, y1, z1) = self.end;
        if z0 != z1 {
            Dir::Z
        } else if x0 != x1 {
            Dir::X
        } else if y0 != y1 {
            Dir::Y
        } else {
            Dir::Z
        }
    }
    fn bottom(&self) -> Vec<Pos> {
        let dir = self.dir();
        let Pos(x0, y0, z0) = self.start;
        let Pos(x1, y1, z1) = self.end;
        match dir {
            Dir::Z => vec![Pos(x0, y0, z0.min(z1))],
            Dir::X => (x0.min(x1)..=x0.max(x1)).map(|x| Pos(x, y0, z0)).collect(),
            Dir::Y => (y0.min(y1)..=y0.max(y1)).map(|y| Pos(x0, y, z0)).collect(),
        }
    }

    fn top(&self) -> Vec<Pos> {
        let dir = self.dir();
        let Pos(x0, y0, z0) = self.start;
        let Pos(x1, y1, z1) = self.end;
        match dir {
            Dir::Z => vec![Pos(x0, y0, z0.max(z1) + 1)],
            Dir::X => (x0.min(x1)..=x0.max(x1))
                .map(|x| Pos(x, y0, z0 + 1))
                .collect(),
            Dir::Y => (y0.min(y1)..=y0.max(y1))
                .map(|y| Pos(x0, y, z0 + 1))
                .collect(),
        }
    }

    fn as_range(&self) -> (Range, Range, Range) {
        (
            Range::from(self.start.0, self.end.0),
            Range::from(self.start.1, self.end.1),
            Range::from(self.start.2, self.end.2),
        )
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let (sx, sy, sz) = self.as_range();
        let (ex, ey, ez) = other.as_range();
        if !(sx.overlaps(&ex) && sy.overlaps(&ey)) {
            None
        } else {
            (sz.1 - 1).partial_cmp(&ez.0)
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Range(usize, usize);

impl Range {
    fn from(a: usize, b: usize) -> Self {
        let s = a.min(b);
        let e = a.max(b);
        Self(s, e + 1)
    }
    fn size(&self) -> usize {
        if self.1 <= self.0 {
            0
        } else {
            self.1 - self.0
        }
    }
    fn overlap(&self, rhs: &Range) -> Option<Range> {
        if self.overlaps(rhs) {
            Some(Range(self.0.max(rhs.0), self.1.min(rhs.1)))
        } else {
            None
        }
    }
    fn overlaps(&self, rhs: &Range) -> bool {
        self.0 < rhs.1 && rhs.0 < self.1
    }
}

impl IntoIterator for Range {
    type IntoIter = StdRange<usize>;
    type Item = usize;
    fn into_iter(self) -> Self::IntoIter {
        self.0..self.1
    }
}

fn topological_sort(
    bricks: &[Brick],
) -> (
    Vec<Vec<usize>>, // relations
    Vec<usize>,      //result
) {
    let mut edges: Vec<Vec<usize>> = bricks.iter().map(|_| vec![]).collect();
    let len = bricks.len();
    for i in 0..(len - 1) {
        for j in (i + 1)..len {
            let lhs = &bricks[i];
            let rhs = &bricks[j];
            match lhs.partial_cmp(rhs) {
                None => {}
                Some(Ordering::Less) => edges[i].push(j),
                Some(Ordering::Greater) => edges[j].push(i),
                _ => unreachable!(),
            }
        }
    }

    let mut visited: Vec<bool> = bricks.iter().map(|_| false).collect();

    let mut ans: VecDeque<usize> = VecDeque::new();

    fn dfs(from: usize, edges: &[Vec<usize>], visited: &mut [bool], ans: &mut VecDeque<usize>) {
        for &next in edges[from].iter() {
            if !visited[next] {
                visited[next] = true;
                dfs(next, edges, visited, ans);
            }
        }
        ans.push_front(from);
    }

    for i in 0..len {
        if !visited[i] {
            dfs(i, &edges, &mut visited, &mut ans);
        }
    }

    (edges, Vec::from(ans))
}

#[cfg(test)]
mod day22_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day22::test_input();
        let ans = Day22::solve_part_1(input);
        assert_eq!(ans, "5");
    }

    #[test]
    fn test_part_2() {
        let input = Day22::test_input();
        let ans = Day22::solve_part_2(input);
        assert_eq!(ans, "");
    }

    #[test]
    fn test_top_bottom() {
        let a: Brick = "1,0,1~1,2,1".parse().unwrap();
        assert_eq!(a.top(), vec![Pos(1, 0, 2), Pos(1, 1, 2), Pos(1, 2, 2)]);
        assert_eq!(a.bottom(), vec![Pos(1, 0, 1), Pos(1, 1, 1), Pos(1, 2, 1)]);

        let g: Brick = "1,1,8~1,1,9".parse().unwrap();
        assert_eq!(g.top(), vec![Pos(1, 1, 10)]);
        assert_eq!(g.bottom(), vec![Pos(1, 1, 8)]);
    }

    #[test]
    fn test_partial_ord() {
        let a: Brick = "1,0,1~1,2,1".parse().unwrap();
        let b: Brick = "1,1,8~1,1,9".parse().unwrap();
        assert!(a <= b);

        let b: Brick = "2,1,8~2,1,9".parse().unwrap();
        assert!(a.partial_cmp(&b).is_none())
    }

    #[test]
    fn test_topological_sort() {
        let input = Day22::test_input();
        let bricks: Vec<Brick> = input.lines().map(|line| line.parse().unwrap()).collect();
        let (edges, ans) = topological_sort(&bricks);
        assert_eq!(
            edges,
            vec![
                vec![1, 2, 5, 6],
                vec![3, 4],
                vec![3, 4],
                vec![5],
                vec![5],
                vec![6],
                vec![]
            ]
        );

        assert!(
            ans == [0, 1, 2, 3, 4, 5, 6]
                || ans == [0, 2, 1, 3, 4, 5, 6]
                || ans == [0, 1, 2, 4, 3, 5, 6]
                || ans == [0, 2, 1, 4, 3, 5, 6]
        );
    }
}
