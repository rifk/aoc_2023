use eyre::eyre;
use std::collections::{HashSet, VecDeque};
use utils::derive::aoc;

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c as u64 - '0' as u64)
                .collect::<Vec<u64>>()
        })
        .collect()
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Search {
    loss: u64,
    pos: (usize, usize),
    dir: Dir,
    straight: u8,
}

fn next_steps(
    max_i: usize,
    max_j: usize,
    pos: &(usize, usize),
    dir: &Dir,
) -> Vec<((usize, usize), Dir)> {
    let mut steps = vec![];
    if *dir != Dir::Down && pos.0 > 0 {
        steps.push(((pos.0 - 1, pos.1), Dir::Up));
    }
    if *dir != Dir::Up && pos.0 < max_i - 1 {
        steps.push(((pos.0 + 1, pos.1), Dir::Down));
    }
    if *dir != Dir::Right && pos.1 > 0 {
        steps.push(((pos.0, pos.1 - 1), Dir::Left));
    }
    if *dir != Dir::Left && pos.1 < max_j - 1 {
        steps.push(((pos.0, pos.1 + 1), Dir::Right));
    }
    steps
}

#[aoc(day17, part1)]
fn solve_one(input: &str) -> Result<String> {
    use Dir::*;
    let grid = &parse_input(input);
    let max_i = grid.len();
    let max_j = grid[0].len();

    let mut lowest = vec![vec![None; max_j]; max_i];
    lowest[0][0] = Some(0);
    lowest[1][0] = Some(grid[1][0]);
    lowest[0][1] = Some(grid[0][1]);

    let mut search = VecDeque::new();
    search.push_back(Search {
        loss: grid[1][0],
        pos: (1, 0),
        dir: Down,
        straight: 0,
    });
    search.push_back(Search {
        loss: grid[0][1],
        pos: (0, 1),
        dir: Right,
        straight: 0,
    });

    while lowest[max_i - 1][max_j - 1].is_none() {
        // sort by loss
        search.make_contiguous().sort_by_key(|s| s.loss);

        // take all elements with lowest loss
        let mut low_loss = HashSet::new();
        let first = search.pop_front().ok_or(eyre!("empty search vec"))?;
        while let Some(f) = search.front() {
            if f.loss == first.loss {
                low_loss.insert(search.pop_front().ok_or(eyre!("empty search vec"))?);
            } else {
                break;
            }
        }
        low_loss.insert(first);

        // take next step of lowest loss
        let mut low_loss = low_loss
            .into_iter()
            .flat_map(|s| {
                let search = s.clone();
                next_steps(max_i, max_j, &search.pos, &search.dir)
                    .into_iter()
                    .filter_map(move |n| {
                        if n.1 == search.dir && search.straight >= 2 {
                            None
                        } else {
                            Some(Search {
                                loss: search.loss + grid[n.0 .0][n.0 .1],
                                pos: n.0,
                                straight: if n.1 == search.dir {
                                    search.straight + 1
                                } else {
                                    0
                                },
                                dir: n.1,
                            })
                        }
                    })
            })
            .collect::<Vec<Search>>();
        // sort next steps
        // if lowest not yet set; set it and add back into search
        // if lowest is set already; check not falling too far behind and add back into search
        low_loss.sort_by_key(|s| s.loss);
        low_loss.into_iter().for_each(|s| {
            if let Some(l) = lowest[s.pos.0][s.pos.1] {
                // 18 is worse case additional loss when going around instead of straight
                if s.loss <= l || s.loss - l <= 18 {
                    search.push_back(s);
                }
            } else {
                lowest[s.pos.0][s.pos.1] = Some(s.loss);
                search.push_back(s);
            }
        });
    }

    lowest[max_i - 1][max_j - 1]
        .ok_or(eyre!("no finish value"))
        .map(|v| v.to_string())
}

