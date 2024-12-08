use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn grid_coord(grid: &[Vec<char>]) -> HashMap<char, Vec<(i32, i32)>> {
    grid.iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (y, line)| {
            for (x, c) in line.iter().enumerate() {
                if *c == '.' {
                    continue;
                }
                let p = (x as i32, y as i32);
                acc.entry(*c)
                    .and_modify(|coord: &mut Vec<(i32, i32)>| coord.push(p))
                    .or_insert(vec![p]);
            }
            acc
        })
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_input(input);
    let grid_coord = grid_coord(&grid);
    let x_max = grid[0].len() as i32 - 1;
    let y_max = grid.len() as i32 - 1;
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for (ant, coords) in grid_coord.iter() {
        for i in 0..coords.len() {
            let coord_i = coords[i];
            for j in (i + 1)..coords.len() {
                let coord_j = coords[j];
                let dx = coord_i.0 - coord_j.0;
                let dy = coord_i.1 - coord_j.1;

                let x_new = coord_i.0 - dx;
                let y_new = coord_i.1 - dy;
                if 0 <= x_new && x_new <= x_max && 0 <= y_new && y_new <= y_max {
                    if grid[y_new as usize][x_new as usize] != *ant {
                        antinodes.insert((x_new, y_new));
                    }
                }

                let x_new = coord_i.0 + dx;
                let y_new = coord_i.1 + dy;
                if 0 <= x_new && x_new <= x_max && 0 <= y_new && y_new <= y_max {
                    if grid[y_new as usize][x_new as usize] != *ant {
                        antinodes.insert((x_new, y_new));
                    }
                }

                let x_new = coord_j.0 - dx;
                let y_new = coord_j.1 - dy;
                if 0 <= x_new && x_new <= x_max && 0 <= y_new && y_new <= y_max {
                    if grid[y_new as usize][x_new as usize] != *ant {
                        antinodes.insert((x_new, y_new));
                    }
                }

                let x_new = coord_j.0 + dx;
                let y_new = coord_j.1 + dy;
                if 0 <= x_new && x_new <= x_max && 0 <= y_new && y_new <= y_max {
                    if grid[y_new as usize][x_new as usize] != *ant {
                        antinodes.insert((x_new, y_new));
                    }
                }
            }
        }
    }
    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse_input(input);
    let grid_coord = grid_coord(&grid);
    let x_max = grid[0].len() as i32 - 1;
    let y_max = grid.len() as i32 - 1;
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for (_, coords) in grid_coord.iter() {
        for i in 0..coords.len() {
            let coord_i = coords[i];
            for j in (i + 1)..coords.len() {
                let coord_j = coords[j];
                let dx = coord_i.0 - coord_j.0;
                let dy = coord_i.1 - coord_j.1;

                let mut x_new = coord_i.0 - dx;
                let mut y_new = coord_i.1 - dy;
                while 0 <= x_new && x_new <= x_max && 0 <= y_new && y_new <= y_max {
                    antinodes.insert((x_new, y_new));
                    x_new -= dx;
                    y_new -= dy;
                }

                let mut x_new = coord_i.0 + dx;
                let mut y_new = coord_i.1 + dy;
                while 0 <= x_new && x_new <= x_max && 0 <= y_new && y_new <= y_max {
                    antinodes.insert((x_new, y_new));
                    x_new += dx;
                    y_new += dy;
                }

                let mut x_new = coord_j.0 - dx;
                let mut y_new = coord_j.1 - dy;
                while 0 <= x_new && x_new <= x_max && 0 <= y_new && y_new <= y_max {
                    antinodes.insert((x_new, y_new));
                    x_new -= dx;
                    y_new -= dy;
                }

                let mut x_new = coord_j.0 + dx;
                let mut y_new = coord_j.1 + dy;
                while 0 <= x_new && x_new <= x_max && 0 <= y_new && y_new <= y_max {
                    antinodes.insert((x_new, y_new));
                    x_new += dx;
                    y_new += dy;
                }
            }
        }
    }
    Some(antinodes.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
