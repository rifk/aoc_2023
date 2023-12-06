use eyre::Result;
use std::collections::HashSet;
use std::ops::Range;

pub fn solve_one(input: &str) -> Result<String> {
    let val_ranges = get_val_ranges(input)?;

    let to_check = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            // find none numeric none '.' in each line
            l.match_indices(|c: char| !c.is_numeric() && c != '.')
                // get surrounding indicies
                .flat_map(move |(j, _)| get_surrounding(i, j))
        })
        // collect indicies in set to remove duplicates
        .collect::<HashSet<(usize, usize)>>();

    Ok(to_check
        .into_iter()
        // get (number, line, range_on_line) for each indice
        .filter_map(|(i, j)| {
            val_ranges.get(i).and_then(|l| {
                l.iter()
                    .find(|(_, r)| r.contains(&j))
                    .map(|(v, r)| (v, i, r))
            })
        })
        // collect in set to remove duplicates
        .collect::<HashSet<(&i64, usize, &Range<usize>)>>()
        .iter()
        .map(|(&v, _, _)| v)
        .sum::<i64>()
        .to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let val_ranges = get_val_ranges(input)?;

    let gears = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| l.match_indices('*').map(move |(j, _)| (i, j)))
        .collect::<Vec<(usize, usize)>>();

    Ok(gears
        .into_iter()
        .map(|(i, j)| {
            let vals = get_surrounding(i, j)
                .into_iter()
                .filter_map(|(i, j)| {
                    val_ranges
                        .get(i)
                        .and_then(|l| l.iter().find(|(_, r)| r.contains(&j)))
                        .map(|(v, r)| (v, i, r))
                })
                .collect::<HashSet<(&i64, usize, &Range<usize>)>>()
                .into_iter()
                .map(|(v, _, _)| v)
                .collect::<Vec<&i64>>();
            if vals.len() == 2 {
                vals[0] * vals[1]
            } else {
                0
            }
        })
        .sum::<i64>()
        .to_string())
}

#[allow(clippy::type_complexity)]
fn get_val_ranges(input: &str) -> Result<Vec<Vec<(i64, Range<usize>)>>> {
    input
        .lines()
        .map(|l| {
            let mut v_r = vec![];
            let mut rem = l;
            let mut skip = 0;
            while let Some(s) = rem.find(char::is_numeric) {
                let mut e = s + 1;
                while rem.len() > e
                    && rem
                        .chars()
                        .nth(e)
                        .ok_or(eyre::eyre!("missing char"))?
                        .is_numeric()
                {
                    e += 1;
                }
                v_r.push((rem[s..e].parse::<i64>()?, skip + s..skip + e));
                skip += e;
                rem = &rem[e..];
            }
            Ok(v_r)
        })
        .collect::<Result<Vec<Vec<(i64, Range<usize>)>>>>()
}

fn get_surrounding(i: usize, j: usize) -> Vec<(usize, usize)> {
    let i_sub = i.checked_sub(1);
    let i_add = i.checked_add(1);
    let j_sub = j.checked_sub(1);
    let j_add = j.checked_add(1);
    vec![
        i_sub.and_then(|is| j_sub.map(|js| (is, js))),
        i_sub.map(|is| (is, j)),
        i_sub.and_then(|is| j_add.map(|ja| (is, ja))),
        j_sub.map(|js| (i, js)),
        j_add.map(|ja| (i, ja)),
        i_add.and_then(|ia| j_sub.map(|js| (ia, js))),
        i_add.map(|ia| (ia, j)),
        i_add.and_then(|ia| j_add.map(|ja| (ia, ja))),
    ]
    .into_iter()
    .flatten()
    .collect()
}
