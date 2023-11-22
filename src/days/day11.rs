
use super::Solution;

pub struct Day11;

impl Solution for Day11 {
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
mod day11_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day11::test_input();
        let ans = Day11::solve_part_1(input);
        assert_eq!(ans, "");
    }

    #[test]
    fn test_part_2() {
        let input = Day11::test_input();
        let ans = Day11::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
