use super::Solution;

pub struct Day06;

impl Solution for Day06 {
    fn test_input() -> String {
        String::from(
            "Time:      7  15   30
Distance:  9  40  200",
        )
    }

    fn solve_part_1(input: String) -> String {
        let mut lines = input.lines();
        let times: Vec<usize> = lines
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .skip(1)
            .map(|w| w.parse().unwrap())
            .collect();
        let distances: Vec<usize> = lines
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .skip(1)
            .map(|w| w.parse().unwrap())
            .collect();
        times
            .iter()
            .enumerate()
            .map(|(i, t)| {
                let dist = distances[i];

                valid_range(*t, dist)
            })
            .fold(1, |i, j| i * j)
            .to_string()
    }

    fn solve_part_2(input: String) -> String {
        let mut lines = input.lines();
        let time: usize = lines
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .skip(1)
            .map(|w| w.to_string())
            .fold(String::new(), |a, b| a + &b)
            .parse()
            .unwrap();
        let distance: usize = lines
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .skip(1)
            .map(|w| w.to_string())
            .fold(String::new(), |a, b| a + &b)
            .parse()
            .unwrap();
        valid_range(time, distance).to_string()
    }
}

fn valid_range(t: usize, dist: usize) -> usize {
    // x * (t-x) > dist
    // - x * x + tx - dist > 0
    // x*x - tx + dist < 0
    // (x - t/2)^2 + dist - t ^ 2 /4 < 0
    // (x -  t/2) ^ 2 - (t^2/4 - dist) < 09
    // (x - t / 2 + sqrt(t^2/4 - dist) (x - t/2 - sqrt(t^2/4 - dist)) < 0
    // t/2 - sqrt(t^2/4 - dist) < x < t/2 + sqrt(t^2/4 - dist)
    let t = t as f64;
    let dist = dist as f64;
    let min_x = t / 2.0 - f64::sqrt(t * t / 4.0 - dist);
    let max_x = t / 2.0 + f64::sqrt(t * t / 4.0 - dist);

    let min_x = min_x.floor() as usize;
    let max_x = max_x.ceil() as usize;

    max_x - min_x - 1
}

#[cfg(test)]
mod day06_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day06::test_input();
        let ans = Day06::solve_part_1(input);
        assert_eq!(ans, "288");
    }

    #[test]
    fn test_part_2() {
        let input = Day06::test_input();
        let ans = Day06::solve_part_2(input);
        assert_eq!(ans, "71503");
    }
}
