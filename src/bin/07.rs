advent_of_code::solution!(7);

struct Equation {
    target: u64,
    operands: Vec<u64>,
}

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|l| {
            let mut l = l.split_whitespace();
            let mut val_c = l.next().unwrap().chars();
            val_c.next_back();
            let target = val_c.as_str().parse::<u64>().unwrap();
            Equation {
                target,
                operands: l.map(|op| op.parse::<u64>().unwrap()).collect(),
            }
        })
        .collect::<Vec<Equation>>()
}

fn solve(operands: &[u64], result: u64, target: u64, p2: bool) -> bool {
    if operands.is_empty() {
        return result == target;
    }
    let v = operands[0];
    let add = solve(&operands[1..], result + v, target, p2);
    let mul = solve(&operands[1..], result * v, target, p2);
    let con = if p2 {
        let mut res_str = result.to_string();
        res_str.push_str(&v.to_string());
        solve(&operands[1..], res_str.parse::<u64>().unwrap(), target, p2)
    } else {
        false
    };
    add || mul || con
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse_input(input);
    let sum = equations.into_iter().fold(0, |mut acc, eq| {
        if solve(&eq.operands, 0, eq.target, false) {
            acc += eq.target;
        }
        acc
    });
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_input(input);
    let sum = equations.into_iter().fold(0, |mut acc, eq| {
        if solve(&eq.operands, 0, eq.target, true) {
            acc += eq.target;
        }
        acc
    });
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
