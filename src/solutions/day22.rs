use std::collections::{HashSet, VecDeque};
use std::{cmp::Ordering, ops::SubAssign, str::FromStr};

use std::ops::{Range as StdRange, Sub};

use super::Solution;

pub struct Day22;

impl Solution for Day22 {
    fn test_input() -> String {
        String::from(
            "1,1,8~1,1,9
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6",
        )
    }

    fn solve_part_1(input: String) -> String {
        let mut bricks: Vec<Brick> = input.lines().map(|line| line.parse().unwrap()).collect();

        let (under, over, topo) = collapse_bricks(&mut bricks);

        let mut supported_by: Vec<Vec<usize>> = bricks.iter().map(|_| vec![]).collect();

        for i in 0..bricks.len() {
            let me = &bricks[i];
            for j in over[i].iter() {
                let under_me = &bricks[*j];
                if under_me.distance(me) == 0 {
                    supported_by[i].push(*j);
                }
            }
        }

        let mut crucial: HashSet<usize> = HashSet::new();

        for supports in supported_by {
            if supports.len() == 1 {
                crucial.insert(supports[0]);
            }
        }
        (bricks.len() - crucial.len()).to_string()
    }

    fn solve_part_2(input: String) -> String {
        let mut bricks: Vec<Brick> = input.lines().map(|line| line.parse().unwrap()).collect();

        let (under, over, topo) = collapse_bricks(&mut bricks);

        let mut supported_by: Vec<Vec<usize>> = bricks.iter().map(|_| vec![]).collect();
        let mut supports: Vec<Vec<usize>> = bricks.iter().map(|_| vec![]).collect();

        for i in 0..bricks.len() {
            let me = &bricks[i];
            for j in over[i].iter() {
                let under_me = &bricks[*j];
                if under_me.distance(me) == 0 {
                    supported_by[i].push(*j);
                    supports[*j].push(i);
                }
            }
        }

        let simulate = |root: usize| {
            let mut removed: Vec<bool> = bricks.iter().map(|_| false).collect();
            let mut queue = VecDeque::from([root]);
            removed[root] = true;
            let mut count = 0;
            while let Some(i) = queue.pop_front() {
                count += 1;
                for &next in supports[i].iter() {
                    if removed[next] {
                        continue;
                    }

                    let should_remove = supported_by[next]
                        .iter()
                        .map(|index| removed[*index])
                        .fold(true, |acc, now| acc && now);
                    if should_remove {
                        removed[next] = true;
                        queue.push_back(next);
                    }
                }
            }
            count - 1
        };

        let ans = (0..bricks.len())
            .into_iter()
            .map(simulate)
            .sum::<usize>()
            .to_string();

        ans
    }
}

fn collapse_bricks(bricks: &mut [Brick]) -> (Vec<Vec<usize>>, Vec<Vec<usize>>, Vec<usize>) {
    let (under, over, topo) = topological_sort(&bricks);
    for i in 0..bricks.len() {
        let i = topo[i];

        if over[i].len() == 0 {
            let brick = &mut bricks[i];
            *brick -= Pos(0, 0, brick.bottom() - 1);
        } else {
            let brick = &bricks[i];
            let dist = over[i]
                .iter()
                .map(|j| bricks[*j].distance(brick))
                .min()
                .unwrap();
            let brick = &mut bricks[i];
            *brick -= Pos(0, 0, dist);
        }
    }
    (under, over, vec![])
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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Brick {
    start: Pos,
    end: Pos,
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
    Vec<Vec<usize>>, // shows which indices are above me
    Vec<Vec<usize>>, // show which indices are under me
    Vec<usize>,      //result
) {
    let mut under: Vec<Vec<usize>> = bricks.iter().map(|_| vec![]).collect();
    let mut over: Vec<Vec<usize>> = bricks.iter().map(|_| vec![]).collect();
    let len = bricks.len();
    for i in 0..(len - 1) {
        for j in (i + 1)..len {
            let lhs = &bricks[i];
            let rhs = &bricks[j];
            match lhs.partial_cmp(rhs) {
                None => {}
                Some(Ordering::Less) => {
                    under[i].push(j);
                    over[j].push(i);
                }
                Some(Ordering::Greater) => {
                    under[j].push(i);
                    over[i].push(j);
                }
                _ => unreachable!(),
            }
        }
    }

    let mut visited: Vec<bool> = bricks.iter().map(|_| false).collect();

    let mut ans: VecDeque<usize> = VecDeque::new();

    fn dfs(from: usize, edges: &[Vec<usize>], visited: &mut [bool], ans: &mut VecDeque<usize>) {
        visited[from] = true;
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
            dfs(i, &under, &mut visited, &mut ans);
        }
    }

    (under, over, Vec::from(ans))
}

impl Brick {
    fn top(&self) -> usize {
        self.start.2.max(self.end.2)
    }
    fn bottom(&self) -> usize {
        self.start.2.min(self.end.2)
    }

    fn distance(&self, up: &Self) -> usize {
        if !(self <= up) {
            unreachable!()
        }
        up.bottom() - self.top() - 1
    }
}

#[cfg(test)]
mod day22_tests {
    use super::*;

    #[test]
    fn test_collapse() {
        let input = Day22::test_input();
        let mut bricks: Vec<Brick> = input.lines().map(|line| line.parse().unwrap()).collect();
        let mut original: Vec<Brick> = input.lines().map(|line| line.parse().unwrap()).collect();
        let _ = collapse_bricks(&mut bricks);
        let fall_dist: [usize; 7] = [3, 0, 0, 1, 1, 2, 2];
        for i in 0..7 {
            let brick = &mut original[i];
            *brick -= Pos(0, 0, fall_dist[i]);

            assert_eq!(*brick, bricks[i]);
        }
    }

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
        assert_eq!(ans, "7");
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
        let (under, _, ans) = topological_sort(&bricks);
        // G A B C D E F
        assert_eq!(
            under,
            vec![
                vec![],
                vec![0, 2, 3, 6],
                vec![4, 5],
                vec![4, 5],
                vec![6],
                vec![6],
                vec![0]
            ]
        );

        assert!(
            ans == [1, 2, 3, 4, 5, 6, 0]
                || ans == [1, 3, 2, 4, 5, 6, 0]
                || ans == [1, 2, 3, 5, 4, 6, 0]
                || ans == [1, 3, 2, 5, 4, 6, 0]
        );
    }
}
