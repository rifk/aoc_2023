use eyre::{eyre, Result};
use std::ops::Range;

#[derive(Clone, Debug, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn parse_input(input: &str) -> Result<Vec<(Dir, i64, &str)>> {
    input
        .lines()
        .map(|l| {
            let (d, l) = l
                .split_once(' ')
                .ok_or(eyre!("missing space in line {}", l))?;
            let d = match d {
                "U" => Dir::Up,
                "D" => Dir::Down,
                "L" => Dir::Left,
                "R" => Dir::Right,
                _ => eyre::bail!("unknown direction {}", d),
            };
            let (num, col) = l
                .split_once(' ')
                .ok_or(eyre!("missing second space in line {}", l))?;
            let num = num.parse::<i64>()?;
            let col = col
                .strip_prefix("(#")
                .ok_or(eyre!("color missing prefix (#"))?
                .strip_suffix(')')
                .ok_or(eyre!("color missing suffix )"))?;
            Ok((d, num, col))
        })
        .collect()
}

fn get_corner(from: &Dir, to: &Dir) -> Result<char> {
    use Dir::*;
    Ok(match (from, to) {
        (Up, Left) | (Right, Down) => '7',
        (Up, Right) | (Left, Down) => 'F',
        (Down, Left) | (Right, Up) => 'J',
        (Down, Right) | (Left, Up) => 'L',
        _ => eyre::bail!("unexpected subsequent directions - {:?} {:?}", from, to),
    })
}

fn count_fill(grid: &[Vec<char>]) -> Result<u64> {
    let mut count = 0;
    grid.iter().try_for_each(|l| {
        let mut inside = false;
        let mut top = false;
        l.iter().try_for_each(|c| {
            match c {
                '|' => {
                    count += 1;
                    inside = !inside;
                }
                'L' => {
                    count += 1;
                    top = false;
                }
                'F' => {
                    count += 1;
                    top = true;
                }
                'J' => {
                    count += 1;
                    if top {
                        inside = !inside;
                    }
                }
                '7' => {
                    count += 1;
                    if !top {
                        inside = !inside;
                    }
                }
                '-' => {
                    count += 1;
                }
                ' ' => {
                    if inside {
                        count += 1;
                    }
                }
                _ => eyre::bail!("unexpected char {}", c),
            }
            Ok::<(), eyre::Error>(())
        })
    })?;
    Ok(count)
}

pub fn solve_one(input: &str) -> Result<String> {
    let mut plan = parse_input(input)?;

    let (min_i, max_i, min_j, max_j) = {
        let mut cur = (0, 0);
        let mut min_max = (0, 0, 0, 0);
        plan.iter().for_each(|(d, n, _)| match d {
            Dir::Up => {
                cur.0 -= n;
                min_max.0 = min_max.0.min(cur.0);
            }
            Dir::Down => {
                cur.0 += n;
                min_max.1 = min_max.1.max(cur.0);
            }
            Dir::Left => {
                cur.1 -= n;
                min_max.2 = min_max.2.min(cur.1);
            }
            Dir::Right => {
                cur.1 += n;
                min_max.3 = min_max.3.max(cur.1);
            }
        });
        (
            min_max.0.unsigned_abs() as usize,
            min_max.1 as usize,
            min_max.2.unsigned_abs() as usize,
            min_max.3 as usize,
        )
    };

    let mut grid = vec![vec![' '; 1 + max_j + min_j]; 1 + max_i + min_i];

    plan.push(plan[0].clone());
    let mut cur = (min_i, min_j);
    plan.windows(2).try_for_each(|w| {
        let (d, n, _) = &w[0];
        (0..*n as usize).for_each(|_| match d {
            Dir::Up => {
                cur.0 -= 1;
                grid[cur.0][cur.1] = '|';
            }
            Dir::Down => {
                cur.0 += 1;
                grid[cur.0][cur.1] = '|';
            }
            Dir::Left => {
                cur.1 -= 1;
                grid[cur.0][cur.1] = '-';
            }
            Dir::Right => {
                cur.1 += 1;
                grid[cur.0][cur.1] = '-';
            }
        });
        grid[cur.0][cur.1] = get_corner(d, &w[1].0)?;
        Ok::<(), eyre::Error>(())
    })?;

    Ok(count_fill(&grid)?.to_string())
}

