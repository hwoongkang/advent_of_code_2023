use super::Solution;

pub struct Day12;

impl Solution for Day12 {
    fn test_input() -> String {
        String::from(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        )
    }

    fn solve_part_1(input: String) -> String {
        input
            .lines()
            .map(|line| parse_line(line))
            .map(|(springs, pattern)| brute_force(&springs, &pattern))
            .sum::<usize>()
            .to_string()
    }

    fn solve_part_2(input: String) -> String {
        String::from("0")
    }
}

fn parse_line(line: &str) -> (Vec<Spring>, Vec<usize>) {
    let mut words = line.trim().split_whitespace();
    (
        words
            .next()
            .unwrap()
            .trim()
            .chars()
            .map(|ch| Spring::from(ch))
            .collect(),
        words
            .next()
            .unwrap()
            .trim()
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect(),
    )
}

fn check(springs: &[Spring], pattern: &[usize]) -> bool {
    let mut chunks: Vec<usize> = vec![];
    let mut last = 0;
    for s in springs.iter() {
        if let Spring::Broken = s {
            last += 1;
        } else {
            if last != 0 {
                chunks.push(last);
                last = 0;
            }
        }
    }
    if last != 0 {
        chunks.push(last);
    }

    chunks == pattern
}

fn perm_with_rep(len: usize) -> Vec<Vec<bool>> {
    if len == 1 {
        vec![vec![true], vec![false]]
    } else {
        let prev = perm_with_rep(len - 1);
        prev.iter()
            .map(|p| {
                let mut new = vec![];
                let mut n = p.clone();
                n.push(false);
                new.push(n);
                let mut n = p.clone();
                n.push(true);
                new.push(n);
                new
            })
            .flatten()
            .collect()
    }
}

fn brute_force(springs: &[Spring], pattern: &[usize]) -> usize {
    let mut springs: Vec<Spring> = springs.iter().map(|s| *s).collect();
    let wild_cards: Vec<usize> = springs
        .iter()
        .enumerate()
        .filter_map(|(i, spring)| {
            if let Spring::Unknown = spring {
                Some(i)
            } else {
                None
            }
        })
        .collect();
    let to_check = perm_with_rep(wild_cards.len());
    to_check
        .iter()
        .map(|bools| {
            wild_cards.iter().enumerate().for_each(|(i, j)| {
                springs[*j] = if bools[i] {
                    Spring::Operational
                } else {
                    Spring::Broken
                };
            });
            if check(&springs, pattern) {
                1
            } else {
                0
            }
        })
        .sum()
}

#[derive(Copy, Clone, Debug)]
enum Spring {
    Operational,
    Broken,
    Unknown,
}

impl Spring {
    fn from(char: char) -> Self {
        match char {
            '.' => Self::Operational,
            '#' => Self::Broken,
            '?' => Self::Unknown,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod Day12_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day12::test_input();
        let ans = Day12::solve_part_1(input);
        assert_eq!(ans, "21");
    }

    #[test]
    fn test_part_2() {
        let input = Day12::test_input();
        let ans = Day12::solve_part_2(input);
        assert_eq!(ans, "");
    }

    #[test]
    fn test_permutation() {
        let perm = perm_with_rep(4);
        assert_eq!(perm.len(), 16);
    }
}
