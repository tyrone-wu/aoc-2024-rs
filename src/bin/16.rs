use std::{
    collections::{HashMap, HashSet, VecDeque},
    u32,
};

advent_of_code::solution!(16);

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum Dir {
    North,
    South,
    East,
    West,
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_se(map: &[Vec<char>]) -> ((usize, usize), (usize, usize)) {
    let mut s = (0, 0);
    let mut e = (0, 0);
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            match c {
                'S' => s = (x, y),
                'E' => e = (x, y),
                _ => {}
            }
        }
    }
    (s, e)
}

fn add_score(prev_dir: &Dir, dir: &Dir) -> u32 {
    if prev_dir == dir {
        1
    } else {
        1000 + 1
    }
}

fn min_option(a: Option<u32>, b: Option<&u32>) -> Option<u32> {
    match (a, b.copied()) {
        (None, None) => None,
        (None, Some(b)) => Some(b),
        (Some(a), None) => Some(a),
        (Some(a), Some(b)) => Some(a.min(b)),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let maze = parse_input(input);
    let (s, e) = get_se(&maze);
    let mut bfs = VecDeque::from([(s.0, s.1, Dir::East, 0u32)]);
    // let mut visited = HashSet::from([(s.0, s.1, Dir::East)]);
    let mut min_maze_scores: HashMap<(usize, usize, Dir), u32> = HashMap::new();
    while let Some((x, y, dir, score)) = bfs.pop_front() {
        if let Some(m_score) = min_maze_scores.get(&(x, y, dir)) {
            if *m_score <= score {
                continue;
            }
        }
        min_maze_scores.insert((x, y, dir), score);
        if maze[y][x] == 'E' {
            continue;
        }

        // north
        if maze[y - 1][x] != '#' {
            // && !min_maze_scores.contains_key(&(x, y - 1, Dir::North)) { // !visited.contains(&(x, y - 1, Dir::North)) {
            let add = add_score(&dir, &Dir::North);
            bfs.push_back((x, y - 1, Dir::North, score + add));
            // visited.insert((x, y - 1, Dir::North));
        }
        // east
        if maze[y][x + 1] != '#' {
            // && !min_maze_scores.contains_key(&(x + 1, y, Dir::East)) { // !visited.contains(&(x + 1, y, Dir::East)) {
            let add = add_score(&dir, &Dir::East);
            bfs.push_back((x + 1, y, Dir::East, score + add));
            // visited.insert((x + 1, y, Dir::East));
        }
        // south
        if maze[y + 1][x] != '#' {
            // && !min_maze_scores.contains_key(&(x, y + 1, Dir::South)) { // !visited.contains(&(x, y + 1, Dir::South)) {
            let add = add_score(&dir, &Dir::South);
            bfs.push_back((x, y + 1, Dir::South, score + add));
            // visited.insert((x, y + 1, Dir::South));
        }
        // west
        if maze[y][x - 1] != '#' {
            // && !min_maze_scores.contains_key(&(x - 1, y, Dir::West)) { // !visited.contains(&(x - 1, y, Dir::West)) {
            let add = add_score(&dir, &Dir::West);
            bfs.push_back((x - 1, y, Dir::West, score + add));
            // visited.insert((x - 1, y, Dir::West));
        }
    }

    min_option(
        min_option(
            min_option(
                min_maze_scores.get(&(e.0, e.1, Dir::North)).copied(),
                min_maze_scores.get(&(e.0, e.1, Dir::East)),
            ),
            min_maze_scores.get(&(e.0, e.1, Dir::South)),
        ),
        min_maze_scores.get(&(e.0, e.1, Dir::West)),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let maze = parse_input(input);
    let (s, _) = get_se(&maze);
    let mut bfs = VecDeque::from([(s.0, s.1, Dir::East, 0u32, HashSet::from([(s.0, s.1)]))]);
    let mut min_maze_scores: HashMap<(usize, usize, Dir), (u32, HashSet<(usize, usize)>)> =
        HashMap::new();
    let mut best_spots: HashSet<(usize, usize)> = HashSet::new();
    let mut min = u32::MAX;
    while let Some((x, y, dir, score, mut path)) = bfs.pop_front() {
        if let Some((m_score, _)) = min_maze_scores.get(&(x, y, dir)) {
            if *m_score < score {
                continue;
            }
        }
        path.insert((x, y));
        min_maze_scores.insert((x, y, dir), (score, path.clone()));
        if maze[y][x] == 'E' {
            if score == min {
                best_spots.extend(path.iter());
            } else if score < min {
                best_spots.clear();
                best_spots.extend(path.iter());
            }
            min = min.min(score);
            continue;
        }

        // north
        if maze[y - 1][x] != '#' {
            let add = add_score(&dir, &Dir::North);
            bfs.push_back((x, y - 1, Dir::North, score + add, path.clone()));
        }
        // east
        if maze[y][x + 1] != '#' {
            let add = add_score(&dir, &Dir::East);
            bfs.push_back((x + 1, y, Dir::East, score + add, path.clone()));
        }
        // south
        if maze[y + 1][x] != '#' {
            let add = add_score(&dir, &Dir::South);
            bfs.push_back((x, y + 1, Dir::South, score + add, path.clone()));
        }
        // west
        if maze[y][x - 1] != '#' {
            let add = add_score(&dir, &Dir::West);
            bfs.push_back((x - 1, y, Dir::West, score + add, path.clone()));
        }
    }

    // for y in 0..maze.len() {
    //     for x in 0..maze[0].len() {
    //         if best_spots.contains(&(x, y)) {
    //             maze[y][x] = 'O'
    //         }
    //     }
    // }
    // for row in maze.iter() {
    //     println!("{}", row.iter().collect::<String>());
    // }

    // idk how but the top left corner is somehow included in the best paths,
    // therefore we `- 1`...
    // edit: nvm i did (x, x) in the bfs instead of (x, y) 🫠
    Some(best_spots.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
