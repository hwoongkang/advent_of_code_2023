use std::collections::HashMap;

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
            .map(|(springs, pattern)| dp(&springs, &pattern))
            .sum::<usize>()
            .to_string()
    }

    fn solve_part_2(input: String) -> String {
        input
            .lines()
            .map(|line| parse_line(line))
            .map(|(springs, pattern)| {
                let mut new_one = springs.clone();
                for _ in 0..4 {
                    new_one.push(Spring::Unknown);
                    new_one.append(&mut springs.clone());
                }
                (
                    new_one,
                    (0..5)
                        .map(|_| pattern.clone())
                        .flatten()
                        .collect::<Vec<_>>(),
                )
            })
            .enumerate()
            .map(|(i, (springs, pattern))| dp(&springs, &pattern))
            .sum::<usize>()
            .to_string()
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

fn dp(springs: &[Spring], pattern: &[usize]) -> usize {
    let mut cache: HashMap<(usize, usize, usize), usize> = HashMap::new();

    fn inner(
        i: usize,
        n: usize,
        b: usize,
        springs: &[Spring],
        pattern: &[usize],
        cache: &mut HashMap<(usize, usize, usize), usize>,
    ) -> usize {
        if let Some(ans) = cache.get(&(i, n, b)) {
            return *ans;
        }

        if i == springs.len() {
            if n == pattern.len() && b == 0 {
                return 1;
            } else if let Some(&last) = pattern.last() {
                if last == b && n == pattern.len() - 1 {
                    return 1;
                }
            }
            return 0;
        }
        let mut ans = 0;

        match springs[i] {
            Spring::Broken => {}
            _ => {
                if b == 0 {
                    ans += inner(i + 1, n, 0, springs, pattern, cache);
                } else {
                    if n == pattern.len() {
                        return 0;
                    }
                    if b == pattern[n] {
                        ans += inner(i + 1, n + 1, 0, springs, pattern, cache);
                    }
                }
            }
        }

        match springs[i] {
            Spring::Operational => {}
            _ => {
                ans += inner(i + 1, n, b + 1, springs, pattern, cache);
            }
        }

        cache.insert((i, n, b), ans);
        ans
    };
    inner(0, 0, 0, springs, pattern, &mut cache)
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
mod day12_tests {
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

        assert_eq!(ans, "525152");
    }
}
