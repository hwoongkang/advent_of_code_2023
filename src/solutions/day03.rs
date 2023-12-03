use std::collections::HashMap;

use super::Solution;

pub struct Day03;

impl Solution for Day03 {
    fn test_input() -> String {
        String::from(
            "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..",
        )
    }

    fn solve_part_1(input: String) -> String {
        let schematic: EngineSchematic = input.parse().unwrap();
        schematic.valid_numbers().iter().sum::<usize>().to_string()
    }

    fn solve_part_2(input: String) -> String {
        let schematic: EngineSchematic = input.parse().unwrap();
        let mut gears: HashMap<Pos, Vec<usize>> = HashMap::new();
        for part in schematic.symbols.iter() {
            if *part.1 == '*' {
                gears.insert(*part.0, vec![]);
            }
        }
        for number in schematic.numbers.iter() {
            for adj in number.adjacent_positions(schematic.size) {
                if let Some(vec) = gears.get_mut(&adj) {
                    vec.push(number.value);
                }
            }
        }
        gears
            .values()
            .filter_map(|vec| {
                if vec.len() == 2 {
                    Some(vec[0] * vec[1])
                } else {
                    None
                }
            })
            .sum::<usize>()
            .to_string()
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct Pos(usize, usize);

#[derive(Debug)]
struct EngineSchematic {
    size: Pos,
    symbols: HashMap<Pos, char>,
    numbers: Vec<Number>,
}
#[derive(Debug)]
struct Number {
    value: usize,
    pos: Pos,
}

fn num_len(mut num: usize) -> usize {
    let mut ans = 0;
    while num > 0 {
        ans += 1;
        num /= 10;
    }
    ans
}

impl Number {
    fn adjacent_positions(&self, clamp: Pos) -> Vec<Pos> {
        let mut ans = vec![];
        let len = num_len(self.value);

        let c_start = 1.max(self.pos.1) - 1;
        let c_end = (self.pos.1 + len + 1).min(clamp.1);
        // prev row
        if self.pos.0 != 0 {
            let r = self.pos.0 - 1;
            for c in c_start..c_end {
                ans.push(Pos(r, c));
            }
        }
        if self.pos.1 != 0 {
            ans.push(Pos(self.pos.0, self.pos.1 - 1));
        }
        if self.pos.1 + len < clamp.1 {
            ans.push(Pos(self.pos.0, self.pos.1 + len));
        }
        if self.pos.0 <= clamp.0 - 2 {
            let r = self.pos.0 + 1;
            for c in c_start..c_end {
                ans.push(Pos(r, c));
            }
        }

        ans
    }
}

impl EngineSchematic {
    fn valid_numbers(&self) -> Vec<usize> {
        self.numbers
            .iter()
            .filter_map(|num| {
                for adj in num.adjacent_positions(self.size) {
                    if let Some(_) = self.symbols.get(&adj) {
                        return Some(num.value);
                    }
                }
                None
            })
            .collect()
    }
}

impl std::str::FromStr for EngineSchematic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<Vec<char>> = s
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect();
        let size: Pos = Pos(chars.len(), chars[1].len());
        let mut numbers = vec![];
        let mut symbols = HashMap::new();
        let mut num = 0;
        for (r, row) in chars.iter().enumerate() {
            let max_c = row.len();
            let mut c = 0;
            loop {
                let ch = chars[r][c];
                if ch.is_digit(10) {
                    let mut ind = c;
                    loop {
                        let lookahead = ind + 1;
                        let now = chars[r][ind].to_digit(10).unwrap() as usize;
                        num = num * 10 + now;
                        if lookahead >= size.0 || !chars[r][lookahead].is_digit(10) {
                            numbers.push(Number {
                                value: num,
                                pos: Pos(r, c),
                            });
                            num = 0;
                            break;
                        }

                        ind += 1;
                    }
                    c = ind;
                } else if ch == '.' {
                } else {
                    symbols.insert(Pos(r, c), ch);
                }
                c += 1;
                if c >= max_c {
                    break;
                }
            }
        }

        Ok(Self {
            size,
            numbers,
            symbols,
        })
    }
}

#[cfg(test)]
mod day03_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day03::test_input();
        let ans = Day03::solve_part_1(input);
        assert_eq!(ans, "4361");
    }

    #[test]
    fn test_part_2() {
        let input = Day03::test_input();
        let ans = Day03::solve_part_2(input);
        assert_eq!(ans, "467835");
    }
}
