use std::{
    cmp::Ordering, collections::HashMap,
    ops::Deref, str::FromStr, mem::discriminant,
};

use anyhow::Context;

#[derive(Ord, PartialEq, Eq, Debug)]
struct Card(char);

impl Deref for Card {
    type Target = char;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        const HAND_STRING_ORD: [char; 13] = [
            'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
        ];

        let s = HAND_STRING_ORD.iter().position(|&c| c == **self).unwrap();
        let o = HAND_STRING_ORD.iter().position(|&c| c == **other).unwrap();

        o.partial_cmp(&s)
    }
}

#[derive(Ord, PartialEq, Eq, Debug)]
struct HandString(String);

impl Deref for HandString {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialOrd for HandString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for (s, o) in self.0.chars().zip(other.0.chars()) {
            match Card(s).partial_cmp(&Card(o)) {
                Some(Ordering::Equal) => (),
                ord => return ord,
            }
        }

        Some(Ordering::Equal)
    }
}

#[derive(Ord, PartialEq, Eq, Debug)]
struct HandPlay {
    hand: (Hand, HandString),
    bid: usize,
}

impl FromStr for HandPlay {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').context("Spliting entry")?;
        let bid: usize = bid.parse()?;

        Ok(HandPlay {
            hand: (
                hand.parse().context("Parsing hand")?,
                HandString(hand.to_string()),
            ),
            bid,
        })
    }
}

impl PartialOrd for HandPlay {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.hand.0.partial_cmp(&other.hand.0)?
                .then(self.hand.1.partial_cmp(&other.hand.1)?)
        )
    }
}

#[derive(Ord, PartialEq, Eq, Debug)]
enum Hand {
    Five(Card),
    Four(Card),
    Full((Card, Card)),
    Three(Card),
    TwoPair((Card, Card)),
    OnePair(Card),
    HighCard(Card),
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s.chars().fold(HashMap::new(), |mut map, card| {
            *map.entry(card).or_insert(0usize) += 1;
            map
        });

        let mut cards = cards.iter().collect::<Vec<_>>();
        cards.sort_by(|a, b| b.1.cmp(a.1).then(b.0.cmp(a.0)));

        let mut last = None;
        for card in cards {
            match card {
                (&card, 5) => return Ok(Hand::Five(Card(card))),
                (&card, 4) => return Ok(Hand::Four(Card(card))),
                (&card, 3) => last = Some(Hand::Three(Card(card))),
                (&card, 2) => match last {
                    Some(Hand::Three(c)) => return Ok(Hand::Full((c, Card(card)))),
                    Some(Hand::OnePair(c)) => return Ok(Hand::TwoPair((c, Card(card)))),
                    None => last = Some(Hand::OnePair(Card(card))),
                    _ => unreachable!("No other combination possbile bc ordering and max 5 cards"),
                },
                (&card, 1) => match last {
                    None => last = Some(Hand::HighCard(Card(card))),
                    Some(Hand::HighCard(c)) if Card(card) > c => {
                        last = Some(Hand::HighCard(Card(card)))
                    }
                    _ => return last.context("Impossible, guarded by match"),
                },
                _ => unreachable!(""),
            }
        }

        last.context("No card found")
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Hand::*;
        match (self, other) {
            (s, o) if discriminant(s) == discriminant(o) => Some(Ordering::Equal),
            (Five(_), _) => Some(Ordering::Greater),
            (_, Five(_)) => Some(Ordering::Less),

            (Four(_), _) => Some(Ordering::Greater),
            (_, Four(_)) => Some(Ordering::Less),

            (Full(_), _) => Some(Ordering::Greater),
            (_, Full(_)) => Some(Ordering::Less),

            (Three(_), _) => Some(Ordering::Greater),
            (_, Three(_)) => Some(Ordering::Less),

            (TwoPair(_), _) => Some(Ordering::Greater),
            (_, TwoPair(_)) => Some(Ordering::Less),

            (OnePair(_), _) => Some(Ordering::Greater),
            (_, OnePair(_)) => Some(Ordering::Less),

            (HighCard(_), _) => Some(Ordering::Greater),
        }
    }
}

#[cfg(feature = "problem_1")]
pub mod problem_1 {

    use super::{Hand, HandPlay};
    use anyhow::{Context, Result};

    pub fn solve(input: &str) -> Result<usize> {
        let mut game = input
            .lines()
            .map(|round| round.parse::<HandPlay>())
            .collect::<Result<Vec<_>>>()
            .context("Parsing the rounds")?;
        game.sort();

        let sum = game.into_iter().enumerate();
        let sum = sum.fold(0, |sum, (i, hand)| {
            sum + (i + 1) * hand.bid
        });

        Ok(sum)
    }
}

#[cfg(feature = "problem_2")]
pub mod problem_2 {

    use super::Hand;
    use anyhow::{Context, Result};

    pub fn solve(input: &str) -> Result<usize> {}
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
    const P2_TRAIN_SOLUTION: usize = 71503;

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
