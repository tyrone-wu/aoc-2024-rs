use std::io::Write;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    fs::File,
};

use petgraph::{dot::Dot, Graph};

advent_of_code::solution!(24);

#[derive(Debug, Clone)]
enum Op {
    And,
    Or,
    Xor,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Op::And => "AND",
                Op::Or => "OR",
                Op::Xor => "XOR",
            }
        )
    }
}

#[derive(Debug, Clone)]
struct Gate<'a> {
    lhs: &'a str,
    op: (Op, &'a str),
    rhs: &'a str,
    res: &'a str,
}

impl<'a> Gate<'a> {
    fn new(input: &'a str) -> Self {
        let split: Vec<&str> = input.split_whitespace().collect();
        Self {
            lhs: split[0],
            op: (
                match split[1] {
                    "AND" => Op::And,
                    "OR" => Op::Or,
                    "XOR" => Op::Xor,
                    _ => unreachable!(),
                },
                split[1],
            ),
            rhs: split[2],
            res: split[4],
        }
    }
}

#[derive(Debug, Clone)]
struct Gates<'a>(Vec<Gate<'a>>);

impl<'a> Gates<'a> {
    fn new(input: &'a str) -> Self {
        Self(input.lines().map(|line| Gate::new(line)).collect())
    }

    // fn swap(&mut self, i: usize, j: usize) {
    //     let tmp = self.0[i].res;
    //     self.0[i].res = self.0[j].res;
    //     self.0[j].res = tmp;
    // }
}

#[derive(Clone)]
struct Wires(HashMap<String, bool>);

impl Wires {
    fn new(input: &str) -> Self {
        Self(input.lines().fold(HashMap::new(), |mut acc, line| {
            acc.insert(line[..3].to_owned(), &line[5..] == "1");
            acc
        }))
    }

    fn try_gate(&mut self, gate: &Gate) -> bool {
        let Gate { lhs, op, rhs, res } = gate;
        match (self.0.get(*lhs), self.0.get(*rhs)) {
            (None, _) | (_, None) => false,
            (Some(lhs), Some(rhs)) => {
                let out = match op.0 {
                    Op::And => lhs & rhs,
                    Op::Or => lhs | rhs,
                    Op::Xor => lhs ^ rhs,
                };
                self.0.insert(res.to_string(), out);
                true
            }
        }
    }

    fn wire_number(&self, target: &str) -> u64 {
        self.0.iter().fold(0, |mut acc, (w, b)| {
            if &w[..1] == target {
                acc = acc | ((*b as u64) << w[1..].parse::<u64>().unwrap());
            }
            acc
        })
    }

    fn run(&mut self, gates: &Gates) {
        let mut queue = VecDeque::from(gates.0.to_owned());
        while let Some(gate) = queue.pop_front() {
            if !self.try_gate(&gate) {
                queue.push_back(gate);
            }
        }
    }

    // fn set(&mut self, w: char, mut num: u64, max_bits: u64) {
    //     for i in 0..=max_bits {
    //         let b = num & 1;
    //         self.0.insert(format!("{w}{:0>2}", i), b == 1);
    //         num >>= 1;
    //     }
    // }

    // fn yay(&mut self, max_bits: u64, gates: &Gates) -> bool {
    //     for x in 0..=(1 << max_bits) - 1 {
    //         for y in 0..=(1 << max_bits) - 1 {
    //             self.set('x', x, max_bits);
    //             self.set('y', y, max_bits);
    //             self.run(gates);
    //             let x = self.wire_number("x");
    //             let y = self.wire_number("y");
    //             let z = self.wire_number("z");
    //             if x + y != z {
    //                 return false;
    //             }
    //         }
    //     }
    //     true
    // }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut split = input.split("\n\n");
    let mut wires = Wires::new(split.next().unwrap());
    let gates = Gates::new(split.next().unwrap());
    wires.run(&gates);
    Some(wires.wire_number("z"))
}

pub fn part_two(input: &str) -> Option<String> {
    let mut split = input.split("\n\n");
    let mut _wires = Wires::new(split.next().unwrap());
    let gates = Gates::new(split.next().unwrap());
    // let max_bits = wires.0.iter().map(|w| w.0[1..].parse::<u64>().unwrap()).max().unwrap();
    // for combo in (0..gates.0.len()).permutations(8) {
    //     let mut g = gates.clone();
    //     g.swap(combo[0], combo[1]);
    //     g.swap(combo[2], combo[3]);
    //     g.swap(combo[4], combo[5]);
    //     g.swap(combo[6], combo[7]);
    //     if wires.yay(max_bits, &g) {
    //         let mut w: Vec<&str> = combo.iter().map(|i| gates.0[*i].res).collect();
    //         w.sort();
    //         return Some(w.join(","));
    //     }
    // }
    let (graph, _) = gates.0.iter().fold(
        (Graph::<&str, &str>::new(), HashMap::new()),
        |(mut graph, mut nodes), gate| {
            let Gate {
                lhs,
                op: (_, op),
                rhs,
                res,
            } = gate;
            let lhs = if let Some(node) = nodes.get(lhs) {
                *node
            } else {
                let node = graph.add_node(lhs);
                nodes.insert(lhs, node);
                node
            };
            let rhs = if let Some(node) = nodes.get(rhs) {
                *node
            } else {
                let node = graph.add_node(rhs);
                nodes.insert(rhs, node);
                node
            };
            let res = if let Some(node) = nodes.get(res) {
                *node
            } else {
                let node = graph.add_node(res);
                nodes.insert(res, node);
                node
            };
            graph.add_edge(lhs, res, op);
            graph.add_edge(rhs, res, op);
            (graph, nodes)
        },
    );
    let dot = Dot::with_config(&graph, &[]);
    let mut file = File::create("day24_graph.dot").unwrap();
    write!(file, "{:?}", dot).unwrap();
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.as_deref(), Some("z00,z01,z02,z05"));
    }
}
