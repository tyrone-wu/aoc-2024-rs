advent_of_code::solution!(3);

fn parse_mul(i: &mut usize, input: &[char]) -> Option<(u32, u32)> {
    let mut mul_i = *i;
    if mul_i + 4 >= input.len() || input[mul_i..mul_i + 4] != ['m', 'u', 'l', '('] {
        return None;
    }
    mul_i += 4;

    let mut x_i = mul_i;
    let mut x = 0;
    while let Some(d) = input[x_i].to_digit(10) {
        x = x * 10 + d;
        x_i += 1;
    }
    if mul_i == x_i {
        return None;
    }

    if input[x_i] != ',' {
        return None;
    }
    x_i += 1;

    let mut y_i = x_i;
    let mut y = 0;
    while let Some(d) = input[y_i].to_digit(10) {
        y = y * 10 + d;
        y_i += 1;
    }
    if x_i == y_i {
        return None;
    }

    if input[y_i] != ')' {
        return None;
    }
    y_i += 1;

    *i = y_i;
    Some((x, y))
}

fn parse_do_dont(i: &mut usize, input: &[char]) -> Option<bool> {
    let start = *i;
    if start + 4 < input.len() && input[start..start + 4] == ['d', 'o', '(', ')'] {
        *i += 4;
        Some(true)
    } else if start + 7 < input.len()
        && input[start..start + 7] == ['d', 'o', 'n', '\'', 't', '(', ')']
    {
        *i += 7;
        Some(false)
    } else {
        return None;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let input: Vec<char> = input.chars().collect();
    let mut i = 0;
    let mut sum = 0;
    while i < input.len() {
        if let Some((lhs, rhs)) = parse_mul(&mut i, &input) {
            sum += lhs * rhs;
        } else {
            i += 1;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input: Vec<char> = input.chars().collect();
    let mut i = 0;
    let mut sum = 0;
    let mut enable = true;
    while i < input.len() {
        if let Some(do_dont) = parse_do_dont(&mut i, &input) {
            enable = do_dont;
        } else if let Some((lhs, rhs)) = parse_mul(&mut i, &input) {
            if enable {
                sum += lhs * rhs;
            }
        } else {
            i += 1;
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
