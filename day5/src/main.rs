use clap::Parser;
use core::ops::Range;
use eyre::{eyre, Result};
use std::collections::HashSet;

fn main() -> Result<()> {
    let args = utils::Args::parse();
    let input = args.get_input(5)?;

    let (seeds, maps) = input.split_once('\n').ok_or(eyre!("missing new line"))?;

    let seeds = seeds
        .strip_prefix("seeds: ")
        .ok_or(eyre!("missing prefix"))?
        .split(' ')
        .map(|v| Ok(Seed(v.parse::<i64>()?)))
        .collect::<Result<Vec<Seed>>>()?;

    let maps = Maps::parse_input(maps.trim())?;

    if args.run_one() {
        println!("part one:\n{}", solve_one(&seeds, &maps)?);
    }
    if args.run_two() {
        println!("part one:\n{}", solve_two(&input)?);
    }

    Ok(())
}

macro_rules! map {
    ($from:ty, $to:ty, $map:ident) => {
        impl $to {
            fn new(v: i64) -> Self {
                Self(v)
            }
        }
        impl $from {
            fn map(&self, maps: &Maps) -> $to {
                <$to>::new(
                    maps.$map
                        .iter()
                        .find(|(r, _)| r.contains(&self.0))
                        .map(|(_, d)| self.0 + d)
                        .unwrap_or(self.0),
                )
            }
        }
    };
}
struct Seed(i64);
map!(Seed, Soil, seed_soil);
struct Soil(i64);
map!(Soil, Fertilizer, soil_fert);
struct Fertilizer(i64);
map!(Fertilizer, Water, fert_watr);
struct Water(i64);
map!(Water, Light, watr_lght);
struct Light(i64);
map!(Light, Temperature, lght_temp);
struct Temperature(i64);
map!(Temperature, Humidity, temp_humd);
struct Humidity(i64);
map!(Humidity, Location, humd_loct);
struct Location(i64);

#[derive(Debug)]
struct Maps {
    seed_soil: Vec<(Range<i64>, i64)>,
    soil_fert: Vec<(Range<i64>, i64)>,
    fert_watr: Vec<(Range<i64>, i64)>,
    watr_lght: Vec<(Range<i64>, i64)>,
    lght_temp: Vec<(Range<i64>, i64)>,
    temp_humd: Vec<(Range<i64>, i64)>,
    humd_loct: Vec<(Range<i64>, i64)>,
}
impl Maps {
    fn parse_input(maps: &str) -> Result<Self> {
        fn gen_map(m: &str) -> Result<Vec<(Range<i64>, i64)>> {
            m.trim()
                .lines()
                .skip(1)
                .map(|l| {
                    let l = l
                        .split(' ')
                        .map(|v| Ok(v.parse::<i64>()?))
                        .collect::<Result<Vec<i64>>>()?;
                    Ok((l[1]..l[1] + l[2], l[0] - l[1]))
                })
                .collect::<Result<Vec<(Range<i64>, i64)>>>()
        }
        let mut maps = maps.split("\n\n");
        Ok(Self {
            seed_soil: gen_map(maps.next().ok_or(eyre!("missing next"))?)?,
            soil_fert: gen_map(maps.next().ok_or(eyre!("missing next"))?)?,
            fert_watr: gen_map(maps.next().ok_or(eyre!("missing next"))?)?,
            watr_lght: gen_map(maps.next().ok_or(eyre!("missing next"))?)?,
            lght_temp: gen_map(maps.next().ok_or(eyre!("missing next"))?)?,
            temp_humd: gen_map(maps.next().ok_or(eyre!("missing next"))?)?,
            humd_loct: gen_map(maps.next().ok_or(eyre!("missing next"))?)?,
        })
    }
}

fn solve_one(seeds: &[Seed], maps: &Maps) -> Result<String> {
    Ok(seeds
        .iter()
        .map(|s| {
            s.map(&maps) // soil
                .map(&maps) // fert
                .map(&maps) // water
                .map(&maps) // light
                .map(&maps) // temp
                .map(&maps) // humd
                .map(&maps) // loc
                .0
        })
        .min()
        .ok_or(eyre!("no values"))?
        .to_string())
}

fn solve_two(input: &str) -> Result<String> {
    todo!()
}
