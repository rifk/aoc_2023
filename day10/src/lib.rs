use eyre::{eyre, Result};

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
}

fn find_start(grid: &[Vec<char>]) -> Result<(usize, usize)> {
    grid.iter()
        .enumerate()
        .find_map(|(i, r)| {
            r.iter()
                .enumerate()
                .find_map(|(j, &c)| if c == 'S' { Some((i, j)) } else { None })
        })
        .ok_or(eyre!("no S found"))
}

fn find_connecting(grid: &[Vec<char>], cur: (usize, usize)) -> Vec<(usize, usize)> {
    vec![
        if cur.0 > 1 && ['7', '|', 'F'].contains(&grid[cur.0 - 1][cur.1]) {
            Some((cur.0 - 1, cur.1))
        } else {
            None
        },
        if cur.0 < grid.len() - 1 && ['J', '|', 'L'].contains(&grid[cur.0 + 1][cur.1]) {
            Some((cur.0 + 1, cur.1))
        } else {
            None
        },
        if cur.1 > 1 && ['L', '-', 'F'].contains(&grid[cur.0][cur.1 - 1]) {
            Some((cur.0, cur.1 - 1))
        } else {
            None
        },
        if cur.1 < grid[cur.0].len() - 1 && ['J', '-', '7'].contains(&grid[cur.0][cur.1 + 1]) {
            Some((cur.0, cur.1 + 1))
        } else {
            None
        },
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn find_next(
    grid: &[Vec<char>],
    last: (usize, usize),
    cur: (usize, usize),
) -> Result<(usize, usize)> {
    Ok(match grid[cur.0][cur.1] {
        '|' => {
            if last.0 == cur.0 - 1 {
                (cur.0 + 1, cur.1)
            } else {
                (cur.0 - 1, cur.1)
            }
        }
        '-' => {
            if last.1 == cur.1 - 1 {
                (cur.0, cur.1 + 1)
            } else {
                (cur.0, cur.1 - 1)
            }
        }
        'L' => {
            if last.1 == cur.1 + 1 {
                (cur.0 - 1, cur.1)
            } else {
                (cur.0, cur.1 + 1)
            }
        }
        'J' => {
            if last.1 == cur.1 - 1 {
                (cur.0 - 1, cur.1)
            } else {
                (cur.0, cur.1 - 1)
            }
        }
        '7' => {
            if last.1 == cur.1 - 1 {
                (cur.0 + 1, cur.1)
            } else {
                (cur.0, cur.1 - 1)
            }
        }
        'F' => {
            if last.1 == cur.1 + 1 {
                (cur.0 + 1, cur.1)
            } else {
                (cur.0, cur.1 + 1)
            }
        }
        other => eyre::bail!("unexpected char at cur {}", other),
    })
}

pub fn solve_one(input: &str) -> Result<String> {
    let grid = parse_input(input);

    let s = find_start(&grid)?;

    let (mut cur1, mut cur2) = {
        let conns = find_connecting(&grid, s);
        if conns.len() != 2 {
            eyre::bail!("expected 2 pipes connect to S - {:?}", conns);
        }
        (conns[0], conns[1])
    };

    let mut steps = 1;
    let mut last1 = s;
    let mut last2 = s;

    while cur1 != cur2 {
        let next1 = find_next(&grid, last1, cur1)?;
        last1 = cur1;
        cur1 = next1;

        let next2 = find_next(&grid, last2, cur2)?;
        last2 = cur2;
        cur2 = next2;

        steps += 1;
    }

    Ok(steps.to_string())
}

// Edit grid to mark left(l), right(r), and path(p) of the current position. Only mark l/r if its
// not part of path.
fn mark_l_r_p(
    grid: &mut [Vec<char>],
    path: &[(usize, usize)],
    last: (usize, usize),
    cur: (usize, usize),
) -> Result<()> {
    let (l, r) = match grid[cur.0][cur.1] {
        '|' => {
            //1|2
            let s1 = vec![(cur.0, cur.1 - 1)];
            let s2 = vec![(cur.0, cur.1 + 1)];
            if last.0 == cur.0 - 1 {
                (s2, s1)
            } else {
                (s1, s2)
            }
        }
        '-' => {
            //1
            //-
            //2
            let s1 = vec![(cur.0 - 1, cur.1)];
            let s2 = vec![(cur.0 + 1, cur.1)];
            if last.1 == cur.1 - 1 {
                (s1, s2)
            } else {
                (s2, s1)
            }
        }
        'L' => {
            //.|2
            //1L-
            //11.
            let s1 = vec![
                (cur.0, cur.1 - 1),
                (cur.0 + 1, cur.1 - 1),
                (cur.0 + 1, cur.1),
            ];
            let s2 = vec![(cur.0 - 1, cur.1 + 1)];
            if last.0 == cur.0 - 1 {
                (s2, s1)
            } else {
                (s1, s2)
            }
        }
        'J' => {
            //1|.
            //-J2
            //.22
            let s1 = vec![(cur.0 - 1, cur.1 - 1)];
            let s2 = vec![
                (cur.0, cur.1 + 1),
                (cur.0 + 1, cur.1 + 1),
                (cur.0 + 1, cur.1),
            ];
            if last.0 == cur.0 - 1 {
                (s2, s1)
            } else {
                (s1, s2)
            }
        }
        '7' => {
            //.11
            //-71
            //2|.
            let s1 = vec![
                (cur.0 - 1, cur.1),
                (cur.0 - 1, cur.1 + 1),
                (cur.0, cur.1 + 1),
            ];
            let s2 = vec![(cur.0 + 1, cur.1 - 1)];
            if last.1 == cur.1 - 1 {
                (s1, s2)
            } else {
                (s2, s1)
            }
        }
        'F' => {
            //11.
            //1F-
            //.|2
            let s1 = vec![
                (cur.0, cur.1 - 1),
                (cur.0 - 1, cur.1 - 1),
                (cur.0 - 1, cur.1),
            ];
            let s2 = vec![(cur.0, cur.1)];
            if last.0 == cur.0 + 1 {
                (s1, s2)
            } else {
                (s2, s1)
            }
        }
        'S' => (vec![], vec![]),
        other => eyre::bail!("unexpected char at cur {}", other),
    };

    l.into_iter().for_each(|l| {
        if !path.contains(&l) {
            grid[l.0][l.1] = 'l'
        }
    });
    r.into_iter().for_each(|r| {
        if !path.contains(&r) {
            grid[r.0][r.1] = 'r'
        }
    });

    grid[cur.0][cur.1] = 'p';
    Ok(())
}

pub fn solve_two(input: &str) -> Result<String> {
    let mut grid = parse_input(input);

    // add boarder, so we dont need to worry about going out of bounds
    grid.iter_mut().for_each(|r| {
        r.push('.');
        r.push('.');
        r.rotate_right(1);
    });
    grid.push(vec!['.'; grid[0].len()]);
    grid.push(vec!['.'; grid[0].len()]);
    grid.rotate_right(1);

    let s = find_start(&grid)?;

    // traverse loop and add to path vec
    let mut last = s;
    let mut cur = find_connecting(&grid, last)[0];
    let mut path = vec![last, cur];
    while cur != s {
        let next = find_next(&grid, last, cur)?;
        path.push(next);
        last = cur;
        cur = next;
    }

    // mark path(p) and both sides of path (l/r)
    path.windows(2).try_for_each(|w| {
        let f = w[0];
        let t = w[1];
        mark_l_r_p(&mut grid, &path, f, t)
    })?;

    let max_i = grid.len() - 1;
    let max_j = grid[0].len() - 1;

    // spread l/r to fill areas
    (1..max_i).for_each(|i| {
        (1..max_j).for_each(|j| match grid[i][j] {
            'r' | 'l' => {
                if grid[i + 1][j] != 'p' {
                    grid[i + 1][j] = grid[i][j];
                }
                if grid[i][j + 1] != 'p' {
                    grid[i][j + 1] = grid[i][j];
                }
            }
            _ => {}
        })
    });

    // find if inside is l or r, l/r on boarder will be outside
    let in_char = vec![
        (0..=max_i)
            .map(|i| (i, max_j))
            .collect::<Vec<(usize, usize)>>()
            .into_iter(),
        (0..=max_j)
            .map(|j| (max_i, j))
            .collect::<Vec<(usize, usize)>>()
            .into_iter(),
    ]
    .into_iter()
    .flatten()
    .find_map(|(i, j)| match grid[i][j] {
        'r' => Some('l'),
        'l' => Some('r'),
        _ => None,
    })
    .ok_or(eyre!("failed to find inside char, no l/r on boarder"))?;

    Ok(grid
        .into_iter()
        .flat_map(|r| r.into_iter())
        .filter(|&c| c == in_char)
        .count()
        .to_string())
}
