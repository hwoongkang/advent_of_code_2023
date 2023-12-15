use super::Solution;

pub struct Day15;

impl Solution for Day15 {
    fn test_input() -> String {
        String::from("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7")
    }

    fn solve_part_1(input: String) -> String {
        input
            .split(",")
            .map(|step| hash(step))
            .sum::<u32>()
            .to_string()
    }

    fn solve_part_2(_input: String) -> String {
        String::from("0")
    }
}

fn hash(s: &str) -> u32 {
    s.chars()
        .map(|ch| ch as u8)
        .fold(0u8, |acc, now| acc.wrapping_add(now).wrapping_mul(17)) as u32
}

#[cfg(test)]
mod day15_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day15::test_input();
        let ans = Day15::solve_part_1(input);
        assert_eq!(ans, "1320");
    }

    #[test]
    fn test_part_2() {
        let input = Day15::test_input();
        let ans = Day15::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
