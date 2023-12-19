use std::{cmp::Ordering, collections::HashMap, rc::Rc, str::FromStr};

use super::Solution;

pub struct Day19;

impl Solution for Day19 {
    fn test_input() -> String {
        String::from(
            "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
        )
    }

    fn solve_part_1(input: String) -> String {
        let (workflows, parts) = parse_input(input);
        let mut ans = 0;
        for part in parts.iter() {
            let mut state = State::Workflow("in".to_string());
            loop {
                match &state {
                    State::Terminated(b) => {
                        if *b {
                            ans += part.rating();
                        }
                        break;
                    }
                    State::Workflow(name) => {
                        let workflow = workflows.get(name).unwrap();

                        let next_state = workflow.process(part);

                        state = next_state;
                    }
                }
            }
        }
        ans.to_string()
    }

    fn solve_part_2(input: String) -> String {
        String::from("0")
    }
}

fn parse_input(input: String) -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut halves = input.split("\n\n");
    let workflows: Vec<Workflow> = halves
        .next()
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let parts = halves
        .next()
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let workflows = workflows
        .into_iter()
        .map(|workflow| (workflow.name.clone(), workflow))
        .collect();

    (workflows, parts)
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<usize> = s[1..(s.len() - 1)]
            .split(",")
            .map(|s| s.split("=").skip(1).next().unwrap().parse().unwrap())
            .collect();
        Ok(Self {
            x: nums[0],
            m: nums[1],
            a: nums[2],
            s: nums[3],
        })
    }
}

struct Workflow {
    name: String,
    steps: Vec<Step>,
}

impl Workflow {
    fn process(&self, part: &Part) -> State {
        for step in self.steps.iter() {
            match step {
                Step::End(state) => return state.clone(),
                Step::Cond(f, state) => {
                    if f(part) {
                        return state.clone();
                    } else {
                        continue;
                    }
                }
            }
        }
        unreachable!("sth wrong")
    }
}

impl FromStr for Workflow {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let mut words = s[..(s.len() - 1)].split("{");
        let name = words.next().unwrap().to_string();
        let steps = words
            .next()
            .unwrap()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        Ok(Self { name, steps })
    }
}

enum Step {
    Cond(Box<dyn Fn(&Part) -> bool>, State), // condition and state,
    End(State),
}

impl FromStr for Step {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split(":").collect();
        if words.len() == 1 {
            Ok(Self::End(s.parse().unwrap()))
        } else {
            let state: State = words[1].parse().unwrap();

            let s = words[0];
            let ch = s.chars().next().unwrap();

            let predicate: usize = s[2..].parse().unwrap();

            let ordering: Ordering = match &s[1..2] {
                ">" => Ordering::Greater,
                "<" => Ordering::Less,
                _ => panic!("input wrong"),
            };

            let f = move |part: &Part| -> bool {
                let part_num = match ch {
                    'x' => &part.x,
                    'm' => &part.m,
                    'a' => &part.a,
                    's' => &part.s,
                    _ => panic!("input wrong"),
                };
                part_num.cmp(&predicate) == ordering
            };
            Ok(Self::Cond(Box::new(f), state))
        }
    }
}

#[derive(Clone, Debug)]
enum State {
    Terminated(bool), // accepted, rejected
    Workflow(String), // to another workflow
}

impl FromStr for State {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "R" => Self::Terminated(false),
            "A" => Self::Terminated(true),
            s => Self::Workflow(s.to_string()),
        })
    }
}

#[cfg(test)]
mod day19_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day19::test_input();
        let ans = Day19::solve_part_1(input);
        assert_eq!(ans, "19114");
    }

    #[test]
    fn test_part_2() {
        let input = Day19::test_input();
        let ans = Day19::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
