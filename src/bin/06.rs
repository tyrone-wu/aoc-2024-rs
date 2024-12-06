use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Clone, Hash, Eq, PartialEq)]
struct Guard {
    x: usize,
    y: usize,
    direction: u8,
}

fn get_guard(grid: &[Vec<char>]) -> Guard {
    grid.iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find_map(|(x, c)| (c == &'^').then_some(x))
                .map(|x| Guard { x, y, direction: 0 })
        })
        .unwrap()
}

fn delta(guard: &Guard, bounds: &(usize, usize)) -> Option<(i32, i32)> {
    let Guard { x, y, direction } = guard;
    let (dx, dy) = match direction {
        0 => (0, -1),
        1 => (1, 0),
        2 => (0, 1),
        _ => (-1, 0),
    };
    let (x_max, y_max) = bounds;
    if (*x == 0 && dx == -1)
        || (x == x_max && dx == 1)
        || (*y == 0 && dy == -1)
        || (y == y_max && dy == 1)
    {
        None
    } else {
        Some((dx, dy))
    }
}

fn step(grid: &[Vec<char>], bounds: (usize, usize), mut guard: Guard) -> Option<Guard> {
    let delta = delta(&guard, &bounds);
    if delta.is_none() {
        return None;
    }
    let (dx, dy) = delta.unwrap();
    let nx = (guard.x as i32 + dx) as usize;
    let ny = (guard.y as i32 + dy) as usize;
    if grid[ny][nx] != '.' {
        guard.direction = (guard.direction + 1) % 4;
    } else {
        guard.x = nx;
        guard.y = ny;
    }
    Some(guard)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let bounds = (grid[0].len() - 1, grid.len() - 1);
    let mut guard = get_guard(&grid);
    grid[guard.y][guard.x] = '.';
    let mut visited = HashSet::from([(guard.x, guard.y)]);
    while let Some(g) = step(&grid, bounds, guard) {
        visited.insert((g.x, g.y));
        guard = g;
    }
    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let bounds = (grid[0].len() - 1, grid.len() - 1);
    let start = get_guard(&grid);
    let mut guard = start.clone();
    grid[guard.y][guard.x] = '.';
    let mut visited = HashSet::from([guard.clone()]);
    let mut loop_blocks = HashSet::new();
    while let Some(g) = step(&grid, bounds, guard) {
        visited.insert(g.clone());
        guard = g;

        if let Some((dx, dy)) = delta(&guard, &bounds) {
            let nx = (guard.x as i32 + dx) as usize;
            let ny = (guard.y as i32 + dy) as usize;
            // check to see if spot ahead is already '#'
            if grid[ny][nx] != '.' {
                continue;
            }
            // brute force
            let mut try_grid = grid.clone();
            try_grid[ny][nx] = 'O';
            let mut try_guard = start.clone();
            let mut try_visited = HashSet::new();
            while let Some(g) = step(&try_grid, bounds, try_guard) {
                if try_visited.contains(&g) {
                    loop_blocks.insert((nx, ny));
                    break;
                }
                try_visited.insert(g.clone());
                try_guard = g;
            }
        }
    }
    Some(loop_blocks.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
