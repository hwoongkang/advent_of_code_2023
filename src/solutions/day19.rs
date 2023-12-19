use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
    str::FromStr,
};

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
        let mut accepted_ranges: Vec<PartRange> = vec![];
        let (workflows, _) = parse_input(input);

        let mut queue: VecDeque<(State, PartRange)> = VecDeque::from(vec![(
            State::Workflow("in".to_string()),
            PartRange {
                x: Range(1, 4001),
                m: Range(1, 4001),
                a: Range(1, 4001),
                s: Range(1, 4001),
            },
        )]);

        while let Some((state, part_range)) = queue.pop_front() {
            match &state {
                State::Terminated(b) => {
                    if *b {
                        accepted_ranges.push(part_range);
                    }
                    continue;
                }
                State::Workflow(name) => {
                    let workflow = workflows.get(name).unwrap();

                    let next_states = workflow.handle(part_range);
                    queue.extend(next_states);
                }
            }
        }
        accepted_ranges
            .iter()
            .map(|range| range.size())
            .sum::<usize>()
            .to_string()
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

#[derive(Copy, Clone, PartialEq, Debug)]
struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartRange {
    fn size(&self) -> usize {
        self.x.size() * self.m.size() * self.a.size() * self.s.size()
    }
    fn get(&self, char: &char) -> &Range {
        match char {
            'x' => &self.x,
            'm' => &self.m,
            'a' => &self.a,
            's' => &self.s,
            _ => unreachable!(),
        }
    }

    fn replaced(&self, char: &char, range: Range) -> Self {
        match char {
            'x' => Self { x: range, ..*self },
            'm' => Self { m: range, ..*self },
            'a' => Self { a: range, ..*self },
            's' => Self { s: range, ..*self },
            _ => unreachable!(),
        }
    }
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
    // get an range of parts
    // and specify which amount of parts goes to where
    fn handle(&self, part_range: PartRange) -> Vec<(State, PartRange)> {
        let mut ans = vec![];
        let mut current_range = part_range;
        for step in self.steps.iter() {
            match step {
                Step::End(state) => ans.push((state.clone(), current_range)),
                Step::Cond((char, predicate, ordering), state) => {
                    let if_range = if ordering == &Ordering::Greater {
                        Range(predicate + 1, 4001)
                    } else {
                        Range(1, *predicate)
                    };
                    let else_range = if ordering == &Ordering::Greater {
                        Range(1, predicate + 1)
                    } else {
                        Range(*predicate, 4001)
                    };

                    if let Some(overlap) = current_range.get(char).overlap(&if_range) {
                        ans.push((state.clone(), current_range.replaced(char, overlap)));
                    }
                    if let Some(overlap) = current_range.get(char).overlap(&else_range) {
                        current_range = current_range.replaced(char, overlap);
                    } else {
                        break;
                    }
                }
            }
        }

        ans
    }

    fn process(&self, part: &Part) -> State {
        for step in self.steps.iter() {
            match step {
                Step::End(state) => return state.clone(),
                Step::Cond(tup, state) => {
                    let f = Step::get_function(*tup);

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
    Cond((char, usize, Ordering), State), // condition and state,
    End(State),
}

impl Step {
    fn get_function(
        (char, predicate, ordering): (char, usize, Ordering),
    ) -> Box<dyn Fn(&Part) -> bool> {
        let f = move |part: &Part| -> bool {
            let part_num = match char {
                'x' => &part.x,
                'm' => &part.m,
                'a' => &part.a,
                's' => &part.s,
                _ => panic!("input wrong"),
            };
            part_num.cmp(&predicate) == ordering
        };
        Box::new(f)
    }
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

            Ok(Self::Cond((ch, predicate, ordering), state))
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Range(usize, usize);

impl Range {
    fn size(&self) -> usize {
        if self.1 <= self.0 {
            0
        } else {
            self.1 - self.0
        }
    }
    fn overlap(&self, rhs: &Range) -> Option<Range> {
        if self.overlaps(rhs) {
            Some(Range(self.0.max(rhs.0), self.1.min(rhs.1)))
        } else {
            None
        }
    }
    fn overlaps(&self, rhs: &Range) -> bool {
        self.0 < rhs.1 && rhs.0 < self.1
    }
    fn _contains(&self, rhs: &Range) -> bool {
        self.0 <= rhs.0 && rhs.1 <= self.1
    }
    fn _slice(&self, rhs: &Range) -> (Option<Range>, Vec<Range>) {
        if !self.overlaps(rhs) {
            (None, vec![*rhs])
        } else if self._contains(rhs) {
            (Some(*rhs), vec![])
        } else if rhs._contains(self) {
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
        assert_eq!(ans, "167409079868000");
    }

    #[test]
    fn test_part_range() {
        let part_range = PartRange {
            x: Range(1, 4001),
            m: Range(1, 4001),
            a: Range(1, 4001),
            s: Range(1, 4001),
        };
        let workflow: Workflow = "in{s<1351:px,qqz}".parse().unwrap();
        let v = workflow.handle(part_range);
        assert_eq!(
            v,
            vec![
                (
                    State::Workflow("px".to_string()),
                    PartRange {
                        x: Range(1, 4001),
                        m: Range(1, 4001),
                        a: Range(1, 4001),
                        s: Range(1, 1351),
                    }
                ),
                (
                    State::Workflow("qqz".to_string()),
                    PartRange {
                        x: Range(1, 4001),
                        m: Range(1, 4001),
                        a: Range(1, 4001),
                        s: Range(1351, 4001),
                    }
                ),
            ]
        );

        let part_range = PartRange {
            x: Range(1, 4001),
            m: Range(1, 4001),
            a: Range(1, 4001),
            s: Range(2001, 4001),
        };
        let workflow: Workflow = "in{s<1351:px,qqz}".parse().unwrap();
        let v = workflow.handle(part_range);
        assert_eq!(
            v,
            vec![(
                State::Workflow("qqz".to_string()),
                PartRange {
                    x: Range(1, 4001),
                    m: Range(1, 4001),
                    a: Range(1, 4001),
                    s: Range(2001, 4001),
                }
            ),]
        );
    }
}
