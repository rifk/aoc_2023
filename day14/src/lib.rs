use eyre::{eyre, Result};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Hash, PartialEq)]
enum Rock {
    Square,
    Round,
}

fn parse_input(input: &str) -> Result<Vec<Vec<Option<Rock>>>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => Ok(Some(Rock::Square)),
                    'O' => Ok(Some(Rock::Round)),
                    '.' => Ok(None),
                    _ => Err(eyre!("unexpected char {}", c)),
                })
                .collect::<Result<Vec<Option<Rock>>>>()
        })
        .collect()
}

enum Dir {
    North,
    South,
    East,
    West,
}
fn tilt(platform: &mut [Vec<Option<Rock>>], dir: Dir) {
    let max_i = platform.len();
    let max_j = platform[0].len();
    fn get_next(dir: &Dir, cur: (usize, usize)) -> (usize, usize) {
        match dir {
            Dir::North => (cur.0 + 1, cur.1),
            Dir::South => (if cur.0 > 0 { cur.0 - 1 } else { 0 }, cur.1),
            Dir::East => (cur.0, if cur.1 > 0 { cur.1 - 1 } else { 0 }),
            Dir::West => (cur.0, cur.1 + 1),
        }
    }
    match dir {
        Dir::North | Dir::South => 0..max_j,
        Dir::East | Dir::West => 0..max_i,
    }
    .for_each(|f| {
        let mut next = match dir {
            Dir::North => (0, f),
            Dir::South => (max_j - 1, f),
            Dir::East => (f, max_i - 1),
            Dir::West => (f, 0),
        };
        match dir {
            Dir::North => (0..max_i).map(|i| (i, f)).collect::<Vec<(usize, usize)>>(),
            Dir::South => (0..max_i)
                .rev()
                .map(|i| (i, f))
                .collect::<Vec<(usize, usize)>>(),
            Dir::East => (0..max_j)
                .rev()
                .map(|j| (f, j))
                .collect::<Vec<(usize, usize)>>(),
            Dir::West => (0..max_j).map(|j| (f, j)).collect::<Vec<(usize, usize)>>(),
        }
        .into_iter()
        .for_each(|(i, j)| match platform[i][j] {
            None => {}
            Some(Rock::Square) => {
                next = get_next(&dir, (i, j));
            }
            Some(Rock::Round) => {
                platform[i][j] = None;
                platform[next.0][next.1] = Some(Rock::Round);
                next = get_next(&dir, next);
            }
        });
    });
}
fn spin(platform: &mut [Vec<Option<Rock>>]) {
    tilt(platform, Dir::North);
    tilt(platform, Dir::West);
    tilt(platform, Dir::South);
    tilt(platform, Dir::East);
}

pub fn solve_one(input: &str) -> Result<String> {
    let platform = parse_input(input)?;
    let max_i = platform.len();
    Ok((0..platform[0].len())
        .map(|j| {
            let mut next_rock = 0;
            (0..max_i)
                .filter_map(|i| match platform[i][j] {
                    None => None,
                    Some(Rock::Square) => {
                        next_rock = i + 1;
                        None
                    }
                    Some(Rock::Round) => {
                        next_rock += 1;
                        Some((max_i + 1 - next_rock) as u64)
                    }
                })
                .sum::<u64>()
        })
        .sum::<u64>()
        .to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let mut platform = parse_input(input)?;
    let mut seen_map = HashMap::new();
    let mut seen_i = None;
    let mut i = 0;
    while i < 1_000_000_000 {
        spin(&mut platform);
        i += 1;

        let hash = {
            let mut h = DefaultHasher::new();
            platform.hash(&mut h);
            h.finish()
        };
        if let Some(s) = seen_map.get(&hash) {
            seen_i = Some(*s);
            break;
        }
        seen_map.insert(hash, i);
    }

    if i < 1_000_000_000 {
        let period = i - seen_i.ok_or(eyre!("expected seen_i"))?;

        let rem = (1_000_000_000 - i) % period;
        (0..rem).for_each(|_| spin(&mut platform));
    }

    let max_i = platform.len();
    Ok(platform
        .into_iter()
        .enumerate()
        .map(|(i, r)| {
            ((max_i - i) * r.into_iter().filter(|p| *p == Some(Rock::Round)).count()) as u64
        })
        .sum::<u64>()
        .to_string())
}
