use super::Solution;

pub struct Day02;

impl Solution for Day02 {
    fn test_input() -> String {
        String::from(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        )
    }

    fn solve_part_1(input: String) -> String {
        let predicate = Game {
            id: usize::MAX,
            red: 12,
            green: 13,
            blue: 14,
        };
        input
            .lines()
            .map(|line| line.parse::<Game>().unwrap())
            .filter_map(|game| {
                if game.red <= predicate.red
                    && game.green <= predicate.green
                    && game.blue <= predicate.blue
                {
                    Some(game.id)
                } else {
                    None
                }
            })
            .sum::<usize>()
            .to_string()
    }

    fn solve_part_2(_input: String) -> String {
        String::from("0")
    }
}

struct Game {
    id: usize,
    red: usize,
    green: usize,
    blue: usize,
}

impl std::str::FromStr for Game {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut colon = line.split(":");
        let part_game = colon.next().unwrap();
        let game_id: usize = part_game
            .split_whitespace()
            .skip(1)
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let part_draw = colon.next().unwrap();
        let draws: Vec<Draw> = part_draw
            .split(";")
            .map(|word| word.trim().parse().unwrap())
            .collect();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for draw in draws {
            red = red.max(draw.red);
            green = green.max(draw.green);
            blue = blue.max(draw.blue);
        }
        Ok(Self {
            id: game_id,
            red,
            green,
            blue,
        })
    }
}

struct Draw {
    red: usize,
    green: usize,
    blue: usize,
}

impl std::str::FromStr for Draw {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let balls = s.split(",");
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for ball in balls {
            let mut words = ball.trim().split_whitespace();
            let num: usize = words.next().unwrap().parse().unwrap();
            let color = words.next().unwrap().trim();
            match color {
                "red" => {
                    red = num;
                }
                "blue" => {
                    blue = num;
                }
                "green" => {
                    green = num;
                }
                _ => unreachable!(),
            }
        }
        Ok(Self { red, green, blue })
    }
}

#[cfg(test)]
mod day02_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day02::test_input();
        let ans = Day02::solve_part_1(input);
        assert_eq!(ans, "8");
    }

    #[test]
    fn test_part_2() {
        let input = Day02::test_input();
        let ans = Day02::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
