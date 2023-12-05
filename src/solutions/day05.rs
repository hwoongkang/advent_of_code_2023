use super::Solution;

pub struct Day05;

impl Solution for Day05 {
    fn test_input() -> String {
        String::from(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        )
    }

    fn solve_part_1(input: String) -> String {
        let mut lines = input.lines();
        let seeds: Vec<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|w| w.trim().parse().unwrap())
            .collect();
        let almanacs: Vec<Almanac> = (0..7).map(|_| Almanac::from(&mut lines)).collect();

        seeds
            .iter()
            .map(|seed| {
                let mut s = *seed;
                for almanac in almanacs.iter() {
                    s = almanac.apply(s);
                }
                s
            })
            .min()
            .unwrap()
            .to_string()
    }

    fn solve_part_2(input: String) -> String {
        String::from("0")
    }
}

struct Mapping {
    src: usize,
    dst: usize,
    rng: usize,
}

struct Almanac {
    mappings: Vec<Mapping>,
}

impl Almanac {
    fn apply(&self, src: usize) -> usize {
        for mapping in self.mappings.iter() {
            if mapping.src <= src && src < mapping.src + mapping.rng {
                return mapping.dst + (src - mapping.src);
            }
        }
        src
    }
    fn from(lines: &mut std::str::Lines) -> Self {
        // find first line
        loop {
            let first_line = lines.next();
            match first_line {
                None => unreachable!(),
                Some(line) => {
                    if line.ends_with(":") {
                        break;
                    }
                }
            }
        }
        let mut mappings = vec![];
        loop {
            let line = lines.next();

            match line {
                None => break,
                Some("") => break,
                Some(line) => {
                    let mut line = line.trim().split_whitespace();
                    let dst = line.next().unwrap().trim().parse().unwrap();
                    let src = line.next().unwrap().trim().parse().unwrap();
                    let rng = line.next().unwrap().trim().parse().unwrap();
                    mappings.push(Mapping { src, dst, rng })
                }
            }
        }
        Self { mappings }
    }
}

#[cfg(test)]
mod day05_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day05::test_input();
        let ans = Day05::solve_part_1(input);
        assert_eq!(ans, "35");
    }

    #[test]
    fn test_part_2() {
        let input = Day05::test_input();
        let ans = Day05::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
