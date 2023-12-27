use eyre::{eyre, Result};
use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> Result<HashMap<String, Vec<String>>> {
    input
        .lines()
        .map(|l| {
            let (k, v) = l
                .split_once(": ")
                .ok_or(eyre!("missing ': ' in line {}", l))?;
            Ok((k.to_string(), v.split(' ').map(|s| s.to_string()).collect()))
        })
        .collect()
}

fn fill_conns(mut conns: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    conns.clone().into_iter().for_each(|(k, v)| {
        v.into_iter().for_each(|v| {
            conns
                .entry(v)
                .and_modify(|vec| {
                    vec.push(k.clone());
                })
                .or_insert(vec![k.clone()]);
        });
    });
    conns
}

#[derive(Clone, Debug)]
struct Split {
    g1: HashSet<String>,
    g2: HashSet<String>,
    c: HashMap<String, HashSet<String>>,
}

pub fn solve_one(input: &str) -> Result<String> {
    let conns = fill_conns(parse_input(input)?);

    let mut split = {
        let mut g1 = conns.keys().cloned().collect::<HashSet<_>>();
        let (k, v) = conns.iter().next().ok_or(eyre!("conns empty"))?;
        let mut g2 = HashSet::new();
        g2.insert(g1.take(k).ok_or(eyre!("{} missing from g1", k))?);
        let mut c = HashMap::new();
        v.iter().for_each(|v| {
            c.insert(v.clone(), {
                let mut s = HashSet::new();
                s.insert(k.clone());
                s
            });
        });
        Split { g1, g2, c }
    };

    while split.c.values().map(|v| v.len()).sum::<usize>() != 3 {
        let max_conns = split
            .c
            .iter()
            .max_by_key(|(_, v)| v.len())
            .map(|(k, _)| k.clone())
            .ok_or(eyre!("empty conns"))?;
        split.g2.insert(
            split
                .g1
                .take(&max_conns)
                .ok_or(eyre!("{} missing from g1", &max_conns))?,
        );
        split.c.remove(&max_conns);
        conns
            .get(&max_conns)
            .ok_or(eyre!("{} missing from conns", &max_conns))?
            .iter()
            .for_each(|v| {
                if split.g1.contains(v) {
                    split
                        .c
                        .entry(v.clone())
                        .and_modify(|s| {
                            s.insert(max_conns.clone());
                        })
                        .or_insert({
                            let mut s = HashSet::new();
                            s.insert(max_conns.clone());
                            s
                        });
                }
            });
    }

    Ok((split.g1.len() * split.g2.len()).to_string())
}

pub fn solve_two(_: &str) -> Result<String> {
    Ok(0.to_string())
}
