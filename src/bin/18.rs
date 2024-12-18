use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(18);

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(',');
            (
                split.next().unwrap().parse::<u64>().unwrap(),
                split.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let byte_pos = parse_input(input);
    let fallen = 1024;
    let max = 70;
    let x = 0;
    let y = 0;
    let mut corrupted: HashSet<(u64, u64)> = HashSet::new();
    for i in 0..fallen {
        corrupted.insert(byte_pos[i]);
    }
    let mut bfs = VecDeque::from([(x, y, 0)]);
    let mut visited = HashSet::from([(x, y)]);
    while let Some((x, y, steps)) = bfs.pop_front() {
        if x == max && y == max {
            return Some(steps);
        }
        // up
        if y > 0 && !visited.contains(&(x, y - 1)) && !corrupted.contains(&(x, y - 1)) {
            visited.insert((x, y - 1));
            bfs.push_back((x, y - 1, steps + 1));
        }
        // right
        if x < max && !visited.contains(&(x + 1, y)) && !corrupted.contains(&(x + 1, y)) {
            visited.insert((x + 1, y));
            bfs.push_back((x + 1, y, steps + 1));
        }
        // down
        if y < max && !visited.contains(&(x, y + 1)) && !corrupted.contains(&(x, y + 1)) {
            visited.insert((x, y + 1));
            bfs.push_back((x, y + 1, steps + 1));
        }
        // left
        if x > 0 && !visited.contains(&(x - 1, y)) && !corrupted.contains(&(x - 1, y)) {
            visited.insert((x - 1, y));
            bfs.push_back((x - 1, y, steps + 1));
        }
    }

    None
}

fn exit_exists(max: u64, corrupted: &HashSet<(u64, u64)>) -> bool {
    let x = 0;
    let y = 0;
    let mut bfs = VecDeque::from([(x, y, 0)]);
    let mut visited = HashSet::from([(x, y)]);
    while let Some((x, y, steps)) = bfs.pop_front() {
        if x == max && y == max {
            return true;
        }
        // up
        if y > 0 && !visited.contains(&(x, y - 1)) && !corrupted.contains(&(x, y - 1)) {
            visited.insert((x, y - 1));
            bfs.push_back((x, y - 1, steps + 1));
        }
        // right
        if x < max && !visited.contains(&(x + 1, y)) && !corrupted.contains(&(x + 1, y)) {
            visited.insert((x + 1, y));
            bfs.push_back((x + 1, y, steps + 1));
        }
        // down
        if y < max && !visited.contains(&(x, y + 1)) && !corrupted.contains(&(x, y + 1)) {
            visited.insert((x, y + 1));
            bfs.push_back((x, y + 1, steps + 1));
        }
        // left
        if x > 0 && !visited.contains(&(x - 1, y)) && !corrupted.contains(&(x - 1, y)) {
            visited.insert((x - 1, y));
            bfs.push_back((x - 1, y, steps + 1));
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<String> {
    let byte_pos = parse_input(input);
    let max = 70;
    let mut corrupted: HashSet<(u64, u64)> = HashSet::new();
    for pos in byte_pos {
        corrupted.insert(pos);
        if !exit_exists(max, &corrupted) {
            return Some(format!("{:?}", pos));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
