
use super::Solution;

pub struct Day18;

impl Solution for Day18 {
    fn test_input() -> String {
        String::from("")
    }

    fn solve_part_1(input: String) -> String {
        input
    }

    fn solve_part_2(input: String) -> String {
        input
    }
}

#[cfg(test)]
mod day18_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day18::test_input();
        let ans = Day18::solve_part_1(input);
        assert_eq!(ans, "");
    }

    #[test]
    fn test_part_2() {
        let input = Day18::test_input();
        let ans = Day18::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
