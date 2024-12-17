use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(17);

#[derive(Debug, Clone)]
struct Computer {
    a: u64,
    b: u64,
    c: u64,
    prog: Vec<u64>,
    insn_ptr: usize,
    out: Vec<u64>,
}

impl Computer {
    fn parse_input(input: &str) -> Self {
        let mut split = input.split("\n\n");
        let regs_split: Vec<&str> = split.next().unwrap().split_whitespace().collect();
        let a = regs_split[2].parse::<u64>().unwrap();
        let b = regs_split[5].parse::<u64>().unwrap();
        let c = regs_split[8].parse::<u64>().unwrap();
        let mut prog_split = split.next().unwrap().split_whitespace();
        prog_split.next();
        let prog: Vec<u64> = prog_split
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        Self {
            a,
            b,
            c,
            prog,
            insn_ptr: 0,
            out: vec![],
        }
    }

    fn combo_operand(&self) -> u64 {
        let operand = self.prog[self.insn_ptr + 1];
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            _ => self.c,
            // 6 => self.c,
            // 7 => unimplemented!(),
            // _ => unreachable!(),
        }
    }

    fn run_instruction(&mut self) {
        let opcode = self.prog[self.insn_ptr];
        let literal_operand = self.prog[self.insn_ptr + 1];
        let combo_operand = self.combo_operand();
        match opcode {
            0 => self.a = self.a >> combo_operand,
            1 => self.b = self.b ^ literal_operand,
            2 => self.b = combo_operand & 0b111,
            3 => {
                if self.a != 0 {
                    self.insn_ptr = literal_operand as usize;
                    return;
                }
            }
            4 => self.b = self.b ^ self.c,
            5 => self.out.push(combo_operand & 0b111),
            6 => self.b = self.a >> combo_operand,
            _ => self.c = self.a >> combo_operand,
            // 7 => self.c = self.a >> combo_operand,
            // _ => unreachable!(),
        }
        self.insn_ptr += 2;
    }

    fn run(&mut self) {
        while self.insn_ptr < self.prog.len() {
            self.run_instruction();
        }
    }

    fn out_format(&self) -> String {
        self.out
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    // fn reset(&mut self, a: u64) {
    //     self.a = a;
    //     self.b = 0;
    //     self.c = 0;
    //     self.insn_ptr = 0;
    //     self.out.clear();
    // }

    // fn run_p2(&mut self, out_len: usize) -> Option<u64> {
    //     while self.insn_ptr < self.prog.len() {
    //         self.run_instruction();
    //         if self.out.len() == out_len {
    //             return Some(self.out[0]);
    //         }
    //     }
    //     None
    // }

    // fn run_test(&mut self) -> bool {
    //     while self.insn_ptr < self.prog.len() {
    //         self.run_instruction();
    //         if !self.prog.starts_with(&self.out) {
    //             return false;
    //         }
    //     }
    //     true
    // }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut comp = Computer::parse_input(input);
    comp.run();
    Some(comp.out_format())
}

// Program: 0,3,5,4,3,0
// a = a >> 3
// out a
// repeat if a != 0
// 000 011 100 101 011 000 000 = 117440
// 0   3   4   5   3   0   end

// simplified prog don't need b or c
// out ((a & 0b111) ^ 4 ^ (a >> ((a & 0b111) ^ 1))) & 0b111
// a = a >> 3
// repeat if a != 0

macro_rules! out {
    ($a:expr) => {
        (($a & 0b111) ^ 4 ^ ($a >> (($a & 0b111) ^ 1))) & 0b111
    };
}

// #[inline(always)]
// fn is_prefix(a: u64, prefix: u64) -> bool {
//     let a_bits = 64 - a.leading_zeros();
//     let prefix_bits = 64 - prefix.leading_zeros();
//     a >> (a_bits - prefix_bits) == prefix
// }

