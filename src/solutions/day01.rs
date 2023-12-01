use super::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn test_input() -> String {
        String::from(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        )
    }

    fn solve_part_1(input: String) -> String {
        input
            .lines()
            .map(|line| get_calibrated_digit(line))
            .sum::<usize>()
            .to_string()
    }

    fn solve_part_2(_input: String) -> String {
        String::from("0")
    }
}

fn get_calibrated_digit(line: &str) -> usize {
    let mut digits: Vec<usize> = line
        .chars()
        .filter_map(|c| {
            if c.is_ascii_digit() {
                Some((c as u8 - '0' as u8) as usize)
            } else {
                None
            }
        })
        .collect();
    10 * digits.first().unwrap() + digits.last().unwrap()
}

#[cfg(test)]
mod day01_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day01::test_input();
        let ans = Day01::solve_part_1(input);
        assert_eq!(ans, "142");
    }

    #[test]
    fn test_part_2() {
        let input = Day01::test_input();
        let ans = Day01::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