#[derive(Clone, Debug)]
enum Edge {
    Vert(usize),
    Cross(Range<usize>),
    Skirt(Range<usize>),
}

pub fn solve_two(input: &str) -> Result<String> {
    let mut plan = parse_input(input)?;

    plan = plan
        .into_iter()
        .map(|(_, _, enc)| {
            let num = i64::from_str_radix(&enc[0..5], 16)?;
            let dir = match &enc[5..] {
                "0" => Dir::Right,
                "1" => Dir::Down,
                "2" => Dir::Left,
                "3" => Dir::Up,
                _ => eyre::bail!("unexpected last char in {}", enc),
            };
            Ok((dir, num, enc))
        })
        .collect::<Result<Vec<(Dir, i64, &str)>>>()?;

    let (min_i, max_i, min_j, _) = {
        let mut cur = (0, 0);
        let mut min_max = (0, 0, 0, 0);
        plan.iter().for_each(|(d, n, _)| match d {
            Dir::Up => {
                cur.0 -= n;
                min_max.0 = min_max.0.min(cur.0);
            }
            Dir::Down => {
                cur.0 += n;
                min_max.1 = min_max.1.max(cur.0);
            }
            Dir::Left => {
                cur.1 -= n;
                min_max.2 = min_max.2.min(cur.1);
            }
            Dir::Right => {
                cur.1 += n;
                min_max.3 = min_max.3.max(cur.1);
            }
        });
        (
            min_max.0.unsigned_abs() as usize,
            min_max.1 as usize,
            min_max.2.unsigned_abs() as usize,
            min_max.3 as usize,
        )
    };

    let mut grid: Vec<Vec<Edge>> = vec![vec![]; 1 + max_i + min_i];

    let mut cur = (min_i, min_j);
    plan.reverse();
    plan.push(plan[0].clone());
    plan.reverse();
    plan.push(plan[1].clone());

    plan.windows(3).for_each(|w| {
        let (d, n, _) = &w[1];
        let n = *n as usize;
        match d {
            Dir::Up => {
                (1..n).for_each(|i| {
                    grid[cur.0 - i].push(Edge::Vert(cur.1));
                });
                cur.0 -= n;
            }
            Dir::Down => {
                (1..n).for_each(|i| {
                    grid[cur.0 + i].push(Edge::Vert(cur.1));
                });
                cur.0 += n;
            }
            Dir::Left => {
                let r = (cur.1 - n)..(cur.1 + 1);
                grid[cur.0].push(if w[0].0 == w[2].0 {
                    Edge::Cross(r)
                } else {
                    Edge::Skirt(r)
                });
                cur.1 -= n;
            }
            Dir::Right => {
                let r = cur.1..(cur.1 + n + 1);
                grid[cur.0].push(if w[0].0 == w[2].0 {
                    Edge::Cross(r)
                } else {
                    Edge::Skirt(r)
                });
                cur.1 += n;
            }
        }
    });

    Ok(grid
        .into_iter()
        .map(|mut l| {
            l.sort_by_key(|e| match e {
                Edge::Vert(v) => *v,
                Edge::Cross(r) => r.start,
                Edge::Skirt(r) => r.start,
            });
            let mut count = 0_u64;
            let mut j = 0_usize;
            let mut inside = false;
            l.iter().for_each(|e| match e {
                Edge::Vert(v) => {
                    if inside {
                        count += (*v - j) as u64;
                    }
                    count += 1;
                    inside = !inside;
                    j = *v + 1;
                }
                Edge::Cross(r) => {
                    if inside {
                        count += (r.start - j) as u64;
                    }
                    count += (r.end - r.start) as u64;
                    inside = !inside;
                    j = r.end;
                }
                Edge::Skirt(r) => {
                    if inside {
                        count += (r.start - j) as u64;
                    }
                    count += (r.end - r.start) as u64;
                    j = r.end;
                }
            });
            count
        })
        .sum::<u64>()
        .to_string())
}
