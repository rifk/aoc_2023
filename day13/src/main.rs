use eyre::eyre;
use utils::derive::aoc;

fn parse_note(note: &str) -> Vec<Vec<char>> {
    note.lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
}

fn find_reflection(note: &[Vec<char>]) -> Option<Reflection> {
    fn find(note: &[Vec<char>]) -> Option<usize> {
        (0..note.len() - 1).find(|&i| {
            for s in 0..=(i).min(note.len() - i - 2) {
                if note[i - s] != note[i + 1 + s] {
                    return false;
                }
            }
            true
        })
    }

    find(note)
        .map(|r| Reflection::Horizontal(r, note.len()))
        .or_else(|| {
            let transpose = (0..note[0].len())
                .map(|note_j| {
                    (0..note.len())
                        .map(|note_i| note[note_i][note_j])
                        .collect::<Vec<char>>()
                })
                .collect::<Vec<Vec<char>>>();
            find(&transpose).map(|r| Reflection::Verticle(r, transpose.len()))
        })
}

fn find_reflection_with_smudge(note: &[Vec<char>]) -> Option<Reflection> {
    fn find(note: &[Vec<char>]) -> Option<usize> {
        (0..note.len() - 1).find(|&i| {
            let mut mismatch = None;
            for s in 0..=(i).min(note.len() - i - 2) {
                if note[i - s] != note[i + 1 + s] {
                    if mismatch.is_some() {
                        return false;
                    } else {
                        mismatch = Some(((i - s), (i + 1 + s)));
                    }
                }
            }
            if let Some((r1, r2)) = mismatch {
                (0..note[0].len())
                    .filter(|&i| note[r1][i] != note[r2][i])
                    .count()
                    == 1
            } else {
                false
            }
        })
    }

    find(note)
        .map(|r| Reflection::Horizontal(r, note.len()))
        .or_else(|| {
            let transpose = (0..note[0].len())
                .map(|note_j| {
                    (0..note.len())
                        .map(|note_i| note[note_i][note_j])
                        .collect::<Vec<char>>()
                })
                .collect::<Vec<Vec<char>>>();
            find(&transpose).map(|r| Reflection::Verticle(r, transpose.len()))
        })
}

#[derive(Debug)]
enum Reflection {
    Verticle(usize, usize),
    Horizontal(usize, usize),
}

#[aoc(day13, part1)]
fn solve_one(input: &str) -> Result<String> {
    Ok(input
        .split("\n\n")
        .map(parse_note)
        .enumerate()
        .map(|(i, note)| {
            Ok(
                match find_reflection(&note).ok_or(eyre!("no reflection found in note {}", i))? {
                    Reflection::Verticle(r, _) => 1 + r as u64,
                    Reflection::Horizontal(r, _) => 100 * (1 + r as u64),
                },
            )
        })
        .sum::<Result<u64>>()?
        .to_string())
}

#[aoc(day13, part2)]
fn solve_two(input: &str) -> Result<String> {
    Ok(input
        .split("\n\n")
        .map(parse_note)
        .enumerate()
        .map(|(i, note)| {
            Ok(
                match find_reflection_with_smudge(&note)
                    .ok_or(eyre!("no reflection with smudge found in note {}", i))?
                {
                    Reflection::Verticle(r, _) => 1 + r as u64,
                    Reflection::Horizontal(r, _) => 100 * (1 + r as u64),
                },
            )
        })
        .sum::<Result<u64>>()?
        .to_string())
}
