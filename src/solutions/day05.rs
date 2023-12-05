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
        let mut lines = input.lines();
        let mut first_line = lines.next().unwrap().split_whitespace().skip(1);
        let mut seeds: Vec<Range> = vec![];
        while let Some(start) = first_line.next() {
            let start = start.parse().unwrap();
            let range: usize = first_line.next().unwrap().parse().unwrap();
            seeds.push(Range(start, start + range));
        }

        let almanacs: Vec<Almanac> = (0..7).map(|_| Almanac::from(&mut lines)).collect();

        for almanac in almanacs {
            seeds = almanac.apply_to(seeds);
        }
        seeds.iter().map(|rng| rng.0).min().unwrap().to_string()
    }
}

#[derive(Debug)]
struct Mapping {
    src: usize,
    dst: usize,
    rng: usize,
}
#[derive(Debug)]
struct Almanac {
    mappings: Vec<Mapping>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Range(usize, usize);

impl Range {
    fn overlaps(&self, rhs: &Range) -> bool {
        self.0 <= rhs.1 && rhs.0 <= self.1
    }
    fn contains(&self, rhs: &Range) -> bool {
        self.0 <= rhs.0 && rhs.1 <= self.1
    }
    fn slice(&self, rhs: &Range) -> (Option<Range>, Vec<Range>) {
        if !self.overlaps(rhs) {
            (None, vec![*rhs])
        } else if self.contains(rhs) {
            (Some(*rhs), vec![])
        } else if rhs.contains(self) {
            (
                Some(*self),
                vec![Range(rhs.0, self.0), Range(self.1, rhs.1)],
            )
        } else {
            // (1, 4) (3, 8) => (3, 4), (4, 8)
            // (3, 8) ,(1, 4) => (3, 4), (1, 3)
            if rhs.0 > self.0 {
                (Some(Range(rhs.0, self.1)), vec![Range(self.1, rhs.1)])
            } else {
                (Some(Range(self.0, rhs.1)), vec![Range(rhs.0, self.0)])
            }
        }
    }
}

impl Almanac {
    fn apply_to(&self, range: Vec<Range>) -> Vec<Range> {
        let mut range = range.clone();
        let mut applied: Vec<Range> = vec![];
        for mapping in self.mappings.iter() {
            let mut new_range: Vec<Range> = vec![];
            for r in range.iter() {
                let (sliced, mut remaining) =
                    Range(mapping.src, mapping.src + mapping.rng).slice(r);
                if let Some(s) = sliced {
                    applied.push(Range(
                        s.0 + mapping.dst - mapping.src,
                        s.1 + mapping.dst - mapping.src,
                    ));
                }
                new_range.append(&mut remaining);
            }
            range = new_range
        }
        applied.append(&mut range);
        applied
    }
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
    fn test_apply_to() {
        let almanac = Almanac {
            mappings: vec![
                Mapping {
                    src: 0,
                    dst: 14,
                    rng: 7,
                },
                Mapping {
                    src: 10,
                    dst: 31,
                    rng: 8,
                },
            ],
        };
        let range = vec![Range(5, 13), Range(46, 81)];
        assert_eq!(
            almanac.apply_to(range),
            vec![Range(19, 21), Range(31, 34), Range(7, 10), Range(46, 81),]
        );
    }

    #[test]
    fn test_slice() {
        // no overlap
        let lhs = Range(0, 2);
        let rhs = Range(5, 10);
        assert_eq!(lhs.slice(&rhs), (None, vec![Range(5, 10)]));

        // contained
        let lhs = Range(0, 200);
        let rhs = Range(5, 10);
        assert_eq!(lhs.slice(&rhs), (Some(Range(5, 10)), vec![]));

        // predicate is contained
        let lhs = Range(10, 50);
        let rhs = Range(0, 200);
        assert_eq!(
            lhs.slice(&rhs),
            (Some(Range(10, 50)), vec![Range(0, 10), Range(50, 200)])
        );

        // overlaps at the end
        let lhs = Range(3, 4);
        let rhs = Range(4, 10);
        assert_eq!(lhs.slice(&rhs), (Some(Range(4, 4)), vec![Range(4, 10)]));

        // lhs first
        let lhs = Range(3, 40);
        let rhs = Range(14, 100);
        assert_eq!(lhs.slice(&rhs), (Some(Range(14, 40)), vec![Range(40, 100)]));

        // rhs first
        let lhs = Range(14, 100);
        let rhs = Range(3, 40);
        assert_eq!(lhs.slice(&rhs), (Some(Range(14, 40)), vec![Range(3, 14)]))
    }

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
        assert_eq!(ans, "46");
    }
}
