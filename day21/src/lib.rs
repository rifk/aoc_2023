use eyre::{eyre, Result};
use std::collections::HashSet;

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> Result<((usize, usize), Vec<Vec<bool>>)> {
    let mut s = None;
    let gs = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '.' => Ok(true),
                    '#' => Ok(false),
                    'S' => {
                        if s.is_none() {
                            s = Some((i, j));
                            Ok(true)
                        } else {
                            Err(eyre!("only one 'S' expected"))
                        }
                    }
                    _ => Err(eyre!("unknown char {}", c)),
                })
                .collect::<Result<Vec<bool>>>()
        })
        .collect::<Result<Vec<Vec<bool>>>>();
    Ok((s.ok_or(eyre!("no 'S' found"))?, gs?))
}

fn next_steps(gardens: &[Vec<bool>], step: (usize, usize)) -> Vec<(usize, usize)> {
    let mut next = vec![];
    if step.0 > 0 && gardens[step.0 - 1][step.1] {
        next.push((step.0 - 1, step.1));
    }
    if step.0 < gardens.len() - 1 && gardens[step.0 + 1][step.1] {
        next.push((step.0 + 1, step.1));
    }
    if step.1 > 0 && gardens[step.0][step.1 - 1] {
        next.push((step.0, step.1 - 1));
    }
    if step.1 < gardens[0].len() - 1 && gardens[step.0][step.1 + 1] {
        next.push((step.0, step.1 + 1));
    }
    next
}

pub fn solve_one(input: &str) -> Result<String> {
    let (s, gardens) = parse_input(input)?;
    let mut odd = HashSet::new();
    let mut new_odd = HashSet::new();
    let mut even = HashSet::new();
    even.insert(s);
    let mut new_even = HashSet::new();
    new_even.insert(s);
    for step_num in 1..=64 {
        let (steps, last_steps, new_steps) = if step_num % 2 == 0 {
            (&mut even, &new_odd, &mut new_even)
        } else {
            (&mut odd, &new_even, &mut new_odd)
        };
        *new_steps = last_steps
            .iter()
            .flat_map(|s| next_steps(&gardens, *s))
            .filter(|s| steps.insert(*s))
            .collect();
    }
    Ok(even.len().to_string())
}

fn next_steps_repeating(
    gardens: &[Vec<bool>],
    max_i: i64,
    max_j: i64,
    step: (i64, i64),
) -> Vec<(i64, i64)> {
    let mut next = Vec::with_capacity(4);
    if gardens[(step.0 - 1).rem_euclid(max_i) as usize][step.1.rem_euclid(max_j) as usize] {
        next.push((step.0 - 1, step.1));
    }
    if gardens[(step.0 + 1).rem_euclid(max_i) as usize][step.1.rem_euclid(max_j) as usize] {
        next.push((step.0 + 1, step.1));
    }
    if gardens[step.0.rem_euclid(max_i) as usize][(step.1 - 1).rem_euclid(max_j) as usize] {
        next.push((step.0, step.1 - 1));
    }
    if gardens[step.0.rem_euclid(max_i) as usize][(step.1 + 1).rem_euclid(max_j) as usize] {
        next.push((step.0, step.1 + 1));
    }
    next
}

// The steps should spread like a diamond, with repeating blocks inside, and repeating diagonals
// around the edge. Compute minimum size diamond so that we have examples of all required blocks,
// diagonals and corners and then can use those to scale up to full size.
// Gotta be careful around odd/even blocks and diagonals from the center.
pub fn solve_two(input: &str) -> Result<String> {
    let (s, gardens) = parse_input(input)?;
    let start = (s.0 as i64, s.1 as i64);
    let total_steps = 26_501_365;
    let max_i = gardens.len() as i64;
    let max_j = gardens[0].len() as i64;
    if max_i != max_j {
        eyre::bail!("only works if input width equals input height");
    }

    let div = total_steps / (max_i);
    let rem = total_steps % (max_i);

    // cover enough area so that we have:
    // - full even block
    // - full odd block
    // - NW/NE/SW/SE diagonal block
    // - all 4 corners
    let min_blocks = 2 + (div % 2);
    let min_steps = (min_blocks * max_i) + rem;
    let mut odd = HashSet::new();
    let mut new_odd = HashSet::new();
    let mut even = HashSet::new();
    even.insert(start);
    let mut new_even = HashSet::new();
    new_even.insert(start);
    for i in 1..=min_steps {
        let (steps, last_steps, new_steps) = if i % 2 == 0 {
            (&mut even, &new_odd, &mut new_even)
        } else {
            (&mut odd, &new_even, &mut new_odd)
        };
        *new_steps = last_steps
            .iter()
            .flat_map(|s| next_steps_repeating(&gardens, max_i, max_j, *s))
            .filter(|s| steps.insert(*s))
            .collect();
    }

    // finished on odd step count, use odd steps
    let even_block = odd
        .iter()
        .filter(|(i, j)| (0..max_i).contains(i) && (0..max_j).contains(j))
        .count();
    let odd_block = odd
        .iter()
        .filter(|(i, j)| (0..max_i).contains(i) && (max_j..max_j * 2).contains(j))
        .count();
    let nw_diag = odd
        .iter()
        .filter(|(i, j)| (-max_i..0).contains(i) && j < &(-max_j * (min_blocks - 2)))
        .count();
    let ne_diag = odd
        .iter()
        .filter(|(i, j)| (-max_i..0).contains(i) && j >= &(max_j * (min_blocks - 1)))
        .count();
    let sw_diag = odd
        .iter()
        .filter(|(i, j)| (max_i..max_i * 2).contains(i) && j < &(-max_j * (min_blocks - 2)))
        .count();
    let se_diag = odd
        .iter()
        .filter(|(i, j)| (max_i..max_i * 2).contains(i) && j >= &(max_j * (min_blocks - 1)))
        .count();
    let n_corner = odd
        .iter()
        .filter(|(i, _)| i < &(-max_i * (min_blocks - 1)))
        .count();
    let s_corner = odd
        .iter()
        .filter(|(i, _)| i >= &(max_i * min_blocks))
        .count();
    let w_corner = odd
        .iter()
        .filter(|(i, j)| (0..max_i).contains(i) && j < &(-max_j * (min_blocks - 1)))
        .count();
    let e_corner = odd
        .iter()
        .filter(|(i, j)| (0..max_i).contains(i) && j >= &(max_j * min_blocks))
        .count();

    // scale up add add together blocks, diags and corners
    let (more, less) = if div % 2 == 0 {
        (odd_block, even_block)
    } else {
        (even_block, odd_block)
    };
    let sum = (0..=div as usize)
        .map(|r| {
            (if r == 0 {
                n_corner + s_corner
            } else {
                let row_blocks = (r * more) + ((r - 1) * less);
                if r == div as usize {
                    w_corner + e_corner + row_blocks
                } else {
                    nw_diag + ne_diag + sw_diag + se_diag + (2 * row_blocks)
                }
            }) as u64
        })
        .sum::<u64>();

    Ok(sum.to_string())
}
