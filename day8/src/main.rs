use clap::Parser;
use eyre::{eyre, Result};
use std::collections::HashMap;

fn main() -> Result<()> {
    let args = utils::Args::parse();
    let input = args.get_input(8)?;

    if args.run_one() {
        println!("part one:\n{}", solve_one(&input)?);
    }
    if args.run_two() {
        println!("part two:\n{}", solve_two(&input)?);
    }

    Ok(())
}

#[derive(Clone, Debug)]
enum LeftRight {
    Left,
    Right,
}

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> Result<(Vec<LeftRight>, HashMap<&str, (&str, &str)>)> {
    let (lr, map) = input.split_once("\n\n").ok_or(eyre!("missing new lines"))?;
    Ok((
        lr.chars()
            .map(|c| match c {
                'L' => Ok(LeftRight::Left),
                'R' => Ok(LeftRight::Right),
                _ => eyre::bail!("unexpected char: {}", c),
            })
            .collect::<Result<Vec<LeftRight>>>()?,
        map.lines()
            .map(|l| {
                let (from, to) = l.split_once(" = ").ok_or(eyre!("missing ="))?;
                let to_lr = to
                    .strip_prefix('(')
                    .ok_or(eyre!("missing ("))?
                    .strip_suffix(')')
                    .ok_or(eyre!("missing )"))?
                    .split_once(", ")
                    .ok_or(eyre!("missing ,"))?;
                Ok((from, to_lr))
            })
            .collect::<Result<HashMap<&str, (&str, &str)>>>()?,
    ))
}

fn num_of_steps(lr: &[LeftRight], map: &HashMap<&str, (&str, &str)>, start: &str, end: impl Fn(&str) -> bool) -> Result<usize> {
    let mut cur: Result<&str> = Ok(start);
    let last = lr
        .iter()
        .cycle()
        .enumerate()
        .take_while(|(_, lr)| {
            if let Ok(c) = cur {
                cur = if let Some(to) = map.get(c) {
                    Ok(match lr {
                        LeftRight::Left => to.0,
                        LeftRight::Right => to.1,
                    })
                } else {
                    Err(eyre!("{} missing from map", c))
                };
            }

            if let Ok(c) = cur {
                !end(c)
            } else {
                false
            }
        })
        .last()
        .ok_or(eyre!("no last"))?;

    // check cur is still ok
    cur?;

    // plus 2 to last step enumarated value
    // 1 because enumerate() starts at 0
    // another 1 because take_while(...).last() does not inculde the last step
    Ok(last.0 + 2)
}

fn solve_one(input: &str) -> Result<String> {
    let (lr, map) = parse_input(input)?;
    Ok(num_of_steps(&lr, &map, "AAA", |e| e == "ZZZ")?.to_string())
}

fn solve_two(input: &str) -> Result<String> {
    let (lr, map) = parse_input(input)?;
    let steps = map
        .clone()
        .into_keys()
        .filter(|k| k.ends_with('A'))
        .map(|s| num_of_steps(&lr, &map, s, |e| e.ends_with('Z')))
        .collect::<Result<Vec<usize>>>()?;
    Ok(steps.into_iter().fold(1, num_integer::lcm).to_string())
}
