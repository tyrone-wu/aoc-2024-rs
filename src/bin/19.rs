use cached::proc_macro::cached;

advent_of_code::solution!(19);

fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let mut split = input.split("\n\n");
    let available_patterns = split
        .next()
        .unwrap()
        .split(", ")
        .map(|pat| pat.to_owned())
        .collect();
    let designs = split
        .next()
        .unwrap()
        .lines()
        .map(|line| line.to_owned())
        .collect();
    (available_patterns, designs)
}

fn dfs(available_patterns: &[String], target: &str) -> bool {
    if target.is_empty() {
        return true;
    }
    for pat in available_patterns.iter() {
        if let Some(design) = target.strip_suffix(pat) {
            if dfs(available_patterns, design) {
                return true;
            }
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<usize> {
    let (available_patterns, designs) = parse_input(input);
    Some(
        designs
            .into_iter()
            .filter(|d| dfs(&available_patterns, d))
            .count(),
    )
}

#[cached]
fn dfs_p2(available_patterns: Vec<String>, target: String) -> u64 {
    if target.is_empty() {
        return 1;
    }
    let mut arrangements = 0;
    for pat in available_patterns.iter() {
        if let Some(design) = target.strip_suffix(pat) {
            arrangements += dfs_p2(available_patterns.clone(), design.to_owned());
        }
    }
    arrangements
}

pub fn part_two(input: &str) -> Option<u64> {
    let (available_patterns, designs) = parse_input(input);
    let filtered_designs: Vec<String> = designs
        .into_iter()
        .filter(|d| dfs(&available_patterns, d))
        .collect();
    Some(
        filtered_designs
            .into_iter()
            .map(|d| dfs_p2(available_patterns.clone(), d))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
