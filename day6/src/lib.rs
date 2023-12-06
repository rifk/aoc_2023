use eyre::{eyre, Result};

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

pub fn solve_one(input: &str) -> Result<String> {
    solve(&parse_input(input)?)
}

fn solve(time_dist: &[(i64, i64)]) -> Result<String> {
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

pub fn solve_two(input: &str) -> Result<String> {
    let (t, d) = parse_input(input)?.iter().fold((0, 0), |mut out, part| {
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
    solve(&[(t, d)])
}
