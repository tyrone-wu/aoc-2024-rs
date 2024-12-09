advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let mut blocks: Vec<Option<usize>> =
        input
            .trim_end()
            .chars()
            .enumerate()
            .fold(Vec::new(), |mut acc, (i, c)| {
                let d = c.to_digit(10).unwrap();
                let block = (i % 2 == 0).then_some(i / 2 + i % 2);
                for _ in 0..d {
                    acc.push(block);
                }
                acc
            });

    let mut left = 0_usize;
    let mut right = blocks.len() - 1;
    while left <= right {
        if blocks[left].is_some() {
            left += 1;
            continue;
        }
        if blocks[right].is_none() {
            right -= 1;
            continue;
        }
        blocks[left] = blocks[right];
        blocks[right] = None;
    }

    let sum: usize = blocks
        .into_iter()
        .enumerate()
        .map(|(id, b)| if let Some(size) = b { id * size } else { 0 })
        .sum();
    Some(sum)
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Block {
    File { id: u64, count: u64, moved: bool },
    FreeSpace(u64),
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut blocks: Vec<Block> =
        input
            .trim_end()
            .chars()
            .enumerate()
            .fold(Vec::new(), |mut acc, (i, c)| {
                let d = c.to_digit(10).unwrap() as u64;
                let block = if i % 2 == 0 {
                    Block::File { id: (i / 2 + i % 2) as u64, count: d, moved: false }
                } else {
                    Block::FreeSpace(d)
                };
                acc.push(block);
                acc
            });

    let mut prev_blocks= vec![];
    while prev_blocks != blocks {
        prev_blocks = blocks.clone();
        let mut left = 0;
        while left < blocks.len() {
            if let Block::FreeSpace(space) = blocks[left] {
                for i in (left + 1..blocks.len()).rev() {
                    match blocks[i] {
                        Block::File { id, count, moved } if !moved => {
                            if space < count {
                                continue;
                            }

                            std::mem::swap(&mut blocks[i], &mut Block::FreeSpace(count));
                            blocks.insert(
                                left,
                                Block::File {
                                    id,
                                    count,
                                    moved: true,
                                },
                            );

                            blocks.remove(left + 1);
                            if space > count {
                                blocks.insert(left + 1, Block::FreeSpace(space - count));
                            }
                            break;
                        }
                        _ => {}
                    }
                }
            }
            left += 1;
        }
    }

    let mut i = 0;
    let sim: u64 = blocks.into_iter().fold(0, |mut acc, b| {
        match b {
            Block::File { id, count, .. } => {
                for _ in 0..count {
                    acc += i * id;
                    i += 1;
                }
            },
            Block::FreeSpace(space) => i += space,
        }
        acc
    });
    Some(sim)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
