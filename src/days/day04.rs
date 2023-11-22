
use super::Solution;

pub struct Day04;

impl Solution for Day04 {
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
mod day04_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day04::test_input();
        let ans = Day04::solve_part_1(input);
        assert_eq!(ans, "");
    }

    #[test]
    fn test_part_2() {
        let input = Day04::test_input();
        let ans = Day04::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
