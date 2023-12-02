use clap::Parser;
use eyre::{bail, eyre, Result};

fn main() -> Result<()> {
    let args = utils::Args::parse();
    let input = args.get_input(2)?;

    let games = parse_input(&input)?;

    if args.run_one() {
        println!("part one:\n{}", solve_one(&games)?);
    }
    if args.run_two() {
        println!("part one:\n{}", solve_two(&games)?);
    }

    Ok(())
}

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

fn solve_one(games: &[Vec<Rgb>]) -> Result<String> {
    Ok(games
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

fn solve_two(games: &[Vec<Rgb>]) -> Result<String> {
    Ok(games
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
