
use super::Solution;

pub struct Day14;

impl Solution for Day14 {
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
mod day14_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day14::test_input();
        let ans = Day14::solve_part_1(input);
        assert_eq!(ans, "");
    }

    #[test]
    fn test_part_2() {
        let input = Day14::test_input();
        let ans = Day14::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
