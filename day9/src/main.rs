use eyre::{eyre, Result};
use utils::derive::aoc;

fn parse_input(input: &str) -> Result<Vec<Vec<i64>>> {
    input
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|v| Ok(v.parse::<i64>()?))
                .collect::<Result<Vec<i64>>>()
        })
        .collect()
}

#[aoc(day9, part1)]
fn solve_one(input: &str) -> Result<String> {
    let history = parse_input(input)?;
    Ok(history
        .into_iter()
        .map(|v| {
            let mut diffs = vec![v];
            loop {
                let l = diffs.last().ok_or(eyre!("no last"))?;
                if l.iter().all(|&i| i == 0) {
                    diffs.pop();
                    break;
                }
                diffs.push(l.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i64>>());
            }
            diffs.iter().rev().try_fold(0, |next, diff| {
                Ok(next + diff.last().ok_or(eyre!("no last"))?)
            })
        })
        .sum::<Result<i64>>()?
        .to_string())
}

#[aoc(day9, part2)]
fn solve_two(input: &str) -> Result<String> {
    let history = parse_input(input)?;
    Ok(history
        .into_iter()
        .map(|v| {
            let mut diffs = vec![v];
            loop {
                let l = diffs.last().ok_or(eyre!("no last"))?;
                if l.iter().all(|&i| i == 0) {
                    diffs.pop();
                    break;
                }
                diffs.push(l.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i64>>());
            }
            diffs.iter().rev().try_fold(0, |prev, diff| {
                Ok(diff.first().ok_or(eyre!("no first"))? - prev)
            })
        })
        .sum::<Result<i64>>()?
        .to_string())
}
