use eyre::{eyre, Result};
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug, PartialEq)]
enum Pulse {
    High,
    Low,
}
#[derive(Clone, Debug)]
enum Module {
    FlipFlop {
        on: bool,
        output: Vec<String>,
    },
    Conjuction {
        input: HashMap<String, Pulse>,
        output: Vec<String>,
    },
    Broadcast {
        output: Vec<String>,
    },
}
impl Module {
    fn pulse(&mut self, from: &str, pulse: Pulse) -> Vec<(&str, Pulse)> {
        match self {
            Self::FlipFlop { on, output } => match pulse {
                Pulse::High => vec![],
                Pulse::Low => {
                    *on = !*on;
                    let send = if *on { Pulse::High } else { Pulse::Low };
                    output.iter().map(|o| (o.as_ref(), send.clone())).collect()
                }
            },
            Self::Conjuction { input, output } => {
                input.entry(from.to_string()).and_modify(|p| *p = pulse);
                let send = if input.values().any(|p| p == &Pulse::Low) {
                    Pulse::High
                } else {
                    Pulse::Low
                };
                output.iter().map(|o| (o.as_ref(), send.clone())).collect()
            }
            Self::Broadcast { output } => {
                output.iter().map(|o| (o.as_ref(), pulse.clone())).collect()
            }
        }
    }
}

fn parse_input(input: &str) -> Result<HashMap<String, Module>> {
    let modules = input
        .lines()
        .map(|l| {
            let (n, to) = l
                .split_once(" -> ")
                .ok_or(eyre!("no \" -> \" in line {}", l))?;
            let (n, m) = if let Some(n) = n.strip_prefix('%') {
                (n, '%')
            } else if let Some(n) = n.strip_prefix('&') {
                (n, '&')
            } else if n == "broadcaster" {
                (n, 'b')
            } else {
                eyre::bail!("unknown module type {}", n);
            };
            Ok((n, (m, to.split(", ").collect())))
        })
        .collect::<Result<HashMap<&str, (char, Vec<&str>)>>>()?;

    let mut m = HashMap::new();
    for (n, (c, to)) in &modules {
        match c {
            '%' => {
                m.insert(
                    n.to_string(),
                    Module::FlipFlop {
                        on: false,
                        output: to.iter().map(|t| t.to_string()).collect(),
                    },
                );
            }
            '&' => {
                m.insert(
                    n.to_string(),
                    Module::Conjuction {
                        input: modules
                            .iter()
                            .filter(|(_, (_, to))| to.contains(n))
                            .map(|(from, _)| (from.to_string(), Pulse::Low))
                            .collect(),
                        output: to.iter().map(|t| t.to_string()).collect(),
                    },
                );
            }
            'b' => {
                m.insert(
                    n.to_string(),
                    Module::Broadcast {
                        output: to.iter().map(|t| t.to_string()).collect(),
                    },
                );
            }
            _ => eyre::bail!("unknown module type {}", c),
        }
    }

    Ok(m)
}

pub fn solve_one(input: &str) -> Result<String> {
    let mut modules = parse_input(input)?;
    let mut high = 0;
    let mut low = 0;
    for _ in 0..1000 {
        let mut pulses = VecDeque::new();
        pulses.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));
        while let Some((from, to, p)) = pulses.pop_front() {
            match &p {
                Pulse::Low => {
                    low += 1;
                }
                Pulse::High => {
                    high += 1;
                }
            }
            if let Some(to_mod) = modules.get_mut(&to) {
                let outgoing = to_mod.pulse(&from, p);
                outgoing.into_iter().for_each(|(out_to, out_p)| {
                    pulses.push_back((to.clone(), out_to.to_string(), out_p))
                });
            }
        }
    }
    Ok((high * low).to_string())
}

pub fn solve_two(input: &str) -> Result<String> {
    let mut modules = parse_input(input)?;
    let to_rx = modules.iter().find_map(|(n, m)| match m {
        Module::Conjuction { output, .. } => {
            if output.contains(&"rx".to_string()) {
                Some(n)
            } else {
                None
            }
        }
        _ => None,
    });
    if let Some(to_rx) = to_rx {
        let mut prevs = modules
            .iter()
            .filter_map(|(n, m)| match m {
                Module::FlipFlop { output, .. } => {
                    if output.contains(to_rx) {
                        Some((n.clone(), None))
                    } else {
                        None
                    }
                }
                Module::Conjuction { output, .. } => {
                    if output.contains(to_rx) {
                        Some((n.clone(), None))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect::<HashMap<String, Option<u64>>>();
        println!("{:?}", prevs);
        let mut button_count = 0;
        while prevs.values().any(|v| v.is_none()) {
            let mut pulses = VecDeque::new();
            pulses.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));
            button_count += 1;
            while let Some((from, to, p)) = pulses.pop_front() {
                if p == Pulse::Low {
                    if let Some(period) = prevs.get_mut(&to) {
                        if period.is_none() {
                            *period = Some(button_count);
                        }
                    }
                }
                if let Some(to_mod) = modules.get_mut(&to) {
                    let outgoing = to_mod.pulse(&from, p);
                    outgoing.into_iter().for_each(|(out_to, out_p)| {
                        pulses.push_back((to.clone(), out_to.to_string(), out_p))
                    });
                }
            }
        }
        prevs
            .values()
            .try_fold(1, |lcm, v| {
                Ok(num_integer::lcm(lcm, v.ok_or(eyre!("missing value"))?))
            })
            .map(|lcm| lcm.to_string())
    } else {
        long_rx_search(modules)
    }
}

fn long_rx_search(mut modules: HashMap<String, Module>) -> Result<String> {
    let mut rx = false;
    let mut button_count = 0;
    while !rx {
        let mut pulses = VecDeque::new();
        pulses.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));
        button_count += 1;
        while let Some((from, to, p)) = pulses.pop_front() {
            if to == "rx" && p == Pulse::Low {
                rx = true;
            }
            if let Some(to_mod) = modules.get_mut(&to) {
                let outgoing = to_mod.pulse(&from, p);
                outgoing.into_iter().for_each(|(out_to, out_p)| {
                    pulses.push_back((to.clone(), out_to.to_string(), out_p))
                });
            }
        }
    }
    Ok((button_count).to_string())
}
