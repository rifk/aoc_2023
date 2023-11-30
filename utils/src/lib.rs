use clap::Parser;
use eyre::{bail, eyre, Result};
use reqwest::blocking::Client;
use reqwest::header::COOKIE;
use std::env;
use std::fs;

#[derive(Debug, Parser)]
#[command(long_about = None)]
pub struct Args {
    /// input file, AOC_SESSION env must be set if not specified
    #[arg(short, long)]
    input: Option<String>,
    /// run part one, will run both parts if --one and --two not specified
    #[arg(short, long)]
    one: bool,
    /// run part two, will run both parts if --one and --two not specified
    #[arg(short, long)]
    two: bool,
}
impl Args {
    pub fn get_input(&self, day: i32) -> Result<String> {
        if let Some(file) = &self.input {
            Ok(fs::read_to_string(file)?)
        } else if let Some(session) = env::var_os("AOC_SESSION") {
            let client = Client::new();
            Ok(client
                .get(format!("https://adventofcode.com/2023/day/{}/input", day))
                .header(
                    COOKIE,
                    format!(
                        "session={}",
                        session
                            .to_str()
                            .ok_or_else(|| eyre!("cannot convert env to str"))?
                    ),
                )
                .send()?
                .text()?)
        } else {
            bail!("no input file provided or AOC_SESSION set");
        }
    }

    pub fn run_one(&self) -> bool {
        self.one || !self.two
    }

    pub fn run_two(&self) -> bool {
        self.two || !self.one
    }
}
