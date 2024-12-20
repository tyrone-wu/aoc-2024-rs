use std::collections::{HashMap, HashSet};

advent_of_code::solution!(20);

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

struct Map(Vec<Vec<char>>);

impl Map {
    fn s(&self, p: (usize, usize)) -> char {
        self.0[p.1][p.0]
    }

    fn s_up(&self, p: (usize, usize)) -> Option<char> {
        (p.1 > 0).then(|| self.0[p.1 - 1][p.0])
    }

    fn s_right(&self, p: (usize, usize)) -> Option<char> {
        (p.0 + 1 < self.0[0].len()).then(|| self.0[p.1][p.0 + 1])
    }

    fn s_down(&self, p: (usize, usize)) -> Option<char> {
        (p.1 + 1 < self.0.len()).then(|| self.0[p.1 + 1][p.0])
    }

    fn s_left(&self, p: (usize, usize)) -> Option<char> {
        (p.0 > 0).then(|| self.0[p.1][p.0 - 1])
    }
}

fn dist_map(map: &Map, p: (usize, usize), ps: usize, dists: &mut HashMap<(usize, usize), usize>) {
    if map.s(p) == 'E' {
        return;
    }
    let (x, y) = p;
    let next = if matches!(map.s_up(p), Some(s) if s != '#') && !dists.contains_key(&(x, y - 1)) {
        Some((x, y - 1))
    } else if matches!(map.s_right(p), Some(s) if s != '#') && !dists.contains_key(&(x + 1, y)) {
        Some((x + 1, y))
    } else if matches!(map.s_down(p), Some(s) if s != '#') && !dists.contains_key(&(x, y + 1)) {
        Some((x, y + 1))
    } else if matches!(map.s_left(p), Some(s) if s != '#') && !dists.contains_key(&(x - 1, y)) {
        Some((x - 1, y))
    } else {
        None
    };
    if let Some(next) = next {
        dists.insert(next, ps + 1);
        dist_map(map, next, ps + 1, dists);
    }
}

fn dfs(
    map: &Map,
    visited: &mut HashSet<(usize, usize)>,
    p: (usize, usize),
    dists: &HashMap<(usize, usize), usize>,
    hacks: usize,
    saves: &mut Vec<usize>,
) {
    if map.s(p) == 'E' {
        return;
    }

    for (y, row) in map.0.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            let man_dist = x.abs_diff(p.0) + y.abs_diff(p.1);
            if man_dist > hacks || *c == '#' {
                continue;
            }
            if *dists.get(&(x, y)).unwrap() > *dists.get(&p).unwrap() + man_dist {
                saves.push(dists.get(&(x, y)).unwrap() - (dists.get(&p).unwrap() + man_dist));
            }
        }
    }
    let (x, y) = p;
    let next = if matches!(map.s_up(p), Some(s) if s != '#') && !visited.contains(&(x, y - 1)) {
        Some((x, y - 1))
    } else if matches!(map.s_right(p), Some(s) if s != '#') && !visited.contains(&(x + 1, y)) {
        Some((x + 1, y))
    } else if matches!(map.s_down(p), Some(s) if s != '#') && !visited.contains(&(x, y + 1)) {
        Some((x, y + 1))
    } else if matches!(map.s_left(p), Some(s) if s != '#') && !visited.contains(&(x - 1, y)) {
        Some((x - 1, y))
    } else {
        None
    };
    if let Some(next) = next {
        visited.insert(next);
        dfs(map, visited, next, dists, hacks, saves);
    }
}

fn solve(input: &str, hacks: usize) -> u32 {
    let map = Map(parse_input(input));
    let (s, _) = get_se(&map.0);
    let mut dists = HashMap::from([(s, 0)]);
    dist_map(&map, s, 0, &mut dists);
    let mut saves = Vec::new();
    dfs(&map, &mut HashSet::from([s]), s, &dists, hacks, &mut saves);
    let freq = saves.into_iter().fold(HashMap::new(), |mut acc, save| {
        acc.entry(save).and_modify(|f| *f += 1).or_insert(1);
        acc
    });
    freq.into_iter()
        .filter_map(|(save, count)| (save >= 100).then_some(count))
        .sum::<u32>()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, 20))
}

