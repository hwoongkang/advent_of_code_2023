
use super::Solution;

pub struct Day13;

impl Solution for Day13 {
    fn test_input() -> String {
        String::from("")
    }

    fn solve_part_1(_input: String) -> String {
        String::from("0")
    }

    fn solve_part_2(_input: String) -> String {
        String::from("0")
    }
}

#[cfg(test)]
mod day13_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day13::test_input();
        let ans = Day13::solve_part_1(input);
        assert_eq!(ans, "");
    }

    #[test]
    fn test_part_2() {
        let input = Day13::test_input();
        let ans = Day13::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
