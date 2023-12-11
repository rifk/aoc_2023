use eyre::{eyre, Result};
use utils::derive::aoc;

fn parse_line(line: &str) -> Result<(Vec<Option<bool>>, Vec<u8>)> {
    let (springs, damage_count) = line
        .split_once(' ')
        .ok_or(eyre!("missing space in line {}", line))?;
    Ok((
        springs
            .chars()
            .map(|c| match c {
                '.' => Ok(Some(false)),
                '#' => Ok(Some(true)),
                '?' => Ok(None),
                _ => Err(eyre!("unexpected char {}", c)),
            })
            .collect::<Result<Vec<Option<bool>>>>()?,
        damage_count
            .split(',')
            .map(|v| Ok(v.parse::<u8>()?))
            .collect::<Result<Vec<u8>>>()?,
    ))
}

fn num_arrangements(springs: &[Option<bool>], cons: &[u8], cur: u8, s: &str) -> u64 {
    let sum = cons.iter().sum::<u8>();
    let gaps = if cons.is_empty() {
        0
    } else {
        cons.len() as u8 - 1
    };
    if springs.len() < (sum + gaps - cur).into() {
        // not enough springs left
        return 0;
    }

    if cur != 0 {
        // have consecutive count, check if next sping fits requirements
        if cons[0] == cur {
            // end of consecutive count, require not damaged
            if springs.is_empty() {
                // no more springs, check no more cons
                if cons.len() == 1 {
                    // no more needed, finish here
                    1
                } else {
                    // more springs needed
                    0
                }
            } else if !springs[0].unwrap_or(false) {
                // next spring meets required not damaged
                num_arrangements(&springs[1..], &cons[1..], 0, &(s.to_owned() + "."))
            } else {
                // next spring fails requirement
                0
            }
        } else {
            // midding of consecutive count, require damaged
            if springs.is_empty() {
                0
            } else if springs[0].unwrap_or(true) {
                // next spring meets requried damanged
                num_arrangements(&springs[1..], cons, cur + 1, &(s.to_owned() + "#"))
            } else {
                // next spring fails requirement
                0
            }
        }
    } else {
        // no consecutive count
        match (springs.is_empty(), cons.is_empty()) {
            (true, true) => {
                // end springs and no more required
                1
            }
            (true, false) => {
                // end springs and but more required
                0
            }
            (false, true) => {
                // still have springs, but must be not damanged
                if !springs[0].unwrap_or(false) {
                    num_arrangements(&springs[1..], cons, 0, &(s.to_owned() + "."))
                } else {
                    0
                }
            }
            (false, false) => match springs[0] {
                Some(true) => num_arrangements(&springs[1..], cons, 1, &(s.to_owned() + "#")),
                Some(false) => num_arrangements(&springs[1..], cons, 0, &(s.to_owned() + ".")),
                None => {
                    num_arrangements(&springs[1..], cons, 0, &(s.to_owned() + "."))
                        + num_arrangements(&springs[1..], cons, 1, &(s.to_owned() + "#"))
                }
            },
        }
    }
}

#[aoc(day12, part1)]
fn solve_one(input: &str) -> Result<String> {
    Ok(input
        .lines()
        .map(|l| parse_line(l).map(|(springs, cons)| num_arrangements(&springs, &cons, 0, "")))
        .sum::<Result<u64>>()?
        .to_string())
}

#[aoc(day12, part2)]
fn solve_two(input: &str) -> Result<String> {
    Ok(input
        .lines()
        .map(|l| {
            parse_line(l).map(|(springs, cons)| {
                let mut s = springs.clone();
                for _ in 0..4 {
                    s.push(None);
                    s.append(&mut s.clone());
                }
                num_arrangements(&s, &cons.repeat(5), 0, "")
            })
        })
        .sum::<Result<u64>>()?
        .to_string())
}