// fn bfs(map: &Map, s: (usize, usize), total_hacks: u8, cuttoff: Option<u32>) -> HashMap<u32, HashSet<(Option<(usize, usize)>, Option<(usize, usize)>)>> {
//     let mut bfs = VecDeque::from([(s, 0, total_hacks, (None, None))]);
//     let mut visited = HashSet::from([(s, (None, None))]);
//     let mut finished: HashMap<u32, HashSet<(Option<(usize, usize)>, Option<(usize, usize)>)>> = HashMap::new();

//     while let Some((curr, mut ps, hacks, mut wall_hack)) = bfs.pop_front() {
//         let s_curr = map.s(curr);
//         if s_curr == 'E' {
//             if wall_hack.1.is_none() {
//                 wall_hack = (wall_hack.0, Some(curr));
//             }
//             finished.entry(ps)
//                 .and_modify(|v| { v.insert(wall_hack); })
//                 .or_insert(HashSet::from([wall_hack]));
//             continue;
//         }
//         if s_curr == '#' && hacks == 0 {
//             continue;
//         }
//         if matches!(cuttoff, Some(cuttoff) if cuttoff <= ps) {
//             continue;
//         }
//         println!("{}", bfs.len());

//         ps += 1;
//         let (x, y) = curr;
//         let (hacks_active, hacks_used) = match wall_hack {
//             (None, None) => (false, false),
//             (None, Some(_)) => unreachable!(),
//             (Some(_), None) => (true, false),
//             (Some(_), Some(_)) => (false, true),
//         };

//         // up
//         if let Some(s_next) = map.s_up(curr) {
//             let next = (x, y - 1);
//             // no hacks active - don't activate
//             if !hacks_active && s_next != '#' && !visited.contains(&(next, wall_hack)) {
//                 visited.insert((next, wall_hack));
//                 bfs.push_back((next, ps, hacks, wall_hack));
//             }
//             // no hacks active - activate
//             if !hacks_active && !hacks_used && hacks > 0 && !visited.contains(&(next, (Some(curr), None))) {
//                 visited.insert((next, (Some(curr), None)));
//                 bfs.push_back((next, ps, hacks - 1, (Some(curr), None)));
//             }
//             // hacks active - de-activate
//             if hacks_active && s_next != '#' && hacks > 0 && !visited.contains(&(next, (wall_hack.0, Some(next)))) {
//                 visited.insert((next, (wall_hack.0, Some(next))));
//                 bfs.push_back((next, ps, hacks - 1, (wall_hack.0, Some(next))));
//             }
//             // hacks active - continue activate
//             if hacks_active && hacks > 0 && !visited.contains(&(next, wall_hack)) {
//                 visited.insert((next, wall_hack));
//                 bfs.push_back((next, ps, hacks - 1, wall_hack));
//             }
//         }
//         // right
//         if let Some(s_next) = map.s_right(curr) {
//             let next = (x + 1, y);
//             // no hacks active - don't activate
//             if !hacks_active && s_next != '#' && !visited.contains(&(next, wall_hack)) {
//                 visited.insert((next, wall_hack));
//                 bfs.push_back((next, ps, hacks, wall_hack));
//             }
//             // no hacks active - activate
//             if !hacks_active && !hacks_used && hacks > 0 && !visited.contains(&(next, (Some(curr), None))) {
//                 visited.insert((next, (Some(curr), None)));
//                 bfs.push_back((next, ps, hacks - 1, (Some(curr), None)));
//             }
//             // hacks active - de-activate
//             if hacks_active && s_next != '#' && hacks > 0 && !visited.contains(&(next, (wall_hack.0, Some(next)))) {
//                 visited.insert((next, (wall_hack.0, Some(next))));
//                 bfs.push_back((next, ps, hacks - 1, (wall_hack.0, Some(next))));
//             }
//             // hacks active - continue activate
//             if hacks_active && hacks > 0 && !visited.contains(&(next, wall_hack)) {
//                 visited.insert((next, wall_hack));
//                 bfs.push_back((next, ps, hacks - 1, wall_hack));
//             }
//         }
//         // down
//         if let Some(s_next) = map.s_down(curr) {
//             let next = (x, y + 1);
//             // no hacks active - don't activate
//             if !hacks_active && s_next != '#' && !visited.contains(&(next, wall_hack)) {
//                 visited.insert((next, wall_hack));
//                 bfs.push_back((next, ps, hacks, wall_hack));
//             }
//             // no hacks active - activate
//             if !hacks_active && !hacks_used && hacks > 0 && !visited.contains(&(next, (Some(curr), None))) {
//                 visited.insert((next, (Some(curr), None)));
//                 bfs.push_back((next, ps, hacks - 1, (Some(curr), None)));
//             }
//             // hacks active - de-activate
//             if hacks_active && s_next != '#' && hacks > 0 && !visited.contains(&(next, (wall_hack.0, Some(next)))) {
//                 visited.insert((next, (wall_hack.0, Some(next))));
//                 bfs.push_back((next, ps, hacks - 1, (wall_hack.0, Some(next))));
//             }
//             // hacks active - continue activate
//             if hacks_active && hacks > 0 && !visited.contains(&(next, wall_hack)) {
//                 visited.insert((next, wall_hack));
//                 bfs.push_back((next, ps, hacks - 1, wall_hack));
//             }
//         }
//         // left
//         if let Some(s_next) = map.s_left(curr) {
//             let next = (x - 1, y);
//             // no hacks active - don't activate
//             if !hacks_active && s_next != '#' && !visited.contains(&(next, wall_hack)) {
//                 visited.insert((next, wall_hack));
//                 bfs.push_back((next, ps, hacks, wall_hack));
//             }
//             // no hacks active - activate
//             if !hacks_active && !hacks_used && hacks > 0 && !visited.contains(&(next, (Some(curr), None))) {
//                 visited.insert((next, (Some(curr), None)));
//                 bfs.push_back((next, ps, hacks - 1, (Some(curr), None)));
//             }
//             // hacks active - de-activate
//             if hacks_active && s_next != '#' && hacks > 0 && !visited.contains(&(next, (wall_hack.0, Some(next)))) {
//                 visited.insert((next, (wall_hack.0, Some(next))));
//                 bfs.push_back((next, ps, hacks - 1, (wall_hack.0, Some(next))));
//             }
//             // hacks active - continue activate
//             if hacks_active && hacks > 0 && !visited.contains(&(next, wall_hack)) {
//                 visited.insert((next, wall_hack));
//                 bfs.push_back((next, ps, hacks - 1, wall_hack));
//             }
//         }
//     }

