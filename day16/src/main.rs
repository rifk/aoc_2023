use eyre::{eyre, Result};
use utils::derive::aoc;

#[derive(Clone, Debug)]
enum Contraption {
    HSplit,
    VSplit,
    UpLeftMirror,
    UpRightMirror,
}
#[derive(Clone, Debug, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn parse_input(input: &str) -> Result<Vec<Vec<Option<Contraption>>>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Ok(None),
                    '-' => Ok(Some(Contraption::HSplit)),
                    '|' => Ok(Some(Contraption::VSplit)),
                    '/' => Ok(Some(Contraption::UpLeftMirror)),
                    '\\' => Ok(Some(Contraption::UpRightMirror)),
                    _ => Err(eyre!("unexpected char {}", c)),
                })
                .collect::<Result<Vec<Option<Contraption>>>>()
        })
        .collect()
}

fn light(
    grid: &[Vec<Option<Contraption>>],
    entered: &mut [Vec<Vec<Dir>>],
    pos: (usize, usize),
    dir: Dir,
) {
    use Contraption::*;
    use Dir::*;
    if entered[pos.0][pos.1].contains(&dir) {
        return;
    }
    entered[pos.0][pos.1].push(dir.clone());
    let next_dirs = match &grid[pos.0][pos.1] {
        Some(HSplit) => match &dir {
            Up | Down => vec![Left, Right],
            Left | Right => vec![dir],
        },
        Some(VSplit) => match &dir {
            Up | Down => vec![dir],
            Left | Right => vec![Up, Down],
        },
        Some(UpLeftMirror) => match &dir {
            Up => vec![Right],
            Down => vec![Left],
            Left => vec![Down],
            Right => vec![Up],
        },
        Some(UpRightMirror) => match &dir {
            Up => vec![Left],
            Down => vec![Right],
            Left => vec![Up],
            Right => vec![Down],
        },
        None => vec![dir],
    };
    next_dirs.into_iter().for_each(|d| {
        if let Some(next) = next_pos(grid, pos, &d) {
            light(grid, entered, next, d);
        }
    });
}
fn next_pos(
    grid: &[Vec<Option<Contraption>>],
    pos: (usize, usize),
    dir: &Dir,
) -> Option<(usize, usize)> {
    use Dir::*;
    match dir {
        Up => {
            if pos.0 > 0 {
                Some((pos.0 - 1, pos.1))
            } else {
                None
            }
        }
        Down => {
            if pos.0 < grid.len() - 1 {
                Some((pos.0 + 1, pos.1))
            } else {
                None
            }
        }
        Left => {
            if pos.1 > 0 {
                Some((pos.0, pos.1 - 1))
            } else {
                None
            }
        }
        Right => {
            if pos.1 < grid[0].len() - 1 {
                Some((pos.0, pos.1 + 1))
            } else {
                None
            }
        }
    }
}

#[aoc(day16, part1)]
fn solve_one(input: &str) -> Result<String> {
    let grid = parse_input(input)?;
    let mut entered: Vec<Vec<Vec<Dir>>> = vec![vec![vec![]; grid[0].len()]; grid.len()];
    light(&grid, &mut entered, (0, 0), Dir::Right);
    Ok(entered
        .into_iter()
        .flat_map(|v| v.into_iter())
        .filter(|dirs| !dirs.is_empty())
        .count()
        .to_string())
}

#[aoc(day16, part2)]
fn solve_two(input: &str) -> Result<String> {
    let grid = parse_input(input)?;
    Ok(vec![
        (0..grid.len())
            .flat_map(|i| {
                vec![((i, 0), Dir::Right), ((i, grid[0].len() - 1), Dir::Left)].into_iter()
            })
            .collect::<Vec<((usize, usize), Dir)>>()
            .into_iter(),
        (0..grid[0].len())
            .flat_map(|j| vec![((0, j), Dir::Down), ((grid.len() - 1, j), Dir::Up)].into_iter())
            .collect::<Vec<((usize, usize), Dir)>>()
            .into_iter(),
    ]
    .into_iter()
    .flatten()
    .map(|(pos, dir)| {
        let mut entered: Vec<Vec<Vec<Dir>>> = vec![vec![vec![]; grid[0].len()]; grid.len()];
        light(&grid, &mut entered, pos, dir);
        entered
            .into_iter()
            .flat_map(|v| v.into_iter())
            .filter(|dirs| !dirs.is_empty())
            .count()
    })
    .max()
    .ok_or(eyre!("no max found"))?
    .to_string())
}
