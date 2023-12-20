use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use super::Solution;

pub struct Day20;

impl Solution for Day20 {
    fn test_input() -> String {
        String::from(
            "broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a",
        )
    }

    fn solve_part_1(input: String) -> String {
        let mut circuit: Circuit = input.parse().unwrap();
        let (mut highs, mut lows) = (0, 0);
        for _ in 0..1000 {
            let (h, l) = circuit.click();
            highs += h;
            lows += l;
        }
        (highs * lows).to_string()
    }

    fn solve_part_2(_input: String) -> String {
        // pz: 100011010111
        // mh: 110010111111
        // rn: 110000101111
        // jt: 111100101111
        let ans = [
            "100011010111",
            "110010111111",
            "110000101111",
            "111100101111",
        ]
        .iter()
        .map(|str| -> String { str.chars().rev().collect() })
        .map(|str| usize::from_str_radix(&str, 2).unwrap())
        .map(|num| num)
        .fold(1, |acc, now| lcm(acc, now));
        (ans).to_string()
    }
}

type Pulse = bool;
const HIGH: Pulse = true;
const LOW: Pulse = false;

#[derive(Debug, PartialEq, Clone)]
struct Broadcaster {
    outputs: Vec<String>,
}

impl FromStr for Broadcaster {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let outputs = s.trim().split("->").skip(1).next().unwrap();
        let outputs: Vec<String> = outputs.split(",").map(|w| w.trim().to_string()).collect();
        Ok(Self { outputs })
    }
}

impl Broadcaster {
    fn propagate(&self) -> Pulse {
        LOW
    }
}

#[derive(Debug, PartialEq, Clone)]
struct FlipFlop {
    name: String,
    outputs: Vec<String>,
    state: Pulse,
}

impl FromStr for FlipFlop {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split("->");
        let name = words.next().unwrap().trim()[1..].to_string();
        let outputs = words
            .next()
            .unwrap()
            .split(",")
            .map(|w| w.trim().to_string())
            .collect();
        Ok(Self {
            name,
            outputs,
            state: LOW,
        })
    }
}
impl FlipFlop {
    fn propagate(&mut self, pulse: Pulse) -> Option<Pulse> {
        if pulse == HIGH {
            None
        } else {
            self.state = !self.state;
            Some(self.state)
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Conjunction {
    name: String,
    outputs: Vec<String>,
    state: HashMap<String, Pulse>,
}

impl FromStr for Conjunction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split("->");
        let name = words.next().unwrap().trim()[1..].to_string();
        let outputs = words
            .next()
            .unwrap()
            .split(",")
            .map(|w| w.trim().to_string())
            .collect();
        Ok(Self {
            name,
            outputs,
            state: HashMap::new(),
        })
    }
}

impl Conjunction {
    fn add_input(&mut self, input: &str) {
        self.state.insert(input.to_string(), LOW);
    }