//     finished
// }

// fn dfs(map: &[Vec<char>], visited: &mut HashSet<(usize, usize, i32)>, x: usize, y: usize, ps: u32, hacks: i32, finished: &mut HashSet<(u32, i32)>) {
//     let x_max = map[0].len() - 1;
//     let y_max = map.len() - 1;
//     if map[y][x] == 'E' {
//         finished.insert((ps, hacks));
//         return;
//     }
//     if map[y][x] == '#' && hacks == 0 {
//         return;
//     }
//     // up
//     if y > 0 {
//         if map[y - 1][x] != '#' && !visited.contains(&(x, y - 1, hacks)) {
//             visited.insert((x, y - 1, hacks));
//             dfs(map, visited, x, y - 1, ps + 1, hacks, finished);
//         } else if ((map[y][x] != '#' && hacks == 2) || (map[y][x] == '#' && hacks == 1)) && map[y - 1][x] == '#' && !visited.contains(&(x, y - 1, hacks - 1)) {
//             visited.insert((x, y - 1, hacks - 1));
//             dfs(map, visited, x, y - 1, ps + 1, hacks - 1, finished);
//         }
//     }
//     // right
//     if x + 1 <= x_max {
//         if map[y][x + 1] != '#' && !visited.contains(&(x + 1, y, hacks)) {
//             visited.insert((x + 1, y, hacks));
//             dfs(map, visited, x + 1, y, ps + 1, hacks, finished);
//         } else if ((map[y][x] != '#' && hacks == 2) || (map[y][x] == '#' && hacks == 1)) && map[y][x + 1] == '#' && !visited.contains(&(x + 1, y, hacks - 1)) {
//             visited.insert((x + 1, y, hacks - 1));
//             dfs(map, visited, x + 1, y, ps + 1, hacks - 1, finished);
//         }
//     }
//     // down
//     if y + 1 <= y_max {
//         if map[y + 1][x] != '#' && !visited.contains(&(x, y + 1, hacks)) {
//             visited.insert((x, y + 1, hacks));
//             dfs(map, visited, x, y + 1, ps + 1, hacks, finished);
//         } else if ((map[y][x] != '#' && hacks == 2) || (map[y][x] == '#' && hacks == 1)) && map[y + 1][x] == '#' && !visited.contains(&(x, y + 1, hacks - 1)) {
//             visited.insert((x, y + 1, hacks - 1));
//             dfs(map, visited, x, y + 1, ps + 1, hacks - 1, finished);
//         }
//     }
//     // left
//     if x > 0 {
//         if map[y][x - 1] != '#' && !visited.contains(&(x - 1, y, hacks)) {
//             visited.insert((x - 1, y, hacks));
//             dfs(map, visited, x - 1, y, ps + 1, hacks, finished);
//         } else if ((map[y][x] != '#' && hacks == 2) || (map[y][x] == '#' && hacks == 1)) && map[y][x - 1] == '#' && !visited.contains(&(x - 1, y, hacks - 1)) {
//             visited.insert((x - 1, y, hacks - 1));
//             dfs(map, visited, x - 1, y, ps + 1, hacks - 1, finished);
//         }
//     }
// }

