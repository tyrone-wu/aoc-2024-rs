advent_of_code::solution!(13);

#[derive(Debug)]
struct Coord {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct ClawMachine {
    a: Coord,
    b: Coord,
    prize: Coord,
}

fn parse_coord(line: &str) -> Coord {
    let x = line[line.find('X').unwrap() + 2..line.find(',').unwrap()]
        .parse::<f64>()
        .unwrap();
    let y = line[line.find('Y').unwrap() + 2..].parse::<f64>().unwrap();
    Coord { x, y }
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    input
        .split("\n\n")
        .map(|block| {
            let mut lines = block.lines();
            let a = parse_coord(lines.next().unwrap());
            let b = parse_coord(lines.next().unwrap());
            let prize = parse_coord(lines.next().unwrap());
            ClawMachine { a, b, prize }
        })
        .collect()
}

fn round(x: f64) -> f64 {
    (x * 1000.).round() / 1000.
}

fn solve_equation(machine: ClawMachine) -> Option<(f64, f64)> {
    let ClawMachine { a, b, prize } = machine;
    let Coord { x: ax, y: ay } = a;
    let Coord { x: bx, y: by } = b;
    let Coord { x: px, y: py } = prize;
    let a_c = ax - ay;
    let b_c = bx - by;
    let p_c = px - py;

    if a_c == 0. {
        let a = round((px - (bx * p_c / b_c)) / (bx * -a_c / b_c + ax));
        if a.fract() == 0. {
            let b = round((px - ax * a) / bx);
            if b.fract() == 0. {
                return Some((a, b));
            }
        }
    } else {
        let b = round((px - (ax * p_c / a_c)) / (ax * -b_c / a_c + bx));
        if b.fract() == 0. {
            let a = round((px - bx * b) / ax);
            if a.fract() == 0. {
                return Some((a, b));
            }
        }
    }
    None
}

fn solve(machines: Vec<ClawMachine>) -> f64 {
    machines.into_iter().fold(0., |mut acc, m| {
        if let Some((a, b)) = solve_equation(m) {
            acc += a * 3. + b;
        }
        acc
    })
}

pub fn part_one(input: &str) -> Option<f64> {
    let machines = parse_input(input);
    Some(solve(machines))
}

pub fn part_two(input: &str) -> Option<f64> {
    let mut machines = parse_input(input);
    for m in machines.iter_mut() {
        m.prize.x += 10000000000000.;
        m.prize.y += 10000000000000.;
    }
    Some(solve(machines))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480.));
    }

    #[test]
    fn test_part_two() {
        let result: Option<f64> = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
