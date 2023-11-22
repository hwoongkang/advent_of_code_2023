
use super::Solution;

pub struct Day07;

impl Solution for Day07 {
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
mod day07_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day07::test_input();
        let ans = Day07::solve_part_1(input);
        assert_eq!(ans, "");
    }

    #[test]
    fn test_part_2() {
        let input = Day07::test_input();
        let ans = Day07::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
