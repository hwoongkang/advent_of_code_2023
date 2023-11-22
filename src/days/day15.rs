
use super::Solution;

pub struct Day15;

impl Solution for Day15 {
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
mod day15_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day15::test_input();
        let ans = Day15::solve_part_1(input);
        assert_eq!(ans, "");
    }

    #[test]
    fn test_part_2() {
        let input = Day15::test_input();
        let ans = Day15::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
