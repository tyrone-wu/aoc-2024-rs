use std::collections::{HashMap, HashSet};

advent_of_code::solution!(12);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn perimeter_count(
    x: usize,
    y: usize,
    x_max: usize,
    y_max: usize,
    visited_region: &HashSet<(usize, usize)>,
) -> i64 {
    let mut surrounded = 0;
    // up
    if y > 0 && visited_region.contains(&(x, y - 1)) {
        surrounded += 1;
    }
    // down
    if y + 1 <= y_max && visited_region.contains(&(x, y + 1)) {
        surrounded += 1;
    }
    // left
    if x > 0 && visited_region.contains(&(x - 1, y)) {
        surrounded += 1;
    }
    // right
    if x + 1 <= x_max && visited_region.contains(&(x + 1, y)) {
        surrounded += 1;
    }
    match surrounded {
        1 => 2,
        2 => 0,
        3 => -2,
        _ => -4,
    }
}

fn perimeter_count_p2(
    x: usize,
    y: usize,
    x_max: usize,
    y_max: usize,
    visited_region: &HashSet<(usize, usize)>,
) -> i64 {
    let mut cardinal = 0;
    let up = 1 << 0;
    let down = 1 << 1;
    let left = 1 << 2;
    let right = 1 << 3;

    // up
    if y > 0 && visited_region.contains(&(x, y - 1)) {
        cardinal |= up;
    }
    // down
    if y + 1 <= y_max && visited_region.contains(&(x, y + 1)) {
        cardinal |= down;
    }
    // left
    if x > 0 && visited_region.contains(&(x - 1, y)) {
        cardinal |= left;
    }
    // right
    if x + 1 <= x_max && visited_region.contains(&(x + 1, y)) {
        cardinal |= right;
    }

    let mut corners: i32 = 0;
    let up_left = 1 << 4;
    let down_left = 1 << 5;
    let down_right = 1 << 6;
    let up_right = 1 << 7;
    // up left
    if x > 0 && y > 0 && visited_region.contains(&(x - 1, y - 1)) {
        corners |= up_left;
    }
    // up right
    if x + 1 <= x_max && y > 0 && visited_region.contains(&(x + 1, y - 1)) {
        corners |= up_right;
    }
    // down left
    if x > 0 && y + 1 <= y_max && visited_region.contains(&(x - 1, y + 1)) {
        corners |= down_left;
    }
    // down right
    if x + 1 <= x_max && y + 1 <= y_max && visited_region.contains(&(x + 1, y + 1)) {
        corners |= down_right;
    }

    if cardinal == up || cardinal == down || cardinal == left || cardinal == right {
        if cardinal == up {
            corners &= up_left | up_right;
            if corners == up_left || corners == up_right {
                2
            } else if corners == (up_left | up_right) {
                4
            } else {
                0
            }
        } else if cardinal == down {
            corners &= down_left | down_right;
            if corners == down_left || corners == down_right {
                2
            } else if corners == (down_left | down_right) {
                4
            } else {
                0
            }
        } else if cardinal == left {
            corners &= up_left | down_left;
            if corners == up_left || corners == down_left {
                2
            } else if corners == (up_left | down_left) {
                4
            } else {
                0
            }
        } else {
            // right
            corners &= up_right | down_right;
            if corners == up_right || corners == down_right {
                2
            } else if corners == (up_right | down_right) {
                4
            } else {
                0
            }
        }
    }
    // two side adjacent next to each other
    else if cardinal == (up | left)
        || cardinal == (left | down)
        || cardinal == (down | right)
        || cardinal == (right | up)
    {
        if cardinal == (up | left) {
            corners &= up_right | down_left;
            if corners == up_right || corners == down_left {
                0
            } else if corners == (up_right | down_left) {
                2
            } else {
                -2
            }
        } else if cardinal == (left | down) {
            corners &= up_left | down_right;
            if corners == up_left || corners == down_right {
                0
            } else if corners == (up_left | down_right) {
                2
            } else {
                -2
            }
        } else if cardinal == (down | right) {
            corners &= up_right | down_left;
            if corners == up_right || corners == down_left {
                0
            } else if corners == (up_right | down_left) {
                2
            } else {
                -2
            }
        } else {
            corners &= up_left | down_right;
            if corners == up_left || corners == down_right {
                0
            } else if corners == (up_left | down_right) {
                2
            } else {
                -2
            }
        }
    }
    // two side adjacent across each other
    else if cardinal == (up | down) || cardinal == (left | right) {
        if corners == up_left
            || corners == up_right
            || corners == down_left
            || corners == down_right
        {
            -2
        } else if corners.count_ones() == 2 {
            0
        } else if corners.count_ones() == 3 {
            2
        } else if corners.count_ones() == 4 {
            4
        } else {
            -4
        }
    }
    // three side adjacent
    else if cardinal == (up | left | down)
        || cardinal == (left | down | right)
        || cardinal == (down | right | up)
        || cardinal == (right | up | left)
    {
        if cardinal == (up | left | down) {
            corners &= up_right | down_right;
            if corners == up_right || corners == down_right {
                -2
            } else if corners == (up_right | down_right) {
                0
            } else {
                -4
            }
        } else if cardinal == (left | down | right) {
            corners &= up_left | up_right;
            if corners == up_left || corners == up_right {
                -2
            } else if corners == (up_left | up_right) {
                0
            } else {
                -4
            }
        } else if cardinal == (down | right | up) {
            corners &= up_left | down_left;
            if corners == up_left || corners == down_left {
                -2
            } else if corners == (up_left | down_left) {
                0
            } else {
                -4
            }
        } else {
            corners &= down_left | down_right;
            if corners == down_left || corners == down_right {
                -2
            } else if corners == (down_left | down_right) {
                0
            } else {
                -4
            }
        }
    } else if cardinal == (up | left | down | right) {
        -4
    } else {
        unreachable!()
    }
}

