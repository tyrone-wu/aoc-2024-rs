use std::{
    collections::{HashMap, VecDeque},
    usize,
};

use itertools::Itertools;

advent_of_code::solution!(21);

const NUMERIC: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    ['.', '0', 'A'],
];

const _DIRECTIONAL: [[char; 3]; 2] = [['.', '^', 'A'], ['<', 'v', '>']];

const PRUNE_HEURISTIC: usize = 0;

struct DirectionalLut(HashMap<char, HashMap<char, Vec<Vec<char>>>>);

impl DirectionalLut {
    fn new() -> Self {
        Self(HashMap::from([
            (
                'A',
                HashMap::from([
                    ('>', vec!["vA".chars().collect()]),
                    ('^', vec!["<A".chars().collect()]),
                    ('v', vec!["v<A".chars().collect(), "<vA".chars().collect()]),
                    ('<', vec!["v<<A".chars().collect()]),
                    ('A', vec!["A".chars().collect()]),
                ]),
            ),
            (
                '>',
                HashMap::from([
                    ('A', vec!["^A".chars().collect()]),
                    ('v', vec!["<A".chars().collect()]),
                    ('^', vec!["<^A".chars().collect(), "^<A".chars().collect()]),
                    ('<', vec!["<<A".chars().collect()]),
                    ('>', vec!["A".chars().collect()]),
                ]),
            ),
            (
                '^',
                HashMap::from([
                    ('A', vec![">A".chars().collect()]),
                    ('v', vec!["vA".chars().collect()]),
                    ('>', vec!["v>A".chars().collect(), ">vA".chars().collect()]),
                    ('<', vec!["v<A".chars().collect()]),
                    ('^', vec!["A".chars().collect()]),
                ]),
            ),
            (
                'v',
                HashMap::from([
                    ('^', vec!["^A".chars().collect()]),
                    ('>', vec![">A".chars().collect()]),
                    ('A', vec!["^>A".chars().collect(), ">^A".chars().collect()]),
                    ('<', vec!["<A".chars().collect()]),
                    ('v', vec!["A".chars().collect()]),
                ]),
            ),
            (
                '<',
                HashMap::from([
                    ('v', vec![">A".chars().collect()]),
                    ('^', vec![">^A".chars().collect()]),
                    ('>', vec![">>A".chars().collect()]),
                    ('A', vec![">>^A".chars().collect()]),
                    ('<', vec!["A".chars().collect()]),
                ]),
            ),
        ]))
    }

    fn get(&self, from: &char, to: &char) -> Vec<Vec<char>> {
        self.0.get(from).unwrap().get(to).unwrap().to_vec()
    }

    fn get_len(&self, from: &char, to: &char) -> usize {
        self.get(from, to)[0].len()
    }
}

fn find(grid: &[[char; 3]], target: &char) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == target {
                return (i, j);
            }
        }
    }
    unreachable!()
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn bfs(grid: &[[char; 3]], target: char, s: (usize, usize)) -> Vec<Vec<char>> {
    let mut bfs = VecDeque::from([(s, vec![], 1 << (s.0 * 3 + s.1))]);
    let mut possible_moves = Vec::new();
    while let Some(((i, j), moves, visited)) = bfs.pop_front() {
        if grid[i][j] == target {
            let mut mvs = moves.clone();
            mvs.push('A');
            possible_moves.push(mvs);
            continue;
        }

        let up = 1 << ((i - 1) * 3 + j);
        if i > 0 && grid[i - 1][j] != '.' && visited & up == 0 {
            let mut mvs = moves.clone();
            mvs.push('^');
            bfs.push_back(((i - 1, j), mvs, visited | up));
        }
        let right = 1 << (i * 3 + j + 1);
        if j + 1 < grid[0].len() && visited & right == 0 {
            let mut mvs = moves.clone();
            mvs.push('>');
            bfs.push_back(((i, j + 1), mvs, visited | right));
        }
        let down = 1 << ((i + 1) * 3 + j);
        if i + 1 < grid.len() && grid[i + 1][j] != '.' && visited & down == 0 {
            let mut mvs = moves.clone();
            mvs.push('v');
            bfs.push_back(((i + 1, j), mvs, visited | down));
        }
        let left = 1 << (i * 3 + j - 1);
        if j > 0 && grid[i][j - 1] != '.' && visited & left == 0 {
            let mut mvs = moves.clone();
            mvs.push('<');
            bfs.push_back(((i, j - 1), mvs, visited | left));
        }
    }
    possible_moves
}

fn possible_moves(grid: &[[char; 3]], seq: &[char], s: (usize, usize)) -> Vec<Vec<char>> {
    let mut move_combos = Vec::new();
    let mut pos = s;
    for c in seq.iter() {
        let c_moves = bfs(grid, *c, pos);
        pos = find(grid, c);
        move_combos.push(c_moves);
    }
    move_combos
        .into_iter()
        .map(|c| c.into_iter())
        .multi_cartesian_product()
        .map(|c| c.into_iter().flatten().collect())
        .collect()
}

