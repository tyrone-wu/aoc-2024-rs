advent_of_code::solution!(14);

#[derive(Debug, Clone)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
struct Robot {
    position: Coord,
    velocity: Coord,
}

fn parse_xy(section: &str) -> Coord {
    let eq = section.find('=').unwrap();
    let comma = section.find(',').unwrap();
    let x = section[eq + 1..comma].parse::<i64>().unwrap();
    let y = section[comma + 1..].parse::<i64>().unwrap();
    Coord { x, y }
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let mut line = line.split_whitespace();
            let position = parse_xy(line.next().unwrap());
            let velocity = parse_xy(line.next().unwrap());
            Robot { position, velocity }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<i64> {
    let wide = 101;
    let tall = 103;
    let seconds = 100;
    let mut robots = parse_input(input);
    for r in robots.iter_mut() {
        let Robot { position, velocity } = r;
        position.x = (position.x + velocity.x * seconds).rem_euclid(wide);
        position.y = (position.y + velocity.y * seconds).rem_euclid(tall);
    }
    let x_mid = wide / 2;
    let y_mid = tall / 2;
    let mut quad_1 = 0;
    let mut quad_2 = 0;
    let mut quad_3 = 0;
    let mut quad_4 = 0;
    for r in robots.into_iter() {
        let Robot { position, .. } = r;
        let Coord { x, y } = position;
        if x < x_mid && y < y_mid {
            quad_1 += 1;
        } else if x < x_mid && y > y_mid {
            quad_2 += 1;
        } else if x > x_mid && y < y_mid {
            quad_3 += 1;
        } else if x > x_mid && y > y_mid {
            quad_4 += 1;
        }
    }
    Some(quad_1 * quad_2 * quad_3 * quad_4)
}

pub fn part_two(input: &str) -> Option<u32> {
    const WIDE: usize = 101;
    const TALL: usize = 103;
    let original_robots = parse_input(input);
    for i in 5000..10000 {
        println!("{i} =====");
        let mut robots = original_robots.clone();
        for r in robots.iter_mut() {
            let Robot { position, velocity } = r;
            position.x = (position.x + velocity.x * i).rem_euclid(WIDE as i64);
            position.y = (position.y + velocity.y * i).rem_euclid(TALL as i64);
        }

        let mut grid = [[0; WIDE]; TALL];
        for r in robots.into_iter() {
            let Robot { position, .. } = r;
            grid[position.y as usize][position.x as usize] += 1;
        }
        for row in grid.into_iter() {
            for c in row.into_iter() {
                if c == 0 {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!();
        }
        println!("=====");
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