// pub fn part_one(input: &str) -> Option<u32> {
//     let map = Map(parse_input(input));
//     let (s, _) = get_se(&map.0);
//     let no_hacks = bfs(&map, s, 0, None);
//     let no_hacks_ps = no_hacks.into_iter().map(|(ps, _)| ps).min().unwrap();

//     let hacks_ps = bfs(&map, s, 2, Some(no_hacks_ps));
//     let mut saves = 0;
//     for (ps, wall_hacks) in hacks_ps.into_iter() {
//         if no_hacks_ps >= ps + 100 {
//             saves += wall_hacks.len();
//         }
//     }
//     Some(saves)

//     let map = parse_input(input);
//     let (s, _) = get_se(&map);
//     let total_hacks = 2;

//     // let mut no_hacks = HashSet::new();
//     // dfs(&map, &mut HashSet::from([(s.0, s.1, 0)]), s.0, s.1, 0, 0, &mut no_hacks);
//     // let no_hacks_ps = no_hacks.into_iter().next().unwrap().0;

//     // let mut finished = HashSet::new();
//     // dfs(&map, HashSet::from([(s.0, s.1, 2)]), s.0, s.1, 0, 2, &mut finished);

//     let x_max = map[0].len() - 1;
//     let y_max = map.len() - 1;
//     let mut bfs = VecDeque::from([(s.0, s.1, 0, total_hacks, (None, None))]);
//     let mut visited = HashSet::from([(s.0, s.1, total_hacks, (None, None))]);
//     let mut finished: HashMap<(u32, i32), u32> = HashMap::new();

//     while let Some((x, y, ps, hacks, wall_hack)) = bfs.pop_front() {
//         if map[y][x] == 'E' {
//             finished.entry((ps, hacks)).and_modify(|v| *v += 1).or_insert(1);
//             continue;
//         }
//         if map[y][x] == '#' && hacks == 0 {
//             continue;
//         }

