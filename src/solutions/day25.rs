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

        String::from("")
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
            node.edges = node
                .edges
                .drain(..)
                .filter_map(|(name, cost)| {
                    if name == b {
                        None
                    } else if name == a {
                        let mut cost = a_cost.unwrap();
                        if let Some(b_cost) = b_cost {
                            cost += b_cost;
                        }
                        updated.push((node.name.clone(), cost));
                        Some((name, cost))
                    } else {
                        Some((name, cost))
                    }
                })
                .collect();
        }
        let node_a = self.nodes.get_mut(a).unwrap();
        node_a.edges = updated;
    }

    // min_cut, partition_size
    fn min_cut_phase(&mut self, node_name: &str) -> (usize, usize) {
        let mut last = ("".to_string(), node_name.to_string());

        let mut partition = HashSet::from([node_name.to_string()]);
        while partition.len() < self.nodes.len() {
            let mut node_to_add = (0, "".to_string());
            for n in partition.iter() {
                let node = self.nodes.get(n).unwrap();
                for (to, cost) in node.edges.iter() {
                    if partition.get(to).is_none() && *cost > node_to_add.0 {
                        node_to_add = (*cost, to.clone());
                    }
                }
            }
            partition.insert(node_to_add.1.to_string());
            last = (last.1, node_to_add.1);
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
        (ans, partition.len())
    }
    fn part_1(&self) -> usize {
        // if min cut is 3: multiply the sizes of the two partition
        let mut min_cut = usize::MAX;
        let mut ans = (0, 0);
        fn minimum_cut_phase(graph: &mut Graph, node_name: &str) -> usize {
            let mut partition = HashSet::from([node_name]);

            for n in partition.iter() {
                let node = graph.nodes.get(*n).unwrap();
            }
            0
        }
        0
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
    fn test_min_cut_phase() {
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
        let mut graph = Graph {
            nodes: HashMap::from([
                ("a".to_string(), a),
                ("b".to_string(), b),
                ("s".to_string(), s),
                ("t".to_string(), t),
            ]),
        };
        println!("{:?}", graph);
        let min_cut = graph.min_cut_phase("a");
        println!("{:?}", graph);
        assert_eq!(min_cut, (4, 4));
        let min_cut = graph.min_cut_phase("a");
        assert_eq!(min_cut, (7, 3));
        let min_cut = graph.min_cut_phase("a");
        assert_eq!(min_cut, (9, 2));
    }
}
