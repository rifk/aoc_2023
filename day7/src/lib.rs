use eyre::{eyre, Result};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
enum Hand {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}
impl Hand {
    fn from_cards(c: &[i64], with_joker: bool) -> Result<Self> {
        if c.len() != 5 {
            eyre::bail!("unexpected num of cards: {:?}", c);
        }
        let count_map = c.iter().fold(HashMap::new(), |mut count, &c| {
            count
                .entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
            count
        });
        if with_joker {
            Self::from_count_map_with_joker(count_map)
        } else {
            Self::from_count_map(count_map)
        }
    }

    fn from_count_map(count_map: HashMap<i64, u32>) -> Result<Self> {
        Ok(match count_map.len() {
            1 => Self::FiveOfKind,
            2 => {
                if count_map.values().any(|&v| v == 4) {
                    Self::FourOfKind
                } else {
                    Self::FullHouse
                }
            }
            3 => {
                if count_map.values().any(|&v| v == 3) {
                    Self::ThreeOfKind
                } else {
                    Self::TwoPair
                }
            }
            4 => Self::Pair,
            5 => Self::HighCard,
            _ => eyre::bail!("unexpected count map len: {:?}", count_map),
        })
    }

    fn from_count_map_with_joker(mut count_map: HashMap<i64, u32>) -> Result<Self> {
        Ok(match count_map.len() {
            1 => Self::FiveOfKind,
            2 => {
                if count_map.keys().any(|&k| k == 0) {
                    Self::FiveOfKind
                } else if count_map.values().any(|&v| v == 4) {
                    Self::FourOfKind
                } else {
                    Self::FullHouse
                }
            }
            3 => {
                if let Some(j) = count_map.remove(&0) {
                    if j == 2 || j == 3 || count_map.values().any(|&v| v == 3) {
                        Self::FourOfKind
                    } else {
                        Self::FullHouse
                    }
                } else if count_map.values().any(|&v| v == 3) {
                    Self::ThreeOfKind
                } else {
                    Self::TwoPair
                }
            }
            4 => {
                if count_map.keys().any(|&k| k == 0) {
                    Self::ThreeOfKind
                } else {
                    Self::Pair
                }
            }
            5 => {
                if count_map.keys().any(|&k| k == 0) {
                    Self::Pair
                } else {
                    Self::HighCard
                }
            }
            _ => eyre::bail!("unexpected count map len: {:?}", count_map),
        })
    }
}

#[allow(clippy::type_complexity)]
fn parse_input(
    input: &str,
    with_joker: bool,
) -> Result<Vec<(Hand, (i64, i64, i64, i64, i64), i64)>> {
    input
        .lines()
        .map(|l| {
            let (cards, bid) = l.trim().split_once(' ').ok_or(eyre!("missing space"))?;
            let cards = cards
                .chars()
                .map(|c| match c {
                    'A' => Ok(14),
                    'K' => Ok(13),
                    'Q' => Ok(12),
                    'J' => Ok(if with_joker { 0 } else { 11 }),
                    'T' => Ok(10),
                    c if c != '1' && c != '0' => Ok(c.to_string().parse::<i64>()?),
                    _ => eyre::bail!("unknown card value: {}", c),
                })
                .collect::<Result<Vec<i64>>>()?;
            Ok((
                Hand::from_cards(&cards, with_joker)?,
                (cards[0], cards[1], cards[2], cards[3], cards[4]),
                bid.parse::<i64>()?,
            ))
        })
        .collect()
}

#[allow(clippy::type_complexity)]
fn get_total_winnings(mut hands: Vec<(Hand, (i64, i64, i64, i64, i64), i64)>) -> Result<String> {
    hands.sort_by(|l, r| {
        if l.0 != r.0 {
            l.0.cmp(&r.0)
        } else if l.1 .0 != r.1 .0 {
            l.1 .0.cmp(&r.1 .0)
        } else if l.1 .1 != r.1 .1 {
            l.1 .1.cmp(&r.1 .1)
        } else if l.1 .2 != r.1 .2 {
            l.1 .2.cmp(&r.1 .2)
        } else if l.1 .3 != r.1 .3 {
            l.1 .3.cmp(&r.1 .3)
        } else {
            l.1 .4.cmp(&r.1 .4)
        }
    });
    Ok(hands
        .iter()
        .enumerate()
        .map(|(i, (_, _, bid))| {
            let rank = 1 + i as i64;
            rank * bid
        })
        .sum::<i64>()
        .to_string())
}

pub fn solve_one(input: &str) -> Result<String> {
    get_total_winnings(parse_input(input, false)?)
}

pub fn solve_two(input: &str) -> Result<String> {
    get_total_winnings(parse_input(input, true)?)
}
