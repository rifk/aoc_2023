use eyre::{eyre, Result};
use utils::derive::aoc;
use std::collections::HashMap;
use std::collections::hash_map::{DefaultHasher, Entry};
use std::hash::{Hasher, Hash};

#[derive(Hash, PartialEq)]
enum Rock {
    Square,
    Round,
}

fn parse_input(input: &str) -> Result<Vec<Vec<Option<Rock>>>> {
    input.lines().map(|l| l.chars().map(|c| match c {
        '#' => Ok(Some(Rock::Square)),
        'O' => Ok(Some(Rock::Round)),
        '.' => Ok(None),
        _ => Err(eyre!("unexpected char {}", c))
    }).collect::<Result<Vec<Option<Rock>>>>()).collect()
}

enum Dir {
    North,
    South,
    East,
    West,
}
fn tilt(platform: &mut [Vec<Option<Rock>>], dir: Dir) {
    let (mut next_rock, mut iter) = match dir {
        Dir::North => (
            (0,0),
            (0..platform[0].len()).flat_map(|j| (0..platform.len()).map(move |i| (i,j))),
        ),
        _ => todo!(),
    };
}
fn spin(platform: &mut [Vec<Option<Rock>>]) {
        tilt(platform, Dir::North);
        tilt(platform, Dir::West);
        tilt(platform, Dir::South);
        tilt(platform, Dir::East);
}

#[aoc(day14, part1)]
fn solve_one(input: &str) -> Result<String> {
    let platform = parse_input(input)?;
    let max_i = platform.len();
    Ok(
        (0..platform[0].len()).map(|j| {
            let mut next_rock = 0;
            (0..max_i).filter_map(|i| {
                match platform[i][j] {
                    None => None,
                    Some(Rock::Square) => {
                        next_rock = i + 1;
                        None
                    },
                    Some(Rock::Round) => {
                        next_rock += 1;
                        Some((max_i + 1 - next_rock) as u64)
                    },
                }
            }).sum::<u64>()
        }).sum::<u64>()
        .to_string()
    )
}


#[aoc(day14, part2)]
fn solve_two(input: &str) -> Result<String> {
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
            dbg!(h.finish())
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
    Ok(platform.into_iter().enumerate().map(|(i, r)| {
        ((max_i - i) * r.into_iter().filter(|p| *p == Some(Rock::Round)).count()) as u64
    })
       .sum::<u64>()
       .to_string())
}
