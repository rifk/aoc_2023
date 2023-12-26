use eyre::{eyre, Result};

const OFFSET: i128 = 300_000_000_000_000;
const AREA: f64 = 100_000_000_000_000.0;

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> Result<Vec<((i128, i128, i128), (i128, i128, i128))>> {
    input
        .lines()
        .map(|l| {
            let (pos, vel) = l
                .split_once(" @ ")
                .ok_or(eyre!("missing ' @ ' in line {}", l))?;
            let pos = pos
                .split(", ")
                .map(|v| Ok(v.parse::<i128>()?))
                .collect::<Result<Vec<i128>>>()?;
            let vel = vel
                .split(", ")
                .map(|v| Ok(v.parse::<i128>()?))
                .collect::<Result<Vec<i128>>>()?;
            if pos.len() != 3 || vel.len() != 3 {
                eyre::bail!("expected 3 position and velocity co-ords - {}", l);
            }
            Ok((
                (pos[0] - OFFSET, pos[1] - OFFSET, pos[2] - OFFSET),
                (vel[0], vel[1], vel[2]),
            ))
        })
        .collect()
}

pub fn solve_one(input: &str) -> Result<String> {
    let stones = &parse_input(input)?;
    Ok((0..stones.len() - 1)
        .flat_map(|a| (a + 1..stones.len()).map(move |b| (stones[a], stones[b])))
        .filter(|((a_pos, a_vel), (b_pos, b_vel))| {
            let a_time = {
                let d = (b_vel.0 * a_vel.1) - (b_vel.1 * a_vel.0);
                if d == 0 {
                    // stone a and b are moving in parallel never collide
                    return false;
                }
                let n = (b_vel.1 * (a_pos.0 - b_pos.0)) + (b_vel.0 * (b_pos.1 - a_pos.1));
                n as f64 / d as f64
            };
            let b_time = {
                let n = (a_vel.1 * (b_pos.0 - a_pos.0)) + (a_vel.0 * (a_pos.1 - b_pos.1));
                let d = (a_vel.0 * b_vel.1) - (a_vel.1 * b_vel.0);
                n as f64 / d as f64
            };

            if a_time < 0.0 || b_time < 0.0 {
                // happens in past
                return false;
            }

            if (a_pos.0 as f64 + (a_time * a_vel.0 as f64)).abs() > AREA
                || (a_pos.1 as f64 + (a_time * a_vel.1 as f64)).abs() > AREA
            {
                // happened outside test area
                return false;
            }

            true
        })
        .count()
        .to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let stones = &parse_input(input)?;

    let mut pos_sum = 0;
    'z: for z in -1000..=1000 {
        'y: for y in -1000..=1000 {
            'x: for x in -1000..=1000 {
                let yz_cross = {
                    let (a_pos, a_vel) = stones[0];
                    let (b_pos, b_vel) = stones[1];
                    let a_vel = (a_vel.0 - x, a_vel.1 - y, a_vel.2 - z);
                    let b_vel = (b_vel.0 - x, b_vel.1 - y, b_vel.2 - z);

                    let a_time = {
                        let d = (b_vel.1 * a_vel.2) - (b_vel.2 * a_vel.1);
                        if d == 0 {
                            // stone a and b are moving in parallel never collide
                            continue 'y;
                        }
                        let n = (b_vel.2 * (a_pos.1 - b_pos.1)) + (b_vel.1 * (b_pos.2 - a_pos.2));
                        (n.div_euclid(d), n.rem_euclid(d), d)
                    };
                    let b_time = {
                        let n = (a_vel.2 * (b_pos.1 - a_pos.1)) + (a_vel.1 * (a_pos.2 - b_pos.2));
                        let d = (a_vel.1 * b_vel.2) - (a_vel.2 * b_vel.1);
                        (n.div_euclid(d), n.rem_euclid(d), d)
                    };

                    if a_time.0 < 0 || b_time.0 < 0 {
                        // happens in past
                        continue 'y;
                    }

                    let a_y = pos_with_frac(a_pos.1, a_vel.1, a_time);

                    let a_z = pos_with_frac(a_pos.2, a_vel.2, a_time);
                    (a_y, a_z)
                };

                for b in &stones[2..] {
                    let (a_pos, a_vel) = stones[0];
                    let (b_pos, b_vel) = b;
                    let a_vel = (a_vel.0 - x, a_vel.1 - y, a_vel.2 - z);
                    let b_vel = (b_vel.0 - x, b_vel.1 - y, b_vel.2 - z);
                    let b_time = {
                        let d = (a_vel.1 * b_vel.2) - (a_vel.2 * b_vel.1);
                        if d == 0 {
                            continue 'y;
                        }
                        let n = (a_vel.2 * (b_pos.1 - a_pos.1)) + (a_vel.1 * (a_pos.2 - b_pos.2));
                        (n.div_euclid(d), n.rem_euclid(d), d)
                    };

                    if b_time.0 < 0 {
                        // happens in past
                        continue 'y;
                    }

                    let b_y = pos_with_frac(b_pos.1, b_vel.1, b_time);
                    let b_z = pos_with_frac(b_pos.2, b_vel.2, b_time);

                    if yz_cross.0 != b_y || yz_cross.1 != b_z {
                        continue 'y;
                    }
                }

                let x_cross = {
                    let (a_pos, a_vel) = stones[0];
                    let (b_pos, b_vel) = stones[1];
                    let a_vel = (a_vel.0 - x, a_vel.1 - y, a_vel.2 - z);
                    let b_vel = (b_vel.0 - x, b_vel.1 - y, b_vel.2 - z);

                    let a_time = {
                        let d = (b_vel.0 * a_vel.2) - (b_vel.2 * a_vel.0);
                        if d == 0 {
                            // stone a and b are moving in parallel never collide
                            continue 'x;
                        }
                        let n = (b_vel.2 * (a_pos.0 - b_pos.0)) + (b_vel.0 * (b_pos.2 - a_pos.2));
                        (n.div_euclid(d), n.rem_euclid(d), d)
                    };
                    let b_time = {
                        let n = (a_vel.2 * (b_pos.0 - a_pos.0)) + (a_vel.0 * (a_pos.2 - b_pos.2));
                        let d = (a_vel.0 * b_vel.2) - (a_vel.2 * b_vel.0);
                        (n.div_euclid(d), n.rem_euclid(d), d)
                    };

                    if a_time.0 < 0 || b_time.0 < 0 {
                        // happens in past
                        continue 'x;
                    }

                    pos_with_frac(a_pos.0, a_vel.0, a_time)
                };

                for b in &stones[2..] {
                    let (a_pos, a_vel) = stones[0];
                    let (b_pos, b_vel) = b;
                    let a_vel = (a_vel.0 - x, a_vel.1 - y, a_vel.2 - z);
                    let b_vel = (b_vel.0 - x, b_vel.1 - y, b_vel.2 - z);
                    let b_time = {
                        let d = (a_vel.0 * b_vel.2) - (a_vel.2 * b_vel.0);
                        if d == 0 {
                            continue 'x;
                        }
                        let n = (a_vel.2 * (b_pos.0 - a_pos.0)) + (a_vel.0 * (a_pos.2 - b_pos.2));
                        (n.div_euclid(d), n.rem_euclid(d), d)
                    };

                    if b_time.0 < 0 {
                        // happens in past
                        continue 'x;
                    }

                    let b_x = pos_with_frac(b_pos.0, b_vel.0, b_time);

                    if x_cross != b_x {
                        continue 'x;
                    }
                }

                pos_sum = x_cross.0 + yz_cross.0 .0 + yz_cross.1 .0;
                break 'z;
            }
        }
    }

    Ok((pos_sum + (3 * OFFSET)).to_string())
}

fn pos_with_frac(p: i128, v: i128, r: (i128, i128, i128)) -> (i128, i128, i128) {
    let n = v * r.1;
    let i = p + (v * r.0) + n.div_euclid(r.2);
    let n = n.rem_euclid(r.2);
    if n == 0 {
        (i, 0, 1)
    } else {
        let gcd = num_integer::gcd(n, r.2);
        let n = n / gcd;
        let d = r.2 / gcd;
        if d < 0 {
            (i - 1, (d + n).abs(), -d)
        } else {
            (i, n, d)
        }
    }
}
