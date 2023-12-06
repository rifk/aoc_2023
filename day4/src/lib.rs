use eyre::{eyre, Result};
use std::collections::HashSet;

fn parse_input(input: &str) -> Result<Vec<(HashSet<i64>, HashSet<i64>)>> {
    input
        .lines()
        .map(|l| {
            let (_, card) = l.split_once(": ").ok_or(eyre!("missing ': '"))?;
            let (win, have) = card.split_once(" | ").ok_or(eyre!("missing ' | '"))?;
            Ok((
                win.split(' ')
                    .filter(|v| !v.is_empty())
                    .map(|num| Ok(num.parse::<i64>()?))
                    .collect::<Result<HashSet<i64>>>()?,
                have.split(' ')
                    .filter(|v| !v.is_empty())
                    .map(|num| Ok(num.parse::<i64>()?))
                    .collect::<Result<HashSet<i64>>>()?,
            ))
        })
        .collect::<Result<Vec<(HashSet<i64>, HashSet<i64>)>>>()
}

pub fn solve_one(input: &str) -> Result<String> {
    Ok(parse_input(input)?
        .iter()
        .map(|c| {
            let win_count = c.0.intersection(&c.1).count() as u32;
            if win_count == 0 {
                0
            } else {
                2_i32.pow(win_count - 1)
            }
        })
        .sum::<i32>()
        .to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let cards = parse_input(input)?;
    let wins = cards
        .iter()
        .map(|c| c.0.intersection(&c.1).count())
        .collect::<Vec<usize>>();
    let mut count = vec![1; cards.len()];
    for (i, w) in wins.iter().enumerate() {
        for j in i + 1..count.len().min(i + 1 + w) {
            count[j] += count[i];
        }
    }
    Ok(count.iter().sum::<i64>().to_string())
}
