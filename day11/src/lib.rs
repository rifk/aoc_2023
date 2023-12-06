use eyre::{eyre, Result};
use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, r)| {
            r.chars()
                .enumerate()
                .filter_map(move |(j, c)| if c == '#' { Some((i, j)) } else { None })
        })
        .collect()
}

fn expand(
    gals: Vec<(usize, usize)>,
    input: &str,
    expand_val: usize,
) -> Result<Vec<(usize, usize)>> {
    let max_i = input.lines().count();
    let max_j = input.lines().next().ok_or(eyre!("no first line"))?.len();

    let no_gal_i = (0..max_i)
        .filter(|i| !gals.iter().any(|(gi, _)| i == gi))
        .collect::<Vec<usize>>();
    let no_gal_j = (0..max_i)
        .filter(|j| !gals.iter().any(|(_, gj)| j == gj))
        .collect::<Vec<usize>>();

    let i_map = {
        let mut m = HashMap::new();
        let mut add = 0;
        for i in 0..max_i {
            if no_gal_i.contains(&i) {
                add += expand_val;
            } else {
                m.insert(i, i + add);
            }
        }
        m
    };

    let j_map = {
        let mut m = HashMap::new();
        let mut add = 0;
        for j in 0..max_j {
            if no_gal_j.contains(&j) {
                add += expand_val;
            } else {
                m.insert(j, j + add);
            }
        }
        m
    };

    gals.into_iter()
        .map(|(i, j)| {
            Ok((
                *i_map.get(&i).ok_or(eyre!("{} missing from i_map", i))?,
                *j_map.get(&j).ok_or(eyre!("{} missing from j_map", j))?,
            ))
        })
        .collect()
}

fn get_total_distances(gals: Vec<(usize, usize)>) -> u64 {
    (0..gals.len())
        .flat_map(|f| {
            let (f_i, f_j) = gals[f];
            let gals = &gals;
            (f..gals.len()).map(move |t| {
                let (t_i, t_j) = gals[t];
                f_i.abs_diff(t_i) as u64 + f_j.abs_diff(t_j) as u64
            })
        })
        .sum()
}

pub fn solve_one(input: &str) -> Result<String> {
    let gals = expand(parse_input(input), input, 1)?;
    Ok(get_total_distances(gals).to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let gals = expand(parse_input(input), input, 999_999)?;
    Ok(get_total_distances(gals).to_string())
}
