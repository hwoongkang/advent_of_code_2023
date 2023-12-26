use std::collections::{HashMap, HashSet};

use super::Solution;

pub struct Day25;

impl Solution for Day25 {
    fn test_input() -> String {
        String::from(
            "jqt: rhn xhk nvd
        rsh: frs pzl lsr
        xhk: hfx
        cmg: qnr nvd lhk bvb
        rhn: xhk bvb hfx
        bvb: xhk hfx
        pzl: lsr hfx nvd
        qnr: nvd
        ntq: jqt hfx bvb xhk
        nvd: lhk
        lsr: lhk
        rzs: qnr cmg lsr rsh
        frs: qnr lhk lsr",
        )
    }

    fn solve_part_1(input: String) -> String {
        let graph = Graph::from(&input);

        graph.part_1().to_string()
    }

    fn solve_part_2(_input: String) -> String {
        String::from("0")
    }
}
#[derive(Debug, Clone, PartialEq)]
struct Node {
    name: String,
    edges: Vec<(String, usize)>,
}

impl Node {
    fn from(str: &str) -> Self {
        Self {
            name: str.to_string(),
            edges: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Graph {
    nodes: HashMap<String, Node>,
}

impl Graph {
    fn merge(&mut self, a: &str, b: &str) {
        self.nodes.remove(b);
        let mut updated: Vec<(String, usize)> = vec![];
        for node in self.nodes.values_mut().filter(|n| n.name != a) {
            let mut a_cost: Option<usize> = None;
            let mut b_cost: Option<usize> = None;
            for (name, cost) in node.edges.iter() {
                if name == a {
                    a_cost = Some(*cost);
                }
                if name == b {
                    b_cost = Some(*cost);
                }
            }
            let cost = match (a_cost, b_cost) {
                (None, None) => {
                    None //do nothing
                }
                (Some(a_cost), None) => {
                    Some(a_cost) // do nothing
                }
                (None, Some(b_cost)) => {
                    node.edges = node
                        .edges
                        .drain(..)
                        .map(|(name, cost)| {
                            if name == b {
                                (a.to_string(), b_cost)
                            } else {
                                (name, cost)
                            }
                        })
                        .collect();
                    Some(b_cost)
                }
                (Some(a_cost), Some(b_cost)) => {
                    let new_cost = a_cost + b_cost;

                    node.edges = node
                        .edges
                        .drain(..)
                        .filter_map(|(name, cost)| {
                            if name == b {
                                None
                            } else if name == a {
                                Some((name, new_cost))
                            } else {
                                Some((name, cost))
                            }
                        })
                        .collect();
                    Some(new_cost)
                }
            };
            if let Some(cost) = cost {
                updated.push((node.name.clone(), cost));
            }
        }
        let node_a = self.nodes.get_mut(a).unwrap();
        node_a.edges = updated;
    }

    // min_cut, deleted
    fn min_cut_phase(&mut self, a: &str) -> (usize, (String, String)) {
        let mut last = ("".to_string(), a.to_string());

        let mut partition = HashSet::from([a.to_string()]);

        while partition.len() < self.nodes.len() {
            let candidates = self
                .nodes
                .values()
                .filter(|node| partition.get(&node.name).is_none());
            let with_tightness = candidates.map(|node| {
                let tightness = node
                    .edges
                    .iter()
                    .filter_map(|(name, cost)| {
                        if partition.get(name).is_some() {
                            Some(cost)
                        } else {
                            None
                        }
                    })
                    .sum::<usize>();
                (node, tightness)
            });
            let node_to_add = with_tightness.max_by(|a, b| a.1.cmp(&b.1)).unwrap();
            // let candidates: Vec<&Node> = self.nodes.values().filter_map(|node| {}).collect();

            partition.insert(node_to_add.0.name.to_string());
            last = (last.1, node_to_add.0.name.clone());
        }

        let ans = self
            .nodes
            .get(&last.1)
            .unwrap()
            .edges
            .iter()
            .map(|(_, cost)| *cost)
            .sum::<usize>();
        self.merge(&last.0, &last.1);
        (ans, last)
    }

    // minimum_cut, partition_size
    fn minimum_cut(&self, node_name: &str) -> (usize, usize) {
        let mut graph = self.clone();
        // min cut, partition size at that time
        let mut min_cut = (usize::MAX, 0);
        let mut order: HashMap<String, usize> =
            self.nodes.keys().map(|key| (key.clone(), 1)).collect();
        let size = self.nodes.len();

        while graph.nodes.len() > 1 {
            if graph.nodes.len() % (size / 10) == 0 {
                println!("{}%", 100 * graph.nodes.len() / size)
            }

            let (curr, (s, t)) = graph.min_cut_phase(&node_name);

            let deleted = order.remove(&t).unwrap();

            *order.get_mut(&s).unwrap() += deleted;

            if curr < min_cut.0 {
                min_cut = (curr, deleted);
            }
        }
        min_cut
    }
    fn part_1(&self) -> usize {
        // if min cut is 3: multiply the sizes of the two partition
        let pivot = self.nodes.keys().next().unwrap();

        let (min_cut, node_order) = self.minimum_cut(pivot);
        println!("min_cut was: {}", min_cut);
        println!("pivot was: {}", pivot);
        let n = self.nodes.len();
        (n - node_order) * node_order
    }
    fn from(str: &str) -> Self {
        let mut nodes: HashMap<String, Node> = HashMap::new();
        for line in str.lines() {
            let line = line.trim();
            let mut words = line.split(":");
            let me = words.next().unwrap().trim();
            let neighbors: Vec<&str> = words.next().unwrap().trim().split_whitespace().collect();
            let node = if let Some(n) = nodes.get_mut(me) {
                n
            } else {
                nodes.insert(me.to_string(), Node::from(me));
                nodes.get_mut(me).unwrap()
            };

            for neighbor in neighbors.iter() {
                node.edges.push((neighbor.to_string(), 1));
            }
            for neighbor in neighbors {
                if let Some(n) = nodes.get_mut(neighbor) {
                    n.edges.push((me.to_string(), 1));
                } else {
                    nodes.insert(
                        neighbor.to_string(),
                        Node {
                            name: neighbor.to_string(),
                            edges: vec![(me.to_string(), 1)],
                        },
                    );
                }
            }
        }
        Self { nodes }
    }
}
#[cfg(test)]
mod day25_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day25::test_input();
        let ans = Day25::solve_part_1(input);
        assert_eq!(ans, "54");
    }

    #[test]
    fn test_part_2() {
        let input = Day25::test_input();
        let ans = Day25::solve_part_2(input);
        assert_eq!(ans, "");
    }

    #[test]
    fn test_merge() {
        let input = "a: b c
        b: c d
        c: d";
        let mut graph = Graph::from(input);

        graph.merge("b", "c");
        assert_eq!(
            graph.nodes.get("a"),
            Some(&Node {
                name: "a".to_string(),
                edges: vec![("b".to_string(), 2)]
            })
        );
        assert_eq!(
            graph.nodes.get("d"),
            Some(&Node {
                name: "d".to_string(),
                edges: vec![("b".to_string(), 2)]
            })
        );
        assert!(
            graph.nodes.get("b")
                == Some(&Node {
                    name: "b".to_string(),
                    edges: vec![("a".to_string(), 2), ("d".to_string(), 2)]
                })
                || graph.nodes.get("b")
                    == Some(&Node {
                        name: "b".to_string(),
                        edges: vec![("d".to_string(), 2), ("a".to_string(), 2)]
                    })
        );
        assert!(graph.nodes.get("c").is_none())
    }

    #[test]
    fn test_min_cut() {
        let a = Node {
            name: "a".to_string(),
            edges: vec![
                ("b".to_string(), 4),
                ("s".to_string(), 2),
                ("t".to_string(), 3),
            ],
        };
        let b = Node {
            name: "b".to_string(),
            edges: vec![("a".to_string(), 4), ("s".to_string(), 3)],
        };
        let s = Node {
            name: "s".to_string(),
            edges: vec![
                ("a".to_string(), 2),
                ("b".to_string(), 3),
                ("t".to_string(), 1),
            ],
        };
        let t = Node {
            name: "t".to_string(),
            edges: vec![("a".to_string(), 3), ("s".to_string(), 1)],
        };
        let graph = Graph {
            nodes: HashMap::from([
                ("a".to_string(), a),
                ("b".to_string(), b),
                ("s".to_string(), s),
                ("t".to_string(), t),
            ]),
        };
        assert_eq!(graph.part_1(), 3)
    }
}
