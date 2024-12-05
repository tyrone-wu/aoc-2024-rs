use std::collections::HashMap;

advent_of_code::solution!(5);

fn in_order(update: &[u32], before_rules: &HashMap<u32, Vec<u32>>, after_rules: &HashMap<u32, Vec<u32>>) -> bool {
    let mut in_order = true;
    for i in 0..update.len() {
        let curr = update[i];
        if i > 0 {
            if let Some(before) = after_rules.get(&curr) {
                if !update[..i].iter().any(|n| before.contains(n)) {
                    in_order = false;
                    break;
                }
            }
            if let Some(after) = before_rules.get(&curr) {
                if update[..i].iter().any(|n| after.contains(n)) {
                    in_order = false;
                    break;
                }
            }
        }
        if i + 1 < update.len() {
            if let Some(after) = before_rules.get(&curr) {
                if !update[i + 1..].iter().any(|n| after.contains(n)) {
                    in_order = false;
                    break;
                }
            }
            if let Some(before) = after_rules.get(&curr) {
                if update[i + 1..].iter().any(|n| before.contains(n)) {
                    in_order = false;
                    break;
                }
            }
        }
    }
    return in_order;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut input = input.split("\n\n");
    let order_rules = input
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut line = l.split('|');
            (
                line.next().unwrap().parse::<u32>().unwrap(),
                line.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let mut before_rules = HashMap::new();
    let mut after_rules = HashMap::new();
    for (b, a) in order_rules.into_iter() {
        before_rules
            .entry(b)
            .and_modify(|r: &mut Vec<u32>| r.push(a))
            .or_insert(vec![a]);
        after_rules
            .entry(a)
            .and_modify(|r: &mut Vec<u32>| r.push(b))
            .or_insert(vec![b]);
    }
    let pages = input
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut mid_sum = 0;
    for update in pages.into_iter() {
        if in_order(&update, &before_rules, &after_rules) {
            mid_sum += update[update.len() / 2];
        }
    }
    Some(mid_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input = input.split("\n\n");
    let order_rules = input
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut line = l.split('|');
            (
                line.next().unwrap().parse::<u32>().unwrap(),
                line.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let mut before_rules = HashMap::new();
    let mut after_rules = HashMap::new();
    for (b, a) in order_rules.into_iter() {
        before_rules
            .entry(b)
            .and_modify(|r: &mut Vec<u32>| r.push(a))
            .or_insert(vec![a]);
        after_rules
            .entry(a)
            .and_modify(|r: &mut Vec<u32>| r.push(b))
            .or_insert(vec![b]);
    }
    let pages = input
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut mid_sum = 0;
    for update in pages.into_iter() {
        if !in_order(&update, &before_rules, &after_rules) {
            let mut correct_order = vec![update[0]];
            for n_up in update.iter().skip(1) {
                for i in 0..correct_order.len() + 1 {
                    let mut try_order = correct_order.clone();
                    try_order.insert(i, *n_up);
                    if in_order(&try_order, &before_rules, &after_rules) {
                        correct_order = try_order.clone();
                        break;
                    }
                }
            }
            mid_sum += correct_order[correct_order.len() / 2];
        }
    }
    Some(mid_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
