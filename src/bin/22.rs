use std::collections::{HashMap, HashSet};

advent_of_code::solution!(22);

fn mix(num: i64, secret: i64) -> i64 {
    num ^ secret
}

fn prune(secret: i64) -> i64 {
    secret % 16777216
}

fn calc(mut secret: i64) -> i64 {
    secret = mix(secret * 64, secret);
    secret = prune(secret);

    secret = mix(secret / 32, secret);
    secret = prune(secret);

    secret = mix(secret * 2048, secret);
    secret = prune(secret);

    secret
}

pub fn part_one(input: &str) -> Option<i64> {
    let secrets: Vec<i64> = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    Some(
        secrets
            .into_iter()
            .map(|mut secret| {
                for _ in 0..2000 {
                    secret = calc(secret);
                }
                secret
            })
            .sum(),
    )
}

struct Buyer {
    // initial: i64,
    // ones: Vec<i64>,
    // price_changes: Vec<i64>,
    sequences: HashMap<(i64, i64, i64, i64), i64>,
}

impl Buyer {
    fn price_changes(numbers: &[i64]) -> Vec<i64> {
        let mut price_changes = Vec::new();
        for n in 0..numbers.len() - 1 {
            price_changes.push(numbers[n + 1] - numbers[n]);
        }
        price_changes
    }

    fn sequences(price_changes: &[i64], ones: &[i64]) -> HashMap<(i64, i64, i64, i64), i64> {
        let mut sequences = HashMap::new();
        for i in 0..price_changes.len() - 3 {
            let seq = (
                price_changes[i],
                price_changes[i + 1],
                price_changes[i + 2],
                price_changes[i + 3],
            );
            if sequences.contains_key(&seq) {
                continue;
            }
            sequences.insert(seq, ones[i + 4]);
        }
        sequences
    }

    fn new(mut secret: i64) -> Self {
        let mut ones = Vec::new();
        for _ in 0..2000 {
            ones.push(secret % 10);
            secret = calc(secret);
        }
        let price_changes = Self::price_changes(&ones);
        let sequences = Self::sequences(&price_changes, &ones);
        Self {
            // initial: ones[0],
            // ones,
            // price_changes,
            sequences,
        }
    }
}

fn bananas(buyers: &[Buyer], seq: &(i64, i64, i64, i64)) -> i64 {
    let mut bananas = 0;
    for buyer in buyers.iter() {
        if let Some(price) = buyer.sequences.get(seq) {
            bananas += price;
        }
    }
    bananas
}

pub fn part_two(input: &str) -> Option<i64> {
    let secrets: Vec<i64> = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    let buyers: Vec<Buyer> = secrets
        .into_iter()
        .map(|secret| Buyer::new(secret))
        .collect();
    let all_seqs = buyers.iter().fold(HashSet::new(), |mut acc, b| {
        acc.extend(b.sequences.keys());
        acc
    });
    Some(
        all_seqs
            .iter()
            .map(|seq| bananas(&buyers, seq))
            .max()
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }
}
