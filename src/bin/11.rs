use std::collections::HashMap;

advent_of_code::solution!(11);

fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim_end()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn blink(s: u64, seen: &mut HashMap<(u32, u64), u64>, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }
    if let Some(s_len) = seen.get(&(depth, s)) {
        return *s_len;
    }

    let s_len = match s {
        0 => blink(1, seen, depth - 1),
        s if (s.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 => {
            let i = ((s.checked_ilog10().unwrap_or(0) + 1) / 2) as usize;

            let left = s % 10_u64.pow(i as u32);
            let left_len = blink(left, seen, depth - 1);

            let right = s / 10_u64.pow(i as u32);
            let right_len = blink(right, seen, depth - 1);

            left_len + right_len
        }
        _ => blink(s * 2024, seen, depth - 1),
    };
    seen.insert((depth, s), s_len);
    s_len
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut stones = 0;
    let mut seen: HashMap<(u32, u64), u64> = HashMap::new();
    for s in parse_input(input) {
        stones += blink(s, &mut seen, 25);
    }
    Some(stones)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut stones = 0;
    let mut seen: HashMap<(u32, u64), u64> = HashMap::new();
    for s in parse_input(input) {
        stones += blink(s, &mut seen, 75);
    }
    Some(stones)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
