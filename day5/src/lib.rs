use core::ops::Range;
use eyre::{eyre, Result};

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

pub fn solve_one(input: &str) -> Result<String> {
    let (seeds, maps) = input.split_once('\n').ok_or(eyre!("missing new line"))?;
    let seeds = &seeds
        .strip_prefix("seeds: ")
        .ok_or(eyre!("missing prefix"))?
        .split(' ')
        .map(|v| Ok(Seed(v.parse::<i64>()?)))
        .collect::<Result<Vec<Seed>>>()?;
    let maps = &Maps::parse_input(maps.trim())?;

    Ok(seeds
        .iter()
        .map(|s| {
            s.map(maps) // soil
                .map(maps) // fert
                .map(maps) // water
                .map(maps) // light
                .map(maps) // temp
                .map(maps) // humd
                .map(maps) // loc
                .0
        })
        .min()
        .ok_or(eyre!("no values"))?
        .to_string())
}

// Part two - part one solution doesnt generalise nicely to part two, so not reusing
pub fn solve_two(input: &str) -> Result<String> {
    let (seeds, maps) = input.split_once('\n').ok_or(eyre!("missing new line"))?;
    let seeds = &seeds
        .strip_prefix("seeds: ")
        .ok_or(eyre!("missing prefix"))?
        .split(' ')
        .map(|v| Ok(v.parse::<i64>()?))
        .collect::<Result<Vec<i64>>>()?
        .chunks(2)
        .map(|c| c[0]..c[0] + c[1])
        .collect::<Vec<Range<i64>>>();
    let maps = &Maps::parse_input(maps.trim())?;

    let soil = map_ranges(seeds, &maps.seed_soil);
    let fert = map_ranges(&soil, &maps.soil_fert);
    let water = map_ranges(&fert, &maps.fert_watr);
    let light = map_ranges(&water, &maps.watr_lght);
    let temp = map_ranges(&light, &maps.lght_temp);
    let humd = map_ranges(&temp, &maps.temp_humd);
    let loc = map_ranges(&humd, &maps.humd_loct);
    Ok(loc
        .iter()
        .map(|l| l.start)
        .min()
        .ok_or(eyre!("missing min"))?
        .to_string())
}

fn map_ranges(from: &[Range<i64>], map: &[(Range<i64>, i64)]) -> Vec<Range<i64>> {
    let mut orig = from.to_vec();
    let mut mapped = vec![];

    map.iter().for_each(|(r, d)| {
        let (new_orig, mut add_mapped) = orig
            .iter()
            .map(|o| {
                if o.end <= r.start || r.end <= o.start {
                    // no overlap
                    (vec![o.clone()], None)
                } else if o.start >= r.start && o.end <= r.end {
                    // o fully in r
                    (vec![], Some(o.start + d..o.end + d))
                } else if r.start >= o.start && r.end <= o.end {
                    // r fully in o
                    (
                        vec![o.start..r.start, r.end..o.end],
                        Some(r.start + d..o.start + d),
                    )
                } else if o.start < r.start {
                    // o before r with overlap
                    #[allow(clippy::single_range_in_vec_init)]
                    (vec![o.start..r.start], Some(r.start + d..o.end + d))
                } else {
                    // o after r with overlap
                    #[allow(clippy::single_range_in_vec_init)]
                    (vec![r.end..o.end], Some(o.start + d..r.end + d))
                }
            })
            .fold((vec![], vec![]), |mut out, (mut o, m)| {
                out.0.append(&mut o);
                if let Some(m) = m {
                    out.1.push(m);
                }
                out
            });
        orig = new_orig;
        mapped.append(&mut add_mapped);
    });

    mapped.append(&mut orig);
    mapped
}