// fn directional_moves(
//     possible_moves: Vec<Vec<char>>,
//     directional_lut: &DirectionalLut,
// ) -> Vec<Vec<char>> {
//     let mut new_possible_moves = Vec::new();
//     let mut min_len = usize::MAX;
//     let mut max_score = 0;
//     for moves in possible_moves.iter() {
//         let mut new_moves: Vec<Vec<char>> = Vec::new();
//         let mut p = 'A';
//         let mut exit = false;
//         for mv in moves.into_iter() {
//             let possible_moves = directional_lut.get(&p, &mv);

//             match new_moves.len() {
//                 0 => new_moves.push(possible_moves),
//                 1 => new_moves[0].extend(possible_moves),
//                 _ => {
//                     let mut new_moves_extended = Vec::new();
//                     for n_mv in new_moves.iter() {
//                         if n_mv.len() + possible_moves.len() > min_len {
//                             exit = true;
//                             break;
//                         }

//                         let mut n_mv = n_mv.clone();
//                         n_mv.extend(possible_moves.clone());
//                         new_moves_extended.push(n_mv);
//                         if exit {
//                             break;
//                         }
//                     }
//                     new_moves = new_moves_extended;
//                 }
//             }
//             if exit {
//                 break;
//             }
//             p = *mv;
//         }
//         if exit {
//             continue;
//         }

//         if new_moves.len() > 1 {
//             for n_mvs in new_moves.into_iter() {
//                 let score = score(&n_mvs);

//                 if n_mvs.len() < min_len {
//                     new_possible_moves.clear();
//                     min_len = n_mvs.len();
//                     max_score = score;
//                     new_possible_moves.push(n_mvs);
//                 } else if n_mvs.len() == min_len && score > max_score {
//                     new_possible_moves.clear();
//                     max_score = score;
//                     new_possible_moves.push(n_mvs);
//                 } else if n_mvs.len() == min_len && score == max_score {
//                     new_possible_moves.push(n_mvs);
//                 }
//             }
//         } else {
//             new_possible_moves.push(new_moves.into_iter().next().unwrap());
//         }
//     }
//     new_possible_moves
// }

fn score(moves: &[char]) -> usize {
    let mut score = 0;
    let mut last_mv = &moves[0];
    for mv in moves.iter().skip(1) {
        if last_mv == mv {
            score += 1;
        }
        last_mv = mv;
    }
    score
}

fn prune(possible_moves: &mut Vec<Vec<char>>) {
    let min_len = possible_moves.iter().map(|mvs| mvs.len()).min().unwrap();
    possible_moves.retain(|mvs| mvs.len() <= min_len + PRUNE_HEURISTIC);
    let max_score = possible_moves.iter().map(|mvs| score(mvs)).max().unwrap();
    possible_moves.retain(|mvs| score(mvs) + PRUNE_HEURISTIC >= max_score);
}

// fn directional_moves_repeat(
//     mut possible_moves: Vec<Vec<char>>,
//     directional_lut: &DirectionalLut,
//     dir_robots: u8,
// ) -> Vec<Vec<char>> {
//     for _ in 0..dir_robots {
//         possible_moves = directional_moves(possible_moves, &directional_lut);
//         prune(&mut possible_moves);
//         // println!("{} - {}", i, possible_moves.len());
//     }
//     possible_moves
// }

fn dfs(
    dir_lut: &DirectionalLut,
    from: char,
    to: char,
    depth: u8,
    cache: &mut HashMap<(char, char, u8), usize>,
) -> usize {
    if depth == 0 {
        return dir_lut.get_len(&from, &to);
    }
    if let Some(dist) = cache.get(&(from, to, depth)) {
        return *dist;
    }

    let mut possible_moves = dir_lut.get(&from, &to);
    let mut min_dist = usize::MAX;
    for mvs in possible_moves.iter_mut() {
        mvs.insert(0, 'A');

        let mut dist = 0;
        for i in 0..mvs.len() - 1 {
            dist += dfs(dir_lut, mvs[i], mvs[i + 1], depth - 1, cache);
        }
        min_dist = min_dist.min(dist);
    }

    cache.insert((from, to, depth), min_dist);
    min_dist
}

fn solve(input: &str, dir_robots: u8) -> usize {
    let codes = parse_input(input);
    let dir_lut = DirectionalLut::new();
    let mut cache = HashMap::new();
    let mut complexity = 0;
    for code in codes.iter() {
        let code_num = code[..3]
            .iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let mut code_possible_moves = possible_moves(&NUMERIC, &code, (3, 2));
        prune(&mut code_possible_moves);

        let mut min_dist = usize::MAX;
        for mvs in code_possible_moves.iter_mut() {
            mvs.insert(0, 'A');

            let mut dist = 0;
            for i in 0..mvs.len() - 1 {
                dist += dfs(&dir_lut, mvs[i], mvs[i + 1], dir_robots - 1, &mut cache);
            }
            min_dist = min_dist.min(dist);
        }

        // let possible_moves =
        //     directional_moves_repeat(code_possible_moves, &directional_lut, dir_robots);

        // let min_len = possible_moves.iter().map(|mvs| mvs.len()).min().unwrap();
        complexity += code_num * min_dist;
    }
    complexity
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve(input, 25))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
