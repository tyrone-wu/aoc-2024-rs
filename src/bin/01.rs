use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut list_one = Vec::new();
    let mut list_two = Vec::new();
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        list_one.push(iter.next().unwrap().parse::<u32>().unwrap());
        list_two.push(iter.next().unwrap().parse::<u32>().unwrap());
    }
    list_one.sort();
    list_two.sort();
    let diff_sum = list_one
        .into_iter()
        .zip(list_two)
        .map(|(one, two)| one.abs_diff(two))
        .sum();
    Some(diff_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut list_one = Vec::new();
    let mut freq_two = HashMap::new();
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let val_one = iter.next().unwrap().parse::<u32>().unwrap();
        list_one.push(val_one);
        let val_two = iter.next().unwrap().parse::<u32>().unwrap();
        freq_two.entry(val_two).and_modify(|e| *e += 1).or_insert(1);
    }
    let similarity_score = list_one
        .into_iter()
        .map(|val| val * freq_two.get(&val).unwrap_or(&0))
        .sum();
    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
