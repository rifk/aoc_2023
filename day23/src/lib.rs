use eyre::{eyre, Result};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq)]
enum Map {
    Tree,
    Path,
    SlopeUp,
    SlopeDown,
    SlopeLeft,
    SlopeRight,
}

#[derive(Clone, Debug)]
struct Hike {
    pos: (usize, usize),
    seen: HashSet<(usize, usize)>,
}
impl Hike {
    fn new(start: (usize, usize)) -> Self {
        Self {
            pos: start,
            seen: {
                let mut s = HashSet::new();
                s.insert(start);
                s
            },
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Vec<Map>>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => Ok(Map::Tree),
                    '.' => Ok(Map::Path),
                    '^' => Ok(Map::SlopeUp),
                    'v' => Ok(Map::SlopeDown),
                    '<' => Ok(Map::SlopeLeft),
                    '>' => Ok(Map::SlopeRight),
                    _ => Err(eyre!("unknown map char {}", c)),
                })
                .collect::<Result<Vec<Map>>>()
        })
        .collect()
}

fn next_steps(map: &[Vec<Map>], pos: (usize, usize)) -> Vec<(usize, usize)> {
    match map[pos.0][pos.1] {
        Map::SlopeUp => return vec![(pos.0 - 1, pos.1)],
        Map::SlopeDown => return vec![(pos.0 + 1, pos.1)],
        Map::SlopeLeft => return vec![(pos.0, pos.1 - 1)],
        Map::SlopeRight => return vec![(pos.0, pos.1 + 1)],
        _ => {}
    }
    let mut steps = vec![];
    if pos.0 > 0 && map[pos.0 - 1][pos.1] != Map::Tree {
        steps.push((pos.0 - 1, pos.1));
    }
    if pos.0 < map.len() - 1 && map[pos.0 + 1][pos.1] != Map::Tree {
        steps.push((pos.0 + 1, pos.1));
    }
    if pos.1 > 0 && map[pos.0][pos.1 - 1] != Map::Tree {
        steps.push((pos.0, pos.1 - 1));
    }
    if pos.1 < map[0].len() - 1 && map[pos.0][pos.1 + 1] != Map::Tree {
        steps.push((pos.0, pos.1 + 1));
    }
    steps
}

pub fn solve_one(input: &str) -> Result<String> {
    let map = parse_input(input)?;

    let start = (
        0,
        map[0]
            .iter()
            .enumerate()
            .find_map(|(j, m)| if *m == Map::Path { Some(j) } else { None })
            .ok_or(eyre!("no path on top row"))?,
    );

    let mut hikes = vec![Hike::new(start)];
    let mut max = 0;
    while !hikes.is_empty() {
        hikes = hikes
            .into_iter()
            .filter(|h| {
                if h.pos.0 == map.len() - 1 {
                    max = max.max(h.seen.len());
                    false
                } else {
                    true
                }
            })
            .flat_map(|h| {
                next_steps(&map, h.pos)
                    .into_iter()
                    .filter_map(move |next_pos| {
                        if h.seen.contains(&next_pos) {
                            None
                        } else {
                            Some(Hike {
                                pos: next_pos,
                                seen: {
                                    let mut s = h.seen.clone();
                                    s.insert(next_pos);
                                    s
                                },
                            })
                        }
                    })
            })
            .collect();
    }

    Ok((max - 1).to_string())
}

#[derive(Clone, Debug)]
struct Node {
    end: bool,
    to_dist: Vec<(u16, u64)>,
}

fn to_node_map(map: &[Vec<Map>]) -> HashMap<u16, Node> {
    let mut node_ids = HashMap::new();
    let mut next_id = 0;
    let mut node_map = HashMap::new();
    (0..map.len()).for_each(|i| {
        (0..map[0].len())
            .filter(|j| map[i][*j] == Map::Path)
            .for_each(|j| {
                let ns = next_steps(map, (i, j));
                if ns.len() != 2 {
                    let id = if let Some(id) = node_ids.get(&(i, j)) {
                        *id
                    } else {
                        let id = next_id;
                        node_ids.insert((i, j), id);
                        next_id += 1;
                        id
                    };
                    node_map.insert(
                        id,
                        Node {
                            end: i == map.len() - 1,
                            to_dist: ns
                                .into_iter()
                                .map(|mut to| {
                                    let mut from = (i, j);
                                    let mut len = 1;
                                    let to_id;
                                    loop {
                                        let ns = next_steps(map, to);
                                        if ns.len() == 2 {
                                            let next_from = to;
                                            to = if ns[0] == from { ns[1] } else { ns[0] };
                                            from = next_from;
                                            len += 1;
                                        } else {
                                            to_id = if let Some(id) = node_ids.get(&to) {
                                                *id
                                            } else {
                                                let id = next_id;
                                                node_ids.insert(to, id);
                                                next_id += 1;
                                                id
                                            };
                                            break;
                                        }
                                    }
                                    (to_id, len)
                                })
                                .collect(),
                        },
                    );
                }
            });
    });
    node_map
}

#[derive(Clone, Debug)]
struct HikeNodes {
    pos: u16,
    seen: HashSet<u16>,
    dist: u64,
}
impl HikeNodes {
    fn new() -> Self {
        Self {
            pos: 0,
            seen: {
                let mut s = HashSet::new();
                s.insert(0);
                s
            },
            dist: 0,
        }
    }
}

pub fn solve_two(input: &str) -> Result<String> {
    let mut map = parse_input(input)?;

    map.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|p| match p {
            Map::SlopeUp | Map::SlopeDown | Map::SlopeLeft | Map::SlopeRight => {
                *p = Map::Path;
            }
            _ => {}
        })
    });

    let node_map = to_node_map(&map);

    let mut hikes = vec![HikeNodes::new()];
    let mut max = 0;
    while !hikes.is_empty() {
        hikes = hikes
            .into_iter()
            .flat_map(|hike| {
                let node = if let Some(n) = node_map.get(&hike.pos) {
                    n
                } else {
                    return vec![Err(eyre!("missing node from map {}", hike.pos))];
                };
                if node.end {
                    max = max.max(hike.dist);
                    return vec![];
                }
                node.to_dist
                    .iter()
                    .filter_map(|(to_id, dist)| {
                        if hike.seen.contains(to_id) {
                            None
                        } else {
                            Some(Ok(HikeNodes {
                                pos: *to_id,
                                seen: {
                                    let mut seen = hike.seen.clone();
                                    seen.insert(*to_id);
                                    seen
                                },
                                dist: hike.dist + dist,
                            }))
                        }
                    })
                    .collect()
            })
            .collect::<Result<_>>()?;
    }

    Ok(max.to_string())
}