fn dfs(
    garden: &[Vec<char>],
    visited: &mut HashSet<(usize, usize)>,
    visited_region: &mut HashSet<(usize, usize)>,
    plant: char,
    x: usize,
    y: usize,
    area: &mut u64,
    perimeter: &mut u64,
    p2: bool,
) {
    let x_max = garden[0].len() - 1;
    let y_max = garden.len() - 1;

    // up
    if y > 0 && garden[y - 1][x] == plant && !visited_region.contains(&(x, y - 1)) {
        let up = (x, y - 1);
        visited_region.insert(up);
        visited.insert(up);
        *area += 1;
        if p2 {
            *perimeter = (*perimeter as i64
                + perimeter_count_p2(x, y - 1, x_max, y_max, &visited_region))
                as u64;
        } else {
            *perimeter = (*perimeter as i64
                + perimeter_count(x, y - 1, x_max, y_max, &visited_region))
                as u64;
        }
        dfs(
            garden,
            visited,
            visited_region,
            plant,
            x,
            y - 1,
            area,
            perimeter,
            p2,
        );
    }
    // down
    let down = (x, y + 1);
    if y + 1 <= y_max && garden[y + 1][x] == plant && !visited_region.contains(&down) {
        visited_region.insert(down);
        visited.insert(down);
        *area += 1;
        if p2 {
            *perimeter = (*perimeter as i64
                + perimeter_count_p2(x, y + 1, x_max, y_max, &visited_region))
                as u64;
        } else {
            *perimeter = (*perimeter as i64
                + perimeter_count(x, y + 1, x_max, y_max, &visited_region))
                as u64;
        }
        dfs(
            garden,
            visited,
            visited_region,
            plant,
            x,
            y + 1,
            area,
            perimeter,
            p2,
        );
    }
    // left
    if x > 0 && garden[y][x - 1] == plant && !visited_region.contains(&(x - 1, y)) {
        let left = (x - 1, y);
        visited_region.insert(left);
        visited.insert(left);
        *area += 1;
        if p2 {
            *perimeter = (*perimeter as i64
                + perimeter_count_p2(x - 1, y, x_max, y_max, &visited_region))
                as u64;
        } else {
            *perimeter = (*perimeter as i64
                + perimeter_count(x - 1, y, x_max, y_max, &visited_region))
                as u64;
        }
        dfs(
            garden,
            visited,
            visited_region,
            plant,
            x - 1,
            y,
            area,
            perimeter,
            p2,
        );
    }
    // right
    let right = (x + 1, y);
    if x + 1 <= x_max && garden[y][x + 1] == plant && !visited_region.contains(&right) {
        visited_region.insert(right);
        visited.insert(right);
        *area += 1;
        if p2 {
            *perimeter = (*perimeter as i64
                + perimeter_count_p2(x + 1, y, x_max, y_max, &visited_region))
                as u64;
        } else {
            *perimeter = (*perimeter as i64
                + perimeter_count(x + 1, y, x_max, y_max, &visited_region))
                as u64;
        }
        dfs(
            garden,
            visited,
            visited_region,
            plant,
            x + 1,
            y,
            area,
            perimeter,
            p2,
        );
    }
}

fn solve(input: &str, p2: bool) -> u64 {
    let garden = parse_input(input);
    let mut regions: HashMap<char, Vec<(u64, u64)>> = HashMap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut total_price = 0;
    for y in 0..garden.len() {
        for x in 0..garden[0].len() {
            if visited.contains(&(x, y)) {
                continue;
            }

            let mut area = 1;
            let mut perimeter = 4;
            let plant = garden[y][x];

            visited.insert((x, y));
            dfs(
                &garden,
                &mut visited,
                &mut HashSet::from([(x, y)]),
                plant,
                x,
                y,
                &mut area,
                &mut perimeter,
                p2,
            );

            regions
                .entry(plant)
                .and_modify(|r| r.push((area, perimeter)))
                .or_insert(vec![(area, perimeter)]);
            total_price += area * perimeter;
        }
    }

    total_price
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, false))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(368));
    }
}
