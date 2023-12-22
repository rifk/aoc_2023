use eyre::{eyre, Result};
use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> Result<Vec<Vec<Vec<Option<u16>>>>> {
    let bricks = input
        .lines()
        .map(|l| {
            let (s, e) = l.split_once('~').ok_or(eyre!("missing '~' {}", l))?;
            let s = s
                .split(',')
                .map(|v| Ok(v.parse::<usize>()?))
                .collect::<Result<Vec<usize>>>()?;
            let e = e
                .split(',')
                .map(|v| Ok(v.parse::<usize>()?))
                .collect::<Result<Vec<usize>>>()?;
            if s.len() != 3 || e.len() != 3 {
                Err(eyre!("expected 3 numbers per coordinate {}", l))
            } else {
                Ok(((s[0], s[1], s[2]), (e[0], e[1], e[2])))
            }
        })
        .collect::<Result<Vec<((usize, usize, usize), (usize, usize, usize))>>>()?;
    let mut max = (0, 0, 0);
    bricks.iter().for_each(|(bs, be)| {
        max.0 = max.0.max(bs.0).max(be.0);
        max.1 = max.1.max(bs.1).max(be.1);
        max.2 = max.2.max(bs.2).max(be.2);
    });
    max.0 += 1;
    max.1 += 1;
    max.2 += 1;
    let mut grid = vec![vec![vec![None; max.0]; max.1]; max.2];
    bricks.iter().enumerate().for_each(|(i, (s, e))| {
        for plane in grid.iter_mut().take(s.2.max(e.2) + 1).skip(s.2.min(e.2)) {
            for line in plane.iter_mut().take(s.1.max(e.1) + 1).skip(s.1.min(e.1)) {
                for point in line.iter_mut().take(s.0.max(e.0) + 1).skip(s.0.min(e.0)) {
                    *point = Some(i as u16);
                }
            }
        }
    });
    Ok(grid)
}

fn blocks_in_plane(plane: &[Vec<Option<u16>>]) -> HashMap<u16, Vec<(usize, usize)>> {
    plane
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(x, block)| block.as_ref().map(|b| (*b, (y, x))))
        })
        .fold(HashMap::new(), |mut map, (b, point)| {
            map.entry(b)
                .and_modify(|v: &mut Vec<(usize, usize)>| v.push(point))
                .or_insert(vec![point]);
            map
        })
}

fn move_blocks_down(grid: &mut Vec<Vec<Vec<Option<u16>>>>) -> Result<()> {
    for z in 1..grid.len() {
        // get map of all blocks in the current z plane and which co-ords theyre in
        let blocks = blocks_in_plane(&grid[z]);

        // move each block down
        blocks.values().try_for_each(|points| {
            // for each point in a block find how low it can go
            // max of these will be how low the block can go
            let to_z = points
                .iter()
                .map(|(y, x)| {
                    let mut low = z;
                    while low > 0 {
                        if grid[low - 1][*y][*x].is_none() {
                            low -= 1;
                        } else {
                            break;
                        }
                    }
                    low
                })
                .max()
                .ok_or(eyre!("missing max"))?;
            // move block if it can move
            if to_z != z {
                points.iter().for_each(|(y, x)| {
                    grid[to_z][*y][*x] = grid[z][*y][*x];
                    grid[z][*y][*x] = None;
                });
            }
            Ok::<(), eyre::Error>(())
        })?;
    }

    while let Some(plane) = grid.last() {
        if plane.iter().flat_map(|l| l.iter()).all(|p| p.is_none()) {
            grid.pop();
        } else {
            break;
        }
    }

    Ok(())
}

fn get_depends_supports_map(
    grid: &[Vec<Vec<Option<u16>>>],
) -> (HashMap<u16, HashSet<u16>>, HashMap<u16, HashSet<u16>>) {
    let mut depends_on = HashMap::new();
    let mut supports = HashMap::new();

    // iterate top down, storing previous plane's blocks for use in current plane
    let mut prev_blocks: HashMap<u16, Vec<(usize, usize)>> = HashMap::new();
    for plane in grid.iter().rev() {
        // get blocks in current plane
        let blocks = blocks_in_plane(plane);

        for (b, points) in blocks.iter() {
            // if block is in supports map then its already seen
            // only happens if block is standing vertically so can skip
            if supports.contains_key(b) {
                continue;
            }

            // find all blocks directly ontop of this block
            let on_top = prev_blocks
                .iter()
                .filter(|(_, top_points)| points.iter().any(|p| top_points.contains(p)))
                .map(|(top_b, _)| *top_b)
                .collect::<HashSet<u16>>();

            // add maps
            on_top.iter().for_each(|top_b| {
                depends_on
                    .entry(*top_b)
                    .and_modify(|set: &mut HashSet<u16>| {
                        set.insert(*b);
                    })
                    .or_insert({
                        let mut set = HashSet::new();
                        set.insert(*b);
                        set
                    });
            });
            supports.insert(*b, on_top);
        }
        // store blocks for next iter
        prev_blocks = blocks;
    }

    (depends_on, supports)
}

pub fn solve_one(input: &str) -> Result<String> {
    let mut grid = parse_input(input)?;

    move_blocks_down(&mut grid)?;

    let (depends_on, supports) = get_depends_supports_map(&grid);

    dbg!(supports.len());
    Ok(supports
        .values()
        .map(|supps| {
            if supps.is_empty() {
                Ok(1)
            } else {
                for s in supps {
                    if depends_on
                        .get(s)
                        .ok_or(eyre!("cannot find block {} in depends_on", s))?
                        .len()
                        == 1
                    {
                        return Ok(0);
                    }
                }
                Ok(1)
            }
        })
        .sum::<Result<usize>>()?
        .to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let mut grid = parse_input(input)?;

    move_blocks_down(&mut grid)?;

    let (depends_on, supports) = get_depends_supports_map(&grid);

    Ok(supports
        .iter()
        .map(|(b, supporting)| {
            // set of block which disintegrate
            let mut fallen = HashSet::new();
            fallen.insert(*b);
            // set of blocks which has a support removed
            let mut consider = supporting.clone();
            // keep iterating over consider set untill nothing is updated
            let mut updated = true;
            while updated {
                updated = false;
                consider.clone().into_iter().try_for_each(|c| {
                    if depends_on
                        .get(&c)
                        .ok_or(eyre!("cannot find block {} in depends_on", c))?
                        .is_subset(&fallen)
                    {
                        consider.remove(&c);
                        supports
                            .get(&c)
                            .ok_or(eyre!("cannot find block {} in depends_on", c))?
                            .iter()
                            .for_each(|s| {
                                consider.insert(*s);
                            });

                        fallen.insert(c);

                        updated = true;
                    }
                    Ok::<(), eyre::Error>(())
                })?;
            }
            Ok(fallen.len() - 1)
        })
        .sum::<Result<usize>>()?
        .to_string())
}
