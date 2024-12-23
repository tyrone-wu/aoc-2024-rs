use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(23);

fn parse_input(input: &str) -> HashMap<String, HashSet<String>> {
    input.lines().fold(HashMap::new(), |mut acc, line| {
        let mut iter = line.split('-');
        let comp_a = iter.next().unwrap();
        let comp_b = iter.next().unwrap();
        acc.entry(comp_a.to_owned())
            .and_modify(|c| {
                c.insert(comp_a.to_owned());
                c.insert(comp_b.to_owned());
            })
            .or_insert(HashSet::from([comp_a.to_owned(), comp_b.to_owned()]));
        acc.entry(comp_b.to_owned())
            .and_modify(|c| {
                c.insert(comp_a.to_owned());
                c.insert(comp_b.to_owned());
            })
            .or_insert(HashSet::from([comp_a.to_owned(), comp_b.to_owned()]));
        acc
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    let computers = parse_input(input);
    let mut lans = HashSet::new();
    for (_, connected) in computers.iter() {
        for mut combos in connected.into_iter().combinations(3) {
            let mut drop = false;
            for a in combos.iter() {
                for b in combos.iter() {
                    if !computers.get(*a).unwrap().contains(*b) {
                        drop = true;
                        break;
                    }
                }
                if drop {
                    break;
                }
            }
            if drop {
                continue;
            }
            combos.sort();
            lans.insert((combos[0], combos[1], combos[2]));
        }
    }
    Some(
        lans.into_iter()
            .filter(|(a, b, c)| {
                a.chars().next().unwrap() == 't'
                    || b.chars().next().unwrap() == 't'
                    || c.chars().next().unwrap() == 't'
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let computers = parse_input(input);
    let mut lans = HashSet::new();
    for (_, connected) in computers.iter() {
        for size in (1..=connected.len()).rev() {
            for mut combos in connected.iter().combinations(size) {
                let mut drop = false;
                for x in combos.iter() {
                    for y in combos.iter() {
                        if !computers.get(*x).unwrap().contains(*y) {
                            drop = true;
                            break;
                        }
                    }
                    if drop {
                        break;
                    }
                }
                if drop {
                    continue;
                }
                combos.sort();
                lans.insert(combos.into_iter().join(","));
            }
        }
    }
    let max_lan_len = lans.iter().map(|a| a.len()).max().unwrap();
    lans.into_iter().find(|lan| lan.len() == max_lan_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.as_deref(), Some("co,de,ka,ta"));
    }
}
