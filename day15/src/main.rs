use utils::derive::aoc;

fn hash(s: &str) -> u64 {
    let mut h = 0;
    s.chars().for_each(|c| {
        h += c as u64;
        h *= 17;
        h %= 256;
    });
    h
}

fn find_in_box(b: &[(&str, u64)], label: &str) -> Option<usize> {
    b.iter()
        .enumerate()
        .find_map(|(i, (l, _))| if l == &label { Some(i) } else { None })
}

#[aoc(day15, part1)]
fn solve_one(input: &str) -> Result<String> {
    Ok(input
        .replace('\n', "")
        .split(',')
        .map(|s| hash(s))
        .sum::<u64>()
        .to_string())
}

#[aoc(day15, part2)]
fn solve_two(input: &str) -> Result<String> {
    let mut boxes: Vec<Vec<(&str, u64)>> = vec![vec![]; 256];
    let input = input.replace('\n', "");
    input.split(',').try_for_each(|s| {
        if let Some((label, lens)) = s.split_once('=') {
            let b = &mut boxes[hash(label) as usize];
            let lens = lens.parse::<u64>()?;
            if let Some(i) = find_in_box(b, label) {
                b[i] = (label, lens);
            } else {
                b.push((label, lens));
            }
        } else {
            let label = s.trim_end_matches('-');
            let b = &mut boxes[hash(label) as usize];
            if let Some(i) = find_in_box(b, label) {
                b.remove(i);
            }
        }
        Ok::<(), eyre::Error>(())
    })?;

    Ok(boxes
        .iter()
        .enumerate()
        .flat_map(|(bi, b)| {
            b.iter()
                .enumerate()
                .map(move |(li, (_, v))| v * (1 + bi as u64) * (1 + li as u64))
        })
        .sum::<u64>()
        .to_string())
}
