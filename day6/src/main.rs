use clap::Parser;
use eyre::{eyre, Result};

fn main() -> Result<()> {
    let args = utils::Args::parse();
    let input = args.get_input(6)?;

    let time_dist = parse_input(&input)?;

    if args.run_one() {
        println!("part one:\n{}", solve_one(&time_dist)?);
    }
    if args.run_two() {
        println!("part two:\n{}", solve_two(&time_dist)?);
    }

    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<(i64, i64)>> {
    let (t, d) = input.split_once('\n').ok_or(eyre!("missing new line"))?;
    let t = t
        .strip_prefix("Time: ")
        .ok_or(eyre!("missing time prefix"))?
        .trim()
        .split(char::is_whitespace)
        .filter(|s| !s.is_empty())
        .map(|v| Ok(v.parse::<i64>()?))
        .collect::<Result<Vec<i64>>>()?;
    let d = d
        .strip_prefix("Distance: ")
        .ok_or(eyre!("missing distance prefix"))?
        .trim()
        .split(char::is_whitespace)
        .filter(|s| !s.is_empty())
        .map(|v| Ok(v.parse::<i64>()?))
        .collect::<Result<Vec<i64>>>()?;
    Ok(t.into_iter().zip(d).collect::<Vec<(i64, i64)>>())
}

fn solve_one(time_dist: &[(i64, i64)]) -> Result<String> {
    Ok(time_dist
        .iter()
        .map(|(t, d)| {
            let mut c = 1;
            let mut m = t - 1;
            while d >= &(c * m) {
                c += 1;
                m -= 1;
            }
            t - (2 * c) + 1
        })
        .product::<i64>()
        .to_string())
}

fn solve_two(time_dist: &[(i64, i64)]) -> Result<String> {
    let (t, d) = time_dist.iter().fold((0, 0), |mut out, part| {
        let mut t = part.0;
        while t > 0 {
            out.0 *= 10;
            t /= 10;
        }
        out.0 += part.0;
        let mut d = part.1;
        while d > 0 {
            out.1 *= 10;
            d /= 10;
        }
        out.1 += part.1;
        out
    });
    solve_one(&[(t, d)])
}