//         // up
//         if y > 0 {
//             if map[y - 1][x] != '#' && !visited.contains(&(x, y - 1, hacks, wall_hack)) {
//                 visited.insert((x, y - 1, hacks, wall_hack));
//                 bfs.push_back((x, y - 1, ps + 1, hacks, wall_hack));
//             } else if ((map[y][x] != '#' && hacks == 2) || (map[y][x] == '#' && hacks == 1)) && map[y - 1][x] == '#' && !visited.contains(&(x, y - 1, hacks - 1, wall_hack)) {
//                 let new_hack = match wall_hack {
//                     (None, None) => (Some((x, y - 1)), None),
//                     (pp, None) => (pp, Some((x, y - 1))),
//                     _ => unreachable!(),
//                 };
//                 visited.insert((x, y - 1, hacks - 1, new_hack));
//                 bfs.push_back((x, y - 1, ps + 1, hacks - 1, new_hack));
//             }
//         }
//         // right
//         if x + 1 <= x_max {
//             if map[y][x + 1] != '#' && !visited.contains(&(x + 1, y, hacks, wall_hack)) {
//                 visited.insert((x + 1, y, hacks, wall_hack));
//                 bfs.push_back((x + 1, y, ps + 1, hacks, wall_hack));
//             } else if ((map[y][x] != '#' && hacks == 2) || (map[y][x] == '#' && hacks == 1)) && map[y][x + 1] == '#' && !visited.contains(&(x + 1, y, hacks - 1, wall_hack)) {
//                 let new_hack = match wall_hack {
//                     (None, None) => (Some((x + 1, y)), None),
//                     (pp, None) => (pp, Some((x + 1, y))),
//                     _ => unreachable!(),
//                 };
//                 visited.insert((x + 1, y, hacks - 1, new_hack));
//                 bfs.push_back((x + 1, y, ps + 1, hacks - 1, new_hack));
//             }
//         }
//         // down
//         if y + 1 <= y_max {
//             if map[y + 1][x] != '#' && !visited.contains(&(x, y + 1, hacks, wall_hack)) {
//                 visited.insert((x, y + 1, hacks, wall_hack));
//                 bfs.push_back((x, y + 1, ps + 1, hacks, wall_hack));
//             } else if ((map[y][x] != '#' && hacks == 2) || (map[y][x] == '#' && hacks == 1)) && map[y + 1][x] == '#' && !visited.contains(&(x, y + 1, hacks - 1, wall_hack)) {
//                 let new_hack = match wall_hack {
//                     (None, None) => (Some((x, y + 1)), None),
//                     (pp, None) => (pp, Some((x, y + 1))),
//                     _ => unreachable!(),
//                 };
//                 visited.insert((x, y + 1, hacks - 1, new_hack));
//                 bfs.push_back((x, y + 1, ps + 1, hacks - 1, new_hack));
//             }
//         }
//         // left
//         if x > 0 {
//             if map[y][x - 1] != '#' && !visited.contains(&(x - 1, y, hacks, wall_hack)) {
//                 visited.insert((x - 1, y, hacks, wall_hack));
//                 bfs.push_back((x - 1, y, ps + 1, hacks, wall_hack));
//             } else if ((map[y][x] != '#' && hacks == 2) || (map[y][x] == '#' && hacks == 1)) && map[y][x - 1] == '#' && !visited.contains(&(x - 1, y, hacks - 1, wall_hack)) {
//                 let new_hack = match wall_hack {
//                     (None, None) => (Some((x - 1, y)), None),
//                     (pp, None) => (pp, Some((x - 1, y))),
//                     _ => unreachable!(),
//                 };
//                 visited.insert((x - 1, y, hacks - 1, new_hack));
//                 bfs.push_back((x - 1, y, ps + 1, hacks - 1, new_hack));
//             }
//         }
//     }

//     let no_hacks_ps = *finished.iter().filter_map(|((ps, hacks), _)| (hacks == &total_hacks).then_some(ps)).max().unwrap();
//     let mut saves = 0;
//     for ((ps, hacks), count) in finished.into_iter() {
//         if no_hacks_ps - 100 >= ps && hacks < 2 {
//             saves += count;
//         }
//     }
//     Some(saves)
// }

// pub fn part_two(input: &str) -> Option<u32> {
//     let map = parse_input(input);
//     let (s, _) = get_se(&map);

//     // let mut no_hacks = HashSet::new();
//     // dfs(&map, &mut HashSet::from([(s.0, s.1, 0)]), s.0, s.1, 0, 0, &mut no_hacks);
//     // let no_hacks_ps = no_hacks.into_iter().map(|(ps, _)| ps).min().unwrap();

//     let x_max = map[0].len() - 1;
//     let y_max = map.len() - 1;
//     let mut bfs = VecDeque::from([(s.0, s.1, 0, 20, (None, None))]);
//     let mut visited = HashSet::from([(s.0, s.1, (None, None))]);
//     let mut finished: HashMap<u32, HashSet<(Option<(usize, usize)>, Option<(usize, usize)>)>> = HashMap::new();

//     while let Some((x, y, ps, hacks, wall_hack)) = bfs.pop_front() {
//         if map[y][x] == 'E' {
//             finished.entry(ps)
//                 .and_modify(|v| { v.insert(wall_hack); })
//                 .or_insert(HashSet::from([wall_hack]));
//             continue;
//         }
//         if map[y][x] == '#' && hacks == 0 {
//             continue;
//         }
//         if no_hacks_ps <= ps {
//             continue;
//         }