pub fn part_two(input: &str) -> Option<u64> {
    let prog: Vec<u64> = Computer::parse_input(input).prog;

    let len = 16;
    let target = &prog[prog.len() - len..];
    println!("t: {:?}", target);

    let prefix = 20567627247063;
    //  1: 4
    //  2: 37
    //  3: 299
    //  4: 2394
    //  5: 19155
    //  6: 153240
    //  7: 1225926
    //  8: 9807408
    //  9: 78459_271
    // 10: 627674170x 627674171
    // 11: 5021393370
    // 12: 40171146966
    // 13: 321369175735
    // 14: 2570953405882
    // 15: 20567627247056x 20567627247057x 20567627247058x 20567627247061x 20567627247063

    let mut search_space = HashSet::new();
    let mut p = Vec::with_capacity(len);
    for suffix_len in 1..=16u64 {
        // if !is_prefix(a, prefix) {
        //     continue;
        // }

        // if i % 100_000_000 == 0 {
        //     println!("{}", format!("{:b}", i));
        // }

        for suffix in 0..=((1 << suffix_len) - 1) {
            // TODO: figure out how to keep suffix at suffix_len
            let a = (prefix << suffix_len) | suffix;
            let mut a_test = a;

            p.clear();
            while a_test > 0 {
                let out = out!(a_test);
                p.push(out);
                a_test >>= 3;

                if !target.starts_with(&p) {
                    break;
                }
                if p.len() >= len {
                    break;
                }
            }

            if p == target {
                // println!("a: {} - {}", a, format!("{:b}", a));
                search_space.insert(a);
            }
        }
    }
    for (i, a) in search_space.into_iter().sorted().enumerate() {
        println!("a: {a}");
        if i > 3 {
            break;
        }
    }
    None

    // let mut ss: HashMap<u64, Vec<u64>> = HashMap::new();
    // for p in prog.into_iter() {
    //     if ss.contains_key(&p) {
    //         continue;
    //     }
    //     for a in 0..=0b111111111 {
    //         let out = out!(a);
    //         if out == p {
    //             ss.entry(p).and_modify(|v| v.push(a)).or_insert(vec![a]);
    //         }
    //     }
    // }
    // for (k, v) in ss.iter() {
    //     println!("{} {:?}", k, v.len());
    // }

    // let mut comp = Computer::parse_input(input);

    // let mut search_space: HashMap<u64, Vec<u64>> = HashMap::new();
    // for out in comp.clone().prog.into_iter() {
    //     if search_space.contains_key(&out) {
    //         continue;
    //     }
    //     let mut bf: bool = true;
    //     for a in 0..=0b111 {
    //         comp.reset(a);
    //         if matches!(comp.run_p2(1), Some(o) if o == out) {
    //             search_space.entry(out).and_modify(|s| s.push(a)).or_insert(vec![a]);
    //             bf = false;
    //         }
    //     }
    //     // can't out `5` in 3 bits with my input, so instead brute force that mf
    //     if bf {
    //         for a in 0..=0b111 {
    //             search_space.entry(out).and_modify(|s| s.push(a)).or_insert(vec![a]);
    //         }
    //     }
    // }
    // // for (k, v) in search_space.iter() {
    // //     println!("{k}: {}", v.len());
    // // }

    // let entire_search_space: Vec<Vec<u64>> = comp.prog.iter().rev().map(|out| {
    //     search_space.get(out).unwrap().clone()
    // }).collect();
    // // println!("{}", entire_search_space.iter().map(|s| s.len()).product::<usize>());

    // for (i, a) in entire_search_space.into_iter().multi_cartesian_product().map(|a| a.iter().fold(0, |acc, v| (acc << 3) | v)).enumerate() {
    //     if i % 50_000_000 == 0 {
    //         println!("{i} - {a}");
    //     }
    //     comp.reset(a);
    //     if comp.run_test() {
    //         return Some(a);
    //     }
    // }

    // comp.reset(78459271);
    // comp.run();
    // println!("{}", comp.out_format());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}
