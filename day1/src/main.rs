use clap::Parser;
use eyre::Result;

fn main() -> Result<()> {
    let args = utils::Args::parse();
    let input = args.get_input(1)?;

    if args.run_one() {
        println!("part one:\n{}", solve_one(&input)?);
    }
    if args.run_two() {
        println!("part two:\n{}", solve_two(&input)?);
    }

    Ok(())
}

fn solve_one(input: &str) -> Result<String> {
    Ok(input
        .lines()
        .map(line_to_num)
        .collect::<Result<Vec<i64>>>()?
        .into_iter()
        .sum::<i64>()
        .to_string())
}

fn line_to_num(l: &str) -> Result<i64> {
    let mut l = l.trim_matches(char::is_alphabetic).to_string();
    if l.len() > 1 {
        l.replace_range(1..(l.len() - 1), "");
    } else {
        l = l.clone() + &l;
    }
    Ok(l.parse::<i64>()?)
}

fn solve_two(input: &str) -> Result<String> {
    solve_one(&replace(input))
}

fn replace(s: &str) -> String {
    let s = s.replace("one", "one1one");
    let s = s.replace("two", "two2two");
    let s = s.replace("three", "three3three");
    let s = s.replace("four", "four4four");
    let s = s.replace("five", "five5five");
    let s = s.replace("six", "six6six");
    let s = s.replace("seven", "seven7seven");
    let s = s.replace("eight", "eight8eight");
    s.replace("nine", "nine9nine")
}