//         // up
//         if y > 0 {
//             let new_hack = match wall_hack {
//                 (pp, None) if map[y][x] == '#' => (pp, Some((x, y - 1))),
//                 asdf => asdf,
//             };
//             if map[y - 1][x] != '#' && !visited.contains(&(x, y - 1, new_hack)) {
//                 visited.insert((x, y - 1, new_hack));
//                 bfs.push_back((x, y - 1, ps + 1, hacks, new_hack));
//             }
//             if map[y - 1][x] != '#' && !visited.contains(&(x, y - 1, (Some((x, y)), None))) {
//                 visited.insert((x, y - 1, new_hack));
//                 bfs.push_back((x, y - 1, ps + 1, hacks - 1, new_hack));
//             }

//             let new_hack = match wall_hack {
//                 (None, None) => (Some((x, y)), None),
//                 asdf => asdf,
//             };
//             if ((map[y][x] != '#' && hacks == 20) || (map[y][x] == '#' && hacks < 20 && hacks > 0)) && map[y - 1][x] == '#' && !visited.contains(&(x, y - 1, new_hack)) {
//                 visited.insert((x, y - 1, new_hack));
//                 bfs.push_back((x, y - 1, ps + 1, hacks - 1, new_hack));
//             }
//         }
//         // right
//         if x + 1 <= x_max {
//             let new_hack = match wall_hack {
//                 (pp, None) if map[y][x] == '#' => (pp, Some((x + 1, y))),
//                 asdf => asdf,
//             };
//             if map[y][x + 1] != '#' && !visited.contains(&(x + 1, y, new_hack)) {
//                 visited.insert((x + 1, y, new_hack));
//                 bfs.push_back((x + 1, y, ps + 1, hacks, new_hack));
//             }

//             let new_hack = match wall_hack {
//                 (None, None) => (Some((x, y)), None),
//                 asdf => asdf,
//             };
//             if ((map[y][x] != '#' && hacks == 20) || (map[y][x] == '#' && hacks < 20 && hacks > 0)) && map[y][x + 1] == '#' && !visited.contains(&(x + 1, y, new_hack)) {
//                 visited.insert((x + 1, y, new_hack));
//                 bfs.push_back((x + 1, y, ps + 1, hacks - 1, new_hack));
//             }
//         }
//         // down
//         if y + 1 <= y_max {
//             let new_hack = match wall_hack {
//                 (pp, None) if map[y][x] == '#' => (pp, Some((x, y + 1))),
//                 asdf => asdf,
//             };
//             if map[y + 1][x] != '#' && !visited.contains(&(x, y + 1, new_hack)) {
//                 visited.insert((x, y + 1, new_hack));
//                 bfs.push_back((x, y + 1, ps + 1, hacks, new_hack));
//             }

//             let new_hack = match wall_hack {
//                 (None, None) => (Some((x, y)), None),
//                 asdf => asdf,
//             };
//             if ((map[y][x] != '#' && hacks == 20) || (map[y][x] == '#' && hacks < 20 && hacks > 0)) && map[y + 1][x] == '#' && !visited.contains(&(x, y + 1, new_hack)) {
//                 visited.insert((x, y + 1, new_hack));
//                 bfs.push_back((x, y + 1, ps + 1, hacks - 1, new_hack));
//             }
//         }
//         // left
//         if x > 0 {
//             let new_hack = match wall_hack {
//                 (pp, None) if map[y][x] == '#' => (pp, Some((x - 1, y))),
//                 asdf => asdf,
//             };
//             if map[y][x - 1] != '#' && !visited.contains(&(x - 1, y,new_hack)) {
//                 visited.insert((x - 1, y, new_hack));
//                 bfs.push_back((x - 1, y, ps + 1, hacks, new_hack));
//             }

//             let new_hack = match wall_hack {
//                 (None, None) => (Some((x, y)), None),
//                 asdf => asdf,
//             };
//             if ((map[y][x] != '#' && hacks == 20) || (map[y][x] == '#' && hacks < 20 && hacks > 0)) && map[y][x - 1] == '#' && !visited.contains(&(x - 1, y, new_hack)) {
//                 visited.insert((x - 1, y, new_hack));
//                 bfs.push_back((x - 1, y, ps + 1, hacks - 1, new_hack));
//             }
//         }
//     }

//     let mut saves = 0;
//     for (ps, count) in finished.into_iter() {
//         if no_hacks_ps - 50 == ps {
//             saves += count.iter().filter(|(l, _)| l.is_some()).count();
//         }
//     }
//     Some(saves as u32)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