    fn propagate(&mut self, input: &str, pulse: Pulse) -> Pulse {
        *self.state.get_mut(input).unwrap() = pulse;
        if self.state.values().fold(HIGH, |acc, now| acc && *now) {
            LOW
        } else {
            HIGH
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Module {
    Broadcaster(Broadcaster),
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

impl FromStr for Module {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("%") {
            Ok(Self::FlipFlop(s.parse().unwrap()))
        } else if s.starts_with("&") {
            Ok(Self::Conjunction(s.parse().unwrap()))
        } else {
            Ok(Self::Broadcaster(s.parse().unwrap()))
        }
    }
}

impl Module {
    fn propagate(&mut self, input: &str, pulse: Pulse) -> Option<Pulse> {
        match self {
            Self::Broadcaster(b) => Some(b.propagate()),
            Self::FlipFlop(f) => f.propagate(pulse),
            Self::Conjunction(c) => Some(c.propagate(input, pulse)),
        }
    }

    fn _reset(&mut self) {
        match self {
            Self::FlipFlop(f) => f.state = LOW,
            Self::Conjunction(c) => {
                for v in c.state.values_mut() {
                    *v = LOW;
                }
            }
            _ => {}
        }
    }
}

impl<'a> Module {
    fn get_name(&self) -> String {
        match self {
            Self::Broadcaster(_) => "broadcaster".to_string(),
            Self::FlipFlop(f) => f.name.clone(),
            Self::Conjunction(c) => c.name.clone(),
        }
    }
    fn get_outputs(&'a self) -> &'a [String] {
        match self {
            Self::Broadcaster(b) => &b.outputs,
            Self::FlipFlop(f) => &f.outputs,
            Self::Conjunction(c) => &c.outputs,
        }
    }
}

#[derive(Clone)]
struct Circuit {
    modules: HashMap<String, Module>,
}

impl FromStr for Circuit {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut modules = HashMap::new();
        for line in s.lines() {
            let module: Module = line.trim().parse().unwrap();
            modules.insert(module.get_name(), module);
        }

        let rel = modules
            .iter()
            .map(|(k, v)| (k.clone(), v.get_outputs().to_owned()))
            .collect::<Vec<_>>();

        for (input, children) in rel {
            for child_name in children {
                if let Some(child) = modules.get_mut(&child_name) {
                    if let Module::Conjunction(c) = child {
                        c.add_input(&input);
                    }
                }
            }
        }
        Ok(Self { modules })
    }
}

impl Circuit {
    fn click(&mut self) -> (usize, usize) {
        let mut highs = 0;
        let mut lows = 0;

        let from = "".to_string();
        let to = "broadcaster".to_string();
        let signal = LOW;

        let mut queue = VecDeque::from([(from, to, signal)]);

        while let Some(pulse) = queue.pop_front() {
            let (from, to, signal) = pulse;
            if signal {
                highs += 1;
            } else {
                lows += 1;
            }

            let Some(module) = self.modules.get_mut(&to)else {
                continue
            };

            if let Some(signal) = module.propagate(&from, signal) {
                for to in module.get_outputs() {
                    let from = module.get_name();
                    queue.push_back((from, to.to_string(), signal));
                }
            }
        }
        (highs, lows)
    }

    fn _reset(&mut self) {
        for module in self.modules.values_mut() {
            module._reset();
        }
    }

    fn _check(&self, name: &str) -> bool {
        let Some(module) = self.modules.get(name) else {
            panic!("does not exist")
        };
        match module {
            Module::Broadcaster(_) => true,
            Module::FlipFlop(f) => f.state == LOW,
            Module::Conjunction(c) => c
                .state
                .values() // LOW, LOW, LOW
                .map(|b| !b) // true, true, true
                .fold(true, |acc, now| acc && now), //true
        }
    }

    fn _get_cycle(&mut self, name: &str) -> usize {
        let Some(module) = self.modules.get(name) else {
            panic!("does not exist")
        };
        match module {
            Module::Broadcaster(_) => 1,
            Module::Conjunction(c) => c
                .state
                .keys()
                .map(|input| {
                    let mut me = self.clone();
                    me._reset();
                    me._get_cycle(input)
                })
                .fold(1, |acc, now| lcm(acc, now)),
            Module::FlipFlop(f) => {
                let mut me = self.clone();
                me._reset();
                let mut count = 0;
                loop {
                    count += 1;
                    me.click();
                    if me._check(&f.name) {
                        break count;
                    }
                }
            }
        }
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if a > b {
        gcd(b, a)
    } else if b % a == 0 {
        a
    } else {
        gcd(b % a, a)
    }
}
fn lcm(a: usize, b: usize) -> usize {
    let d = gcd(a, b);
    a * b / d
}

#[cfg(test)]
mod day20_tests {
    use super::*;

    fn test_input_complex() -> String {
        String::from(
            "broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output",
        )
    }

    #[test]
    fn test_part_1() {
        let input = Day20::test_input();
        let ans = Day20::solve_part_1(input);
        assert_eq!(ans, "32000000");

        let input = test_input_complex();
        let ans = Day20::solve_part_1(input);
        assert_eq!(ans, "11687500");
    }

    #[test]
    fn test_broadcast() {
        let input = "broadcaster -> a, b, c";
        let broadcast = input.parse::<Broadcaster>();
        assert_eq!(
            broadcast,
            Ok(Broadcaster {
                outputs: vec!["a".to_string(), "b".to_string(), "c".to_string()]
            })
        );
        let broadcast = broadcast.unwrap();
        assert_eq!(broadcast.propagate(), LOW);
        assert_eq!(broadcast.propagate(), LOW);
    }

    #[test]
    fn test_flip_flop() {
        let input = "%a -> inv, con";
        let flipflop = FlipFlop::from_str(input);

        let outputs = vec!["inv".to_string(), "con".to_string()];

        assert_eq!(
            flipflop,
            Ok(FlipFlop {
                name: "a".to_string(),
                outputs: outputs.clone(),
                state: LOW
            })
        );
        let mut flipflop = flipflop.unwrap();

        assert_eq!(flipflop.propagate(HIGH), None);
        assert_eq!(flipflop.propagate(LOW), Some(HIGH));
        assert_eq!(flipflop.propagate(HIGH), None);
        assert_eq!(flipflop.propagate(LOW), Some(LOW));
    }

    #[test]
    fn test_conjunction() {
        let input = "&con -> output";
        let con = Conjunction::from_str(input);

        assert_eq!(
            con,
            Ok(Conjunction {
                name: "con".to_string(),
                outputs: vec!["output".to_string()],
                state: HashMap::new(),
            })
        );

        let mut con = con.unwrap();

        con.add_input("a");
        assert_eq!(con.propagate("a", HIGH), LOW);
        assert_eq!(con.propagate("a", LOW), HIGH);

        con.add_input("b");
        assert_eq!(con.propagate("a", HIGH), HIGH);
        assert_eq!(con.propagate("b", HIGH), LOW);
    }
}
