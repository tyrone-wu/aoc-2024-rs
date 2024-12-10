use std::collections::HashSet;

advent_of_code::solution!(10);

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
        .collect()
}

fn get_trailheads(map: &[Vec<u8>]) -> Vec<(usize, usize)> {
    map.iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (y, row)| {
            for (x, p) in row.iter().enumerate() {
                if *p == 0 {
                    acc.push((x, y))
                }
            }
            acc
        })
}

fn dfs(
    map: &[Vec<u8>],
    visited: &mut HashSet<(usize, usize)>,
    x: usize,
    y: usize,
    trailheads: &mut u32,
) {
    if map[y][x] == 9 {
        *trailheads += 1;
        return;
    }

    let next_val = map[y][x] + 1;
    if y > 0 && next_val == map[y - 1][x] && !visited.contains(&(x, y - 1)) {
        visited.insert((x, y - 1));
        dfs(map, visited, x, y - 1, trailheads);
    }
    if y + 1 < map.len() && next_val == map[y + 1][x] && !visited.contains(&(x, y + 1)) {
        visited.insert((x, y + 1));
        dfs(map, visited, x, y + 1, trailheads);
    }
    if x > 0 && next_val == map[y][x - 1] && !visited.contains(&(x - 1, y)) {
        visited.insert((x - 1, y));
        dfs(map, visited, x - 1, y, trailheads);
    }
    if x + 1 < map[0].len() && next_val == map[y][x + 1] && !visited.contains(&(x + 1, y)) {
        visited.insert((x + 1, y));
        dfs(map, visited, x + 1, y, trailheads);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let start = get_trailheads(&map);
    let mut trailheads = 0;
    for (x, y) in start.into_iter() {
        dfs(&map, &mut HashSet::from([(x, y)]), x, y, &mut trailheads);
    }
    Some(trailheads)
}

fn dfs_p2(
    map: &[Vec<u8>],
    mut visited: HashSet<(usize, usize)>,
    x: usize,
    y: usize,
    trailheads: &mut u32,
) {
    if map[y][x] == 9 {
        *trailheads += 1;
        return;
    }

    let next_val = map[y][x] + 1;
    if y > 0 && next_val == map[y - 1][x] && !visited.contains(&(x, y - 1)) {
        visited.insert((x, y - 1));
        dfs_p2(map, visited.clone(), x, y - 1, trailheads);
    }
    if y + 1 < map.len() && next_val == map[y + 1][x] && !visited.contains(&(x, y + 1)) {
        visited.insert((x, y + 1));
        dfs_p2(map, visited.clone(), x, y + 1, trailheads);
    }
    if x > 0 && next_val == map[y][x - 1] && !visited.contains(&(x - 1, y)) {
        visited.insert((x - 1, y));
        dfs_p2(map, visited.clone(), x - 1, y, trailheads);
    }
    if x + 1 < map[0].len() && next_val == map[y][x + 1] && !visited.contains(&(x + 1, y)) {
        visited.insert((x + 1, y));
        dfs_p2(map, visited.clone(), x + 1, y, trailheads);
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let start = get_trailheads(&map);
    let mut trailheads = 0;
    for (x, y) in start.into_iter() {
        dfs_p2(&map, HashSet::from([(x, y)]), x, y, &mut trailheads);
    }
    Some(trailheads)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
