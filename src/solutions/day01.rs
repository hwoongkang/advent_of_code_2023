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

    fn solve_part_2(input: String) -> String {
        input
            .lines()
            .map(|line| translate_line(line))
            .map(|line| get_calibrated_digit(&line))
            .sum::<usize>()
            .to_string()
    }
}

fn get_calibrated_digit(line: &str) -> usize {
    let digits: Vec<usize> = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|n| n as usize)
        .collect();
    10 * digits.first().unwrap() + digits.last().unwrap()
}

fn translate_line(line: &str) -> String {
    let mut line = line.to_string();
    line = line.replace("one", "o1e");
    line = line.replace("two", "t2o");
    line = line.replace("three", "t3e");
    line = line.replace("four", "f4r");
    line = line.replace("five", "f5e");
    line = line.replace("six", "s6x");
    line = line.replace("seven", "s7n");
    line = line.replace("eight", "e8t");
    line = line.replace("nine", "n9e");

    line
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
        let input = String::from(
            "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen",
        );
        let ans = Day01::solve_part_2(input);
        assert_eq!(ans, "281");
    }
}
