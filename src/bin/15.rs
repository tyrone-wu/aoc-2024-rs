advent_of_code::solution!(15);

#[derive(Debug, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn mv(&self, mv: char) -> Self {
        let mut new = self.clone();
        match mv {
            '>' => new.x += 1,
            'v' => new.y += 1,
            '<' => new.x -= 1,
            '^' => new.y -= 1,
            _ => unreachable!(),
        }
        new
    }
}

struct Map(Vec<Vec<char>>);

impl Map {
    fn get_space(&self, coord: &Coord) -> char {
        self.0[coord.y][coord.x]
    }

    fn set_space(&mut self, coord: &Coord, c: char) {
        self.0[coord.y][coord.x] = c;
    }

    fn robot_position(&self) -> Coord {
        for (y, row) in self.0.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '@' {
                    return Coord { x, y };
                }
            }
        }
        unreachable!();
    }

    #[allow(dead_code)]
    fn display_map(&self) {
        for row in self.0.iter() {
            println!("{}", row.iter().collect::<String>());
        }
    }

    fn calc_sum(self) -> usize {
        self.0.into_iter().enumerate().fold(0, |acc, (y, row)| {
            acc + row
                .into_iter()
                .enumerate()
                .filter_map(|(x, c)| (c == 'O' || c == '[').then_some(100 * y + x))
                .sum::<usize>()
        })
    }

    fn p2_map(&mut self) {
        let mut new = vec![];
        for row in self.0.iter() {
            let mut new_row = vec![];
            for c in row.iter() {
                new_row.extend(match c {
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '.' => ['.', '.'],
                    '@' => ['@', '.'],
                    _ => unreachable!(),
                });
            }
            new.push(new_row);
        }
        self.0 = new;
    }
}

fn parse_input(input: &str) -> (Map, Vec<char>) {
    let mut split = input.split("\n\n");
    let map = split
        .next()
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let moves = split
        .next()
        .unwrap()
        .lines()
        .fold(Vec::new(), |mut acc, line| {
            acc.extend(line.chars());
            acc
        });
    (Map(map), moves)
}

fn can_move(map: &Map, mut curr: Coord, mv: char) -> bool {
    curr = curr.mv(mv);
    match map.get_space(&curr) {
        '.' => true,
        '#' => false,
        'O' => can_move(map, curr, mv),
        _ => unreachable!(),
    }
}

fn shift_robot(map: &mut Map, curr: Coord, c: char, mv: char) {
    let next = curr.mv(mv);
    match map.get_space(&next) {
        '.' => {}
        'O' => shift_robot(map, next.clone(), 'O', mv),
        _ => unreachable!(),
    }
    map.set_space(&next, c);
    map.set_space(&curr, '.');
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut map, moves) = parse_input(input);
    let mut robot = map.robot_position();
    for m in moves.into_iter() {
        if !can_move(&map, robot.clone(), m) {
            continue;
        }
        shift_robot(&mut map, robot, '@', m);
        robot = map.robot_position();
    }
    Some(map.calc_sum())
}

fn can_move_p2(map: &Map, left: Coord, right: Coord, mv: char) -> bool {
    let next_left = left.mv(mv);
    let next_right = right.mv(mv);
    match mv {
        '>' => match map.get_space(&next_right) {
            '#' => false,
            '.' => true,
            '[' => can_move_p2(map, next_right.clone(), next_right.mv('>'), mv),
            _ => unreachable!(),
        },
        '<' => match map.get_space(&next_left) {
            '#' => false,
            '.' => true,
            ']' => can_move_p2(map, next_left.mv('<'), next_left, mv),
            _ => unreachable!(),
        },
        'v' | '^' => match (map.get_space(&next_left), map.get_space(&next_right)) {
            ('#', _) | (_, '#') => false,
            ('.', '.') => true,
            ('[', _) => can_move_p2(map, next_left.clone(), next_left.mv('>'), mv),
            (_, ']') => can_move_p2(map, next_right.mv('<'), next_right, mv),
            (']', '.') => can_move_p2(map, next_left.mv('<'), next_right.mv('<'), mv),
            ('.', '[') => can_move_p2(map, next_left.mv('>'), next_right.mv('>'), mv),
            (']', '[') => {
                can_move_p2(map, next_left.mv('<'), next_left, mv)
                    && can_move_p2(map, next_right.clone(), next_right.mv('>'), mv)
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn shift_robot_p2(map: &mut Map, left: Coord, c_left: char, right: Coord, c_right: char, mv: char) {
    map.set_space(&left, '.');
    map.set_space(&right, '.');
    let next_left = left.mv(mv);
    let next_right = right.mv(mv);
    match mv {
        '>' => match map.get_space(&next_right) {
            '.' => {}
            '[' => shift_robot_p2(map, next_right.clone(), '[', next_right.mv('>'), ']', mv),
            _ => unreachable!(),
        },
        '<' => match map.get_space(&next_left) {
            '.' => {}
            ']' => shift_robot_p2(map, next_left.mv('<'), '[', next_left.clone(), ']', mv),
            _ => unreachable!(),
        },
        'v' | '^' => match (map.get_space(&next_left), map.get_space(&next_right)) {
            ('.', '.') => {}
            ('[', _) => shift_robot_p2(map, next_left.clone(), '[', next_left.mv('>'), ']', mv),
            (_, ']') => shift_robot_p2(map, next_right.mv('<'), '[', next_right.clone(), ']', mv),
            (']', '.') => shift_robot_p2(map, next_left.mv('<'), '[', next_left.clone(), ']', mv),
            ('.', '[') => shift_robot_p2(map, next_right.clone(), '[', next_right.mv('>'), ']', mv),
            (']', '[') => {
                shift_robot_p2(map, next_left.mv('<'), '[', next_left.clone(), ']', mv);
                shift_robot_p2(map, next_right.clone(), '[', next_right.mv('>'), ']', mv);
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
    map.set_space(&next_left.clone(), c_left);
    map.set_space(&next_right, c_right);
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut map, moves) = parse_input(input);
    map.p2_map();
    let mut robot = map.robot_position();
    for m in moves.into_iter() {
        if !can_move_p2(&map, robot.clone(), robot.clone(), m) {
            continue;
        }
        shift_robot_p2(&mut map, robot.clone(), '@', robot.clone(), '@', m);
        robot = map.robot_position();
    }
    Some(map.calc_sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
