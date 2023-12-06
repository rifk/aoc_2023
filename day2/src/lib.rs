use eyre::{bail, eyre, Result};

#[derive(Clone, Debug, Default)]
struct Rgb {
    r: i64,
    g: i64,
    b: i64,
}

fn parse_input(input: &str) -> Result<Vec<Vec<Rgb>>> {
    input
        .lines()
        .map(|l| {
            let (_, l) = l.split_once(": ").ok_or(eyre!("missing ': '"))?;
            l.split("; ")
                .map(|r| {
                    let mut rgb = Rgb::default();
                    for p in r.split(", ") {
                        let (num, colour) = p.split_once(' ').ok_or(eyre!("missing ' '"))?;
                        let num = num.parse::<i64>()?;
                        match colour {
                            "red" => rgb.r = num,
                            "green" => rgb.g = num,
                            "blue" => rgb.b = num,
                            _ => bail!("unknown colour {colour}"),
                        }
                    }
                    Ok(rgb)
                })
                .collect::<Result<Vec<Rgb>>>()
        })
        .collect::<Result<Vec<Vec<Rgb>>>>()
}

pub fn solve_one(input: &str) -> Result<String> {
    Ok(parse_input(input)?
        .iter()
        .enumerate()
        .filter(|(_, game)| {
            !game
                .iter()
                .any(|rgb| rgb.r > 12 || rgb.g > 13 || rgb.b > 14)
        })
        .map(|(i, _)| i + 1)
        .sum::<usize>()
        .to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    Ok(parse_input(input)?
        .iter()
        .map(|game| {
            let mut min = Rgb::default();
            for round in game {
                min.r = min.r.max(round.r);
                min.g = min.g.max(round.g);
                min.b = min.b.max(round.b);
            }
            min.r * min.g * min.b
        })
        .sum::<i64>()
        .to_string())
}