fn next_steps2(
    max_i: usize,
    max_j: usize,
    pos: &(usize, usize),
    dir: &Dir,
    straight: u8,
) -> Vec<((usize, usize), Dir, usize)> {
    let mut steps = vec![];
    fn num_steps(cur_dir: &Dir, to_dir: Dir, straight: u8) -> Option<usize> {
        if cur_dir == &to_dir {
            if straight >= 10 {
                None
            } else {
                Some(1)
            }
        } else {
            match (cur_dir, &to_dir) {
                (Dir::Up, Dir::Down)
                | (Dir::Down, Dir::Up)
                | (Dir::Left, Dir::Right)
                | (Dir::Right, Dir::Left) => None,
                _ => Some(4),
            }
        }
    }
    if let Some(s) = num_steps(dir, Dir::Up, straight) {
        if pos.0 >= s {
            steps.push(((pos.0 - s, pos.1), Dir::Up, s));
        }
    }
    if let Some(s) = num_steps(dir, Dir::Down, straight) {
        if pos.0 < max_i - s {
            steps.push(((pos.0 + s, pos.1), Dir::Down, s));
        }
    }
    if let Some(s) = num_steps(dir, Dir::Left, straight) {
        if pos.1 >= s {
            steps.push(((pos.0, pos.1 - s), Dir::Left, s));
        }
    }
    if let Some(s) = num_steps(dir, Dir::Right, straight) {
        if pos.1 < max_j - s {
            steps.push(((pos.0, pos.1 + s), Dir::Right, s));
        }
    }
    steps
}

#[aoc(day17, part2)]
fn solve_two(input: &str) -> Result<String> {
    use Dir::*;
    let grid = &parse_input(input);
    let max_i = grid.len();
    let max_j = grid[0].len();

    let mut lowest = vec![vec![None; max_j]; max_i];
    lowest[0][0] = Some(0);

    let mut search = VecDeque::new();
    search.push_back(Search {
        loss: 0,
        pos: (0, 0),
        dir: Up,
        straight: 10,
    });
    search.push_back(Search {
        loss: 0,
        pos: (0, 0),
        dir: Left,
        straight: 10,
    });

    while lowest[max_i - 1][max_j - 1].is_none() {
        // sort by loss
        search.make_contiguous().sort_by_key(|s| s.loss);

        // take all elements with lowest loss
        let mut low_loss = HashSet::new();
        let first = search.pop_front().ok_or(eyre!("empty search vec"))?;
        while let Some(f) = search.front() {
            if f.loss == first.loss {
                low_loss.insert(search.pop_front().ok_or(eyre!("empty search vec"))?);
            } else {
                break;
            }
        }
        low_loss.insert(first);

        // take next step of lowest loss
        let mut low_loss = low_loss
            .into_iter()
            .flat_map(|s| {
                let search = s.clone();
                next_steps2(max_i, max_j, &search.pos, &search.dir, search.straight)
                    .into_iter()
                    .map(move |n| {
                        let add_loss = (0..n.2)
                            .map(|i| match n.1 {
                                Up => grid[n.0 .0 + i][n.0 .1],
                                Down => grid[n.0 .0 - i][n.0 .1],
                                Left => grid[n.0 .0][n.0 .1 + i],
                                Right => grid[n.0 .0][n.0 .1 - i],
                            })
                            .sum::<u64>();
                        Search {
                            loss: search.loss + add_loss,
                            pos: n.0,
                            straight: if n.1 == search.dir {
                                search.straight + n.2 as u8
                            } else {
                                n.2 as u8
                            },
                            dir: n.1,
                        }
                    })
            })
            .collect::<Vec<Search>>();
        // sort next steps
        // if lowest not yet set; set it and add back into search
        // if lowest is set already; check not falling too far behind and add back into search
        low_loss.sort_by_key(|s| s.loss);
        low_loss.into_iter().for_each(|s| {
            if let Some(l) = lowest[s.pos.0][s.pos.1] {
                // 96 is worse case additional loss when going around instead of straight
                if s.loss <= l || s.loss - l <= 96 {
                    search.push_back(s);
                }
            } else {
                lowest[s.pos.0][s.pos.1] = Some(s.loss);
                search.push_back(s);
            }
        });
    }

    lowest[max_i - 1][max_j - 1]
        .ok_or(eyre!("no finish value"))
        .map(|v| v.to_string())
}
