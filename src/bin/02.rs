advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_count = 0;
    for line in input.lines() {
        let report: Vec<u32> = line
            .split_whitespace()
            .map(|v| v.parse::<u32>().unwrap())
            .collect();
        let mut safe = report.is_sorted() || report.iter().rev().is_sorted();
        for (i, curr) in report.iter().enumerate().skip(1) {
            let prev = report[i - 1];
            let diff = curr.abs_diff(prev);
            if diff < 1 || diff > 3 {
                safe = false;
                break;
            }
        }
        if safe {
            safe_count += 1;
        }
    }
    Some(safe_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe_count = 0;
    for line in input.lines() {
        let report: Vec<u32> = line
            .split_whitespace()
            .map(|v| v.parse::<u32>().unwrap())
            .collect();
        let mut safe = report.is_sorted() || report.iter().rev().is_sorted();
        for (i, curr) in report.iter().enumerate().skip(1) {
            let prev = report[i - 1];
            let diff = curr.abs_diff(prev);
            if diff < 1 || diff > 3 {
                safe = false;
                break;
            }
        }
        if safe {
            safe_count += 1;
        } else {
            for skip_i in 0..report.len() {
                let mut try_report: Vec<u32> = report[..skip_i].into();
                try_report.extend_from_slice(&report[(skip_i + 1)..]);
                safe = try_report.is_sorted() || try_report.iter().rev().is_sorted();
                for (i, curr) in try_report.iter().enumerate().skip(1) {
                    let prev = try_report[i - 1];
                    let diff = curr.abs_diff(prev);
                    if diff < 1 || diff > 3 {
                        safe = false;
                        break;
                    }
                }
                if safe {
                    safe_count += 1;
                    break;
                }
            }
        }
    }
    Some(safe_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
