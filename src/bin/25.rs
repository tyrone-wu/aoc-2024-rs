advent_of_code::solution!(25);

fn parse_input(input: &str) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for grid in input.split("\n\n") {
        let grid: Vec<Vec<char>> = grid.lines().map(|line| line.chars().collect()).collect();
        let is_lock = grid[0].iter().all(|c| *c == '#');
        let mut heights = Vec::new();
        for i in 0..grid[0].len() {
            let mut col_filled = 0;
            for j in 0..grid.len() {
                if grid[j][i] == '#' {
                    col_filled += 1;
                }
            }
            heights.push(col_filled);
        }
        if is_lock {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }
    (locks, keys)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (locks, keys) = parse_input(input);
    let mut count = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            let mut fits = true;
            for i in 0..lock.len() {
                if lock[i] + key[i] > 7 {
                    fits = false;
                    break;
                }
            }
            if fits {
                count += 1;
            }
        }
    }
    Some(count)
}

pub fn part_two(_input: &str) -> Option<u32> {
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
