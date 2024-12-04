advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let input_2d = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let i_len = input_2d.len();
    let j_len = input_2d[0].len();
    let mut xmas_count = 0;
    for i in 0..i_len {
        for j in 0..j_len {
            // north
            if i >= 3
                && input_2d[i][j] == 'X'
                && input_2d[i - 1][j] == 'M'
                && input_2d[i - 2][j] == 'A'
                && input_2d[i - 3][j] == 'S'
            {
                xmas_count += 1;
            }
            // north east
            if i >= 3
                && j + 3 < j_len
                && input_2d[i][j] == 'X'
                && input_2d[i - 1][j + 1] == 'M'
                && input_2d[i - 2][j + 2] == 'A'
                && input_2d[i - 3][j + 3] == 'S'
            {
                xmas_count += 1;
            }
            // east
            if j + 3 < j_len
                && input_2d[i][j] == 'X'
                && input_2d[i][j + 1] == 'M'
                && input_2d[i][j + 2] == 'A'
                && input_2d[i][j + 3] == 'S'
            {
                xmas_count += 1;
            }
            // south east
            if i + 3 < i_len
                && j + 3 < j_len
                && input_2d[i][j] == 'X'
                && input_2d[i + 1][j + 1] == 'M'
                && input_2d[i + 2][j + 2] == 'A'
                && input_2d[i + 3][j + 3] == 'S'
            {
                xmas_count += 1;
            }
            // south
            if i + 3 < i_len
                && input_2d[i][j] == 'X'
                && input_2d[i + 1][j] == 'M'
                && input_2d[i + 2][j] == 'A'
                && input_2d[i + 3][j] == 'S'
            {
                xmas_count += 1;
            }
            // south west
            if i + 3 < i_len
                && j >= 3
                && input_2d[i][j] == 'X'
                && input_2d[i + 1][j - 1] == 'M'
                && input_2d[i + 2][j - 2] == 'A'
                && input_2d[i + 3][j - 3] == 'S'
            {
                xmas_count += 1;
            }
            // west
            if j >= 3
                && input_2d[i][j] == 'X'
                && input_2d[i][j - 1] == 'M'
                && input_2d[i][j - 2] == 'A'
                && input_2d[i][j - 3] == 'S'
            {
                xmas_count += 1;
            }
            // north west
            if i >= 3
                && j >= 3
                && input_2d[i][j] == 'X'
                && input_2d[i - 1][j - 1] == 'M'
                && input_2d[i - 2][j - 2] == 'A'
                && input_2d[i - 3][j - 3] == 'S'
            {
                xmas_count += 1;
            }
        }
    }
    Some(xmas_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_2d = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let i_len = input_2d.len();
    let j_len = input_2d[0].len();
    let mut xmas_count = 0;
    let mut clon = input_2d.clone();
    for i in 0..i_len {
        for j in 0..j_len {
            if i < 1 || i + 1 >= i_len || j < 1 || j + 1 >= j_len {
                continue;
            }
            if input_2d[i][j] != 'A' {
                continue;
            }
            let top_left = (input_2d[i - 1][j - 1] == 'M' && input_2d[i + 1][j + 1] == 'S')
                || (input_2d[i - 1][j - 1] == 'S' && input_2d[i + 1][j + 1] == 'M');
            let bot_left = (input_2d[i + 1][j - 1] == 'M' && input_2d[i - 1][j + 1] == 'S')
                || (input_2d[i + 1][j - 1] == 'S' && input_2d[i - 1][j + 1] == 'M');
            if top_left && bot_left {
                clon[i][j] = '.';
                xmas_count += 1;
            }
        }
    }
    Some(xmas_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
