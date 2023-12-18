use eyre::{eyre, Result};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Range;

#[derive(Clone, Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Clone, Debug)]
enum Category {
    X,
    M,
    A,
    S,
}
impl Category {
    fn new(cat: &str) -> Result<Self> {
        match cat {
            "x" => Ok(Self::X),
            "m" => Ok(Self::M),
            "a" => Ok(Self::A),
            "s" => Ok(Self::S),
            _ => Err(eyre!("unknown category {}", cat)),
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Destination {
    Accept,
    Reject,
    Workflow(String),
}
impl Destination {
    fn new(dest: &str) -> Self {
        match dest {
            "A" => Self::Accept,
            "R" => Self::Reject,
            w => Self::Workflow(w.to_string()),
        }
    }
}

#[derive(Clone, Debug)]
struct Rule {
    category: Category,
    val: u64,
    check: Ordering,
    destination: Destination,
}
impl Rule {
    fn check(&self, part: &Part) -> Option<Destination> {
        let v = match self.category {
            Category::X => part.x,
            Category::M => part.m,
            Category::A => part.a,
            Category::S => part.s,
        };
        if v.cmp(&self.val) == self.check {
            Some(self.destination.clone())
        } else {
            None
        }
    }
}

fn parse_workflows(input: &str) -> Result<HashMap<String, (Vec<Rule>, Destination)>> {
    input
        .lines()
        .map(|l| {
            let (name, rules) = l.split_once('{').ok_or(eyre!("missing '{{' in workflow"))?;
            let rules = rules
                .strip_suffix('}')
                .ok_or(eyre!("missing '}}' workflow suffix"))?;
            let (rules, dest) = rules
                .rsplit_once(',')
                .ok_or(eyre!("missing otherwise destination in rules"))?;
            let rules = rules
                .split(',')
                .map(|r| {
                    let (cat, check, rest) = if let Some((cat, rest)) = r.split_once('>') {
                        (cat, Ordering::Greater, rest)
                    } else {
                        let (cat, rest) = r
                            .split_once('<')
                            .ok_or(eyre!("expected '>' or '<' in rule"))?;
                        (cat, Ordering::Less, rest)
                    };
                    let (val, dest) = rest.split_once(':').ok_or(eyre!("missing ':' in rule"))?;
                    Ok(Rule {
                        category: Category::new(cat)?,
                        val: val.parse::<u64>()?,
                        check,
                        destination: Destination::new(dest),
                    })
                })
                .collect::<Result<Vec<Rule>>>()?;
            Ok((name.to_string(), (rules, Destination::new(dest))))
        })
        .collect()
}

fn parse_parts(input: &str) -> Result<Vec<Part>> {
    input
        .lines()
        .map(|l| {
            let l = l
                .strip_prefix('{')
                .ok_or(eyre!("missing '{{' at start of part"))?
                .strip_suffix('}')
                .ok_or(eyre!("missing '}}' at end of part"))?;
            let cats = l.split(',').collect::<Vec<&str>>();
            if cats.len() != 4 {
                eyre::bail!("expecting 4 categories in part");
            }
            Ok(Part {
                x: cats[0]
                    .strip_prefix("x=")
                    .ok_or(eyre!("first category should be x"))?
                    .parse::<u64>()?,
                m: cats[1]
                    .strip_prefix("m=")
                    .ok_or(eyre!("second category should be m"))?
                    .parse::<u64>()?,
                a: cats[2]
                    .strip_prefix("a=")
                    .ok_or(eyre!("third category should be a"))?
                    .parse::<u64>()?,
                s: cats[3]
                    .strip_prefix("s=")
                    .ok_or(eyre!("fourth category should be s"))?
                    .parse::<u64>()?,
            })
        })
        .collect()
}

pub fn solve_one(input: &str) -> Result<String> {
    let (workflows, parts) = input
        .split_once("\n\n")
        .ok_or(eyre!("missing workflows parts split"))?;
    let workflows = parse_workflows(workflows)?;
    let parts = parse_parts(parts)?;
    let start = Destination::new("in");
    Ok(parts
        .iter()
        .map(|p| {
            let mut d = start.clone();
            while let Destination::Workflow(wf) = d {
                let (rules, dest) = workflows
                    .get(&wf)
                    .ok_or(eyre!("cannot find workflow {}", wf))?;
                d = rules
                    .iter()
                    .find_map(|r| r.check(p))
                    .unwrap_or(dest.clone());
            }
            Ok((p, d))
        })
        .collect::<Result<Vec<(&Part, Destination)>>>()?
        .into_iter()
        .filter_map(|(p, d)| {
            if d == Destination::Accept {
                Some(p.x + p.m + p.a + p.s)
            } else {
                None
            }
        })
        .sum::<u64>()
        .to_string())
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct PossiblePart {
    x: Range<u64>,
    m: Range<u64>,
    a: Range<u64>,
    s: Range<u64>,
}
impl PossiblePart {
    fn new() -> Self {
        Self {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        }
    }
    fn restrict_success(&self, rule: &Rule) -> Option<Self> {
        self.new_restricted(&rule.category, rule.check, rule.val)
    }
    fn restrict_failure(&self, rule: &Rule) -> Option<Self> {
        let (check, val) = match rule.check {
            Ordering::Greater => (Ordering::Less, rule.val + 1),
            Ordering::Less => (Ordering::Greater, rule.val - 1),
            _ => return None,
        };
        self.new_restricted(&rule.category, check, val)
    }

    fn new_restricted(&self, category: &Category, check: Ordering, val: u64) -> Option<Self> {
        Some(match category {
            Category::X => Self {
                x: Self::restrict_range(&self.x, check, val)?,
                ..self.clone()
            },
            Category::M => Self {
                m: Self::restrict_range(&self.m, check, val)?,
                ..self.clone()
            },
            Category::A => Self {
                a: Self::restrict_range(&self.a, check, val)?,
                ..self.clone()
            },
            Category::S => Self {
                s: Self::restrict_range(&self.s, check, val)?,
                ..self.clone()
            },
        })
    }
    fn restrict_range(r: &Range<u64>, check: Ordering, val: u64) -> Option<Range<u64>> {
        match check {
            Ordering::Greater => {
                if val > r.end - 1 {
                    None
                } else {
                    Some(r.start.max(val + 1)..r.end)
                }
            }
            Ordering::Less => {
                if val < r.start {
                    None
                } else {
                    Some(r.start..r.end.min(val))
                }
            }
            _ => None,
        }
    }
}

pub fn solve_two(input: &str) -> Result<String> {
    let (workflows, _) = input
        .split_once("\n\n")
        .ok_or(eyre!("missing workflows parts split"))?;
    let workflows = parse_workflows(workflows)?;
    let mut accepted_parts = vec![];
    let mut stack = {
        let in_wf = workflows.get("in").ok_or(eyre!("no in workflow found"))?;
        vec![(in_wf, 0_usize, Some(PossiblePart::new()))]
    };
    while !stack.is_empty() {
        let (wf, rule_i, part) = stack.last_mut().ok_or(eyre!("stack should have last"))?;
        if part.is_some() && wf.0.len() > *rule_i {
            // still have rule in wf
            let rule = &wf.0[*rule_i];
            let in_part = part.as_ref().ok_or(eyre!("missing part"))?;
            match &rule.destination {
                Destination::Accept => {
                    if let Some(p) = in_part.restrict_success(rule) {
                        accepted_parts.push(p);
                    }
                    *part = in_part.restrict_failure(rule);
                    *rule_i += 1;
                }
                Destination::Reject => {
                    *part = in_part.restrict_failure(rule);
                    *rule_i += 1;
                }
                Destination::Workflow(name) => {
                    let next_wf = workflows
                        .get(name)
                        .ok_or(eyre!("missing workflow {}", name))?;
                    let next_part = in_part.restrict_success(rule);
                    stack.push((next_wf, 0_usize, next_part));
                }
            }
        } else if part.is_some() && wf.0.len() == *rule_i {
            // just past rules, go into otherwise destination
            let in_part = part.as_ref().ok_or(eyre!("missing part"))?;
            match &wf.1 {
                Destination::Accept => {
                    accepted_parts.push(in_part.clone());
                    *rule_i += 1;
                }
                Destination::Reject => {
                    *rule_i += 1;
                }
                Destination::Workflow(name) => {
                    let next_wf = workflows
                        .get(name)
                        .ok_or(eyre!("missing workflow {}", name))?;
                    let next_part = part.clone();
                    stack.push((next_wf, 0_usize, next_part));
                }
            }
        } else {
            // nothing left with this rule, pop back to previous item in stack
            // increment rule_i and update part with failure restriction
            stack.pop();
            if let Some((wf, rule_i, part)) = stack.last_mut() {
                if wf.0.len() > *rule_i {
                    if let Some(p) = part.as_mut() {
                        *part = p.restrict_failure(&wf.0[*rule_i]);
                    }
                }
                *rule_i += 1;
            }
        }
    }

    Ok(accepted_parts
        .into_iter()
        .map(|p| {
            (p.x.end - p.x.start)
                * (p.m.end - p.m.start)
                * (p.a.end - p.a.start)
                * (p.s.end - p.s.start)
        })
        .sum::<u64>()
        .to_string())
}
