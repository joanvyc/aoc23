use std::{collections::HashMap, hash::Hash, str::FromStr, cmp::{Ordering, PartialOrd}, ops::Deref};
use anyhow::{Context, bail};

trait Card: Ord + PartialEq + Eq + Hash {
    fn from_char(c: char) -> Self;
    fn is_wildcard(&self) -> bool;
}

#[derive(Debug, Ord, PartialEq, Eq, Hash)]
struct Card1(char);

impl Deref for Card1 {
    type Target = char;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialOrd for Card1 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        const HAND_STRING_ORD: [char; 13] = [
            'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
        ];

        let s = HAND_STRING_ORD.iter().position(|&c| c == **self).unwrap();
        let o = HAND_STRING_ORD.iter().position(|&c| c == **other).unwrap();

        s.partial_cmp(&o)
    } 
}

impl Card for Card1 {
    fn from_char(c: char) -> Self {
        Self(c)
    }

    fn is_wildcard(&self) -> bool {
        false
    }
}

#[derive(Debug, Ord, PartialEq, Eq, Hash)]
struct Card2(char);

impl Deref for Card2 {
    type Target = char;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialOrd for Card2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        const HAND_STRING_ORD: [char; 13] = [
            'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J', 
        ];

        let s = HAND_STRING_ORD.iter().position(|&c| c == **self).unwrap();
        let o = HAND_STRING_ORD.iter().position(|&c| c == **other).unwrap();

        s.partial_cmp(&o)
    } 

}

impl Card for Card2 {
    fn from_char(c: char) -> Self {
        Self(c)
    }

    fn is_wildcard(&self) -> bool {
        **self == 'J'
    }
}

#[derive(Debug, Ord, PartialEq, Eq)]
struct Hand<C: Card> {
    cards: Vec<C>,
    bid: usize,
}

impl<C> FromStr for Hand<C> 
where
    C: Card
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (cards, bid) = s.split_once(' ').context("Spliting round")?;

        if cards.len() != 5 {
            bail!("Hand ought to have 5 cards")
        }

        let cards: Vec<C> = cards.chars().map(|c| C::from_char(c)).collect();
        let bid: usize = bid.parse()?;

        Ok(Self{cards, bid})
    }
}

impl<C> PartialOrd for Hand<C> 
where
    C: PartialOrd + PartialEq + Card
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_type: HandType = self.try_into().ok()?;
        let other_type: HandType = other.try_into().ok()?;

        Some(self_type.partial_cmp(&other_type)?
            .then(self.cards.partial_cmp(&other.cards)?))
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    Five,
    Four,
    Full,
    Three,
    Two,
    One,
    High,
}


impl<C: Card> TryFrom<&Hand<C>> for HandType 
{
    type Error = anyhow::Error;

    fn try_from(value: &Hand<C>) -> anyhow::Result<Self> {

        use HandType::*;

        let groups = value.cards.iter().fold(HashMap::new(), |mut groups, card| {
            *groups.entry(card).or_insert(0usize) += 1;
            groups
        });

        let mut groups = groups.iter().collect::<Vec<_>>();
        let wildcards: usize = groups.iter().find_map(|(card, reps)| {
            card.is_wildcard().then_some(**reps)
        }).unwrap_or(0usize);

        if wildcards == 5usize {
            return Ok(Five);
        }

        groups.retain(|&card| !card.0.is_wildcard());
        groups.sort_by(|a, b| b.1.cmp(a.1).then(b.0.cmp(a.0)));
        let mut groups: Vec<usize> = groups.iter().map(|(_, reps)| **reps).collect();
        if let Some(largest) = groups.get_mut(0) {
            *largest += wildcards;
        } else {
            bail!("adding wildcard");
        }

        let mut hand_type = None;

        for reps in groups {
            match reps {
                5 => hand_type = Some(Five),
                4 => hand_type = Some(Four),
                3 => hand_type = Some(Three),
                2 => {
                    hand_type = match hand_type {
                        Some(Three) => Some(Full),
                        Some(One)   => Some(Two),
                        None => Some(One),
                        _ => unreachable!("Others are guarded by hand max size (5)"),

                    }
                },
                1 => {
                    hand_type = match hand_type {
                        None => Some(High),
                        _ => break,
                    }
                },
                _ => unreachable!("No more than 5 cards should be counted")
            }
        }

        hand_type.context("No cards found")
    }
}

fn solve<C: Card>(hands: Vec<Hand<C>>) -> anyhow::Result<usize> {
    let mut game = hands;
    game.sort();

    let sum = game.into_iter().rev().enumerate().fold(0, |sum, (i, hand)| {
        sum + (i+1)*hand.bid
    });

    Ok(sum)
}

pub mod problem_1 {

    use super::{Hand, Card1};
    use anyhow::{Result, Context};

    pub fn solve(input: &str) -> Result<usize> {
        let game = input.lines().map(|hand| hand.parse::<Hand<Card1>>())
        .collect::<Result<Vec<_>>>().context("Parsing hands")?;

        super::solve(game)
    }
}

#[cfg(feature = "problem_2")]
pub mod problem_2 {

    use super::{Hand, Card2};
    use anyhow::{Context, Result};

    pub fn solve(input: &str) -> Result<usize> {
        let game = input.lines().map(|hand| hand.parse::<Hand<Card2>>())
            .collect::<Result<Vec<_>>>().context("Parsing hands")?;

        super::solve(game)
    }
}

#[cfg(test)]
mod test {

    #[allow(unused_imports)]
    use std::error::Error;
    #[allow(unused_imports)]
    use std::fs::read_to_string;
    #[cfg(feature = "problem_1")]
    const P1_TRAIN_SOLUTION: usize = 6440;

    #[cfg(feature = "problem_2")]
    const P2_TRAIN_SOLUTION: usize = 5905;

    #[test]
    #[cfg(feature = "problem_1")]
    fn train_problem_1() -> Result<(), Box<dyn Error>> {
        use super::problem_1::solve;
        let input = read_to_string("resources/day_07/train_problem_1.inp")?;
        let result = solve(&input)?;
        assert_eq!(result, P1_TRAIN_SOLUTION);
        Ok(())
    }

    #[test]
    #[cfg(feature = "problem_1")]
    fn solve_problem_1() -> Result<(), Box<dyn Error>> {
        use super::problem_1::solve;
        let input = read_to_string("resources/day_07/problem_1.inp")?;
        let result = solve(&input)?;
        println!("{result}");
        Ok(())
    }

    #[test]
    #[cfg(feature = "problem_2")]
    fn train_problem_2() -> Result<(), Box<dyn Error>> {
        use super::problem_2::solve;
        let input = read_to_string("resources/day_07/train_problem_2.inp")?;
        let result = solve(&input)?;
        assert_eq!(result, P2_TRAIN_SOLUTION);
        Ok(())
    }

    #[test]
    #[cfg(feature = "problem_2")]
    fn solve_problem_2() -> Result<(), Box<dyn Error>> {
        use super::problem_2::solve;
        let input = read_to_string("resources/day_07/problem_2.inp")?;
        let result = solve(&input)?;
        println!("{result}");
        Ok(())
    }
}
