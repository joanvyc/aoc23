use std::{collections::HashSet, str::FromStr};

use anyhow::Context;
use nom::{
    bytes::complete::{tag, take_while},
    sequence::{delimited, tuple},
};

#[derive(Debug)]
struct Card {
    numbers: HashSet<usize>,
    winning_numbers: HashSet<usize>,
    copies: usize,
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parse_card_id = delimited(
            tuple((
                tag::<_, _, nom::error::Error<_>>("Card"),
                take_while(|c: char| c.is_whitespace()),
            )),
            take_while(|c: char| c.is_ascii_digit()),
            tag(":"),
        );
        let (numbers, card) = parse_card_id(s).unwrap();
        let _ = card.parse::<usize>()?;

        let (winning, mine) = numbers
            .split_once('|')
            .context("Every line must have one |")?;
        let winning_numbers: HashSet<usize> = winning
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        let numbers: HashSet<usize> = mine
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        Ok(Card {
            numbers,
            winning_numbers,
            copies: 1,
        })
    }
}

pub mod problem_1 {
    use super::Card;
    use anyhow::Result;
    use std::collections::HashSet;

    pub fn solve(input: &str) -> Result<usize> {
        Ok(input
            .lines()
            .map(|line| line.parse::<Card>().unwrap())
            .map(|card| {
                let wins = card
                    .numbers
                    .intersection(&card.winning_numbers)
                    .collect::<HashSet<_>>()
                    .len();

                if wins > 0 {
                    2usize.pow((wins as u32) - 1)
                } else {
                    0
                }
            })
            .sum())
    }
}

pub mod problem_2 {
    use super::Card;
    use anyhow::Result;
    use std::collections::HashSet;

    pub fn solve(input: &str) -> Result<usize> {
        let mut cards: Vec<Card> = input
            .lines()
            .map(|line| line.parse::<Card>().unwrap())
            .collect();

        for card_i in 0..cards.len() {
            let wins = cards[card_i]
                .numbers
                .intersection(&cards[card_i].winning_numbers)
                .collect::<HashSet<_>>()
                .len();

            let copy_i_s = card_i + 1;
            let copy_i_e = copy_i_s + wins;
            for copy_i in copy_i_s..copy_i_e {
                cards[copy_i].copies += cards[card_i].copies;
            }
        }

        Ok(cards.into_iter().fold(0, |sum, card| sum + card.copies))
    }
}

#[cfg(test)]
mod test {

    use std::error::Error;
    use std::fs::read_to_string;

    const P1_TRAIN_SOLUTION: usize = 13;
    const P2_TRAIN_SOLUTION: usize = 30;

    #[test]
    fn train_problem_1() -> Result<(), Box<dyn Error>> {
        use super::problem_1::solve;
        let input = read_to_string("resources/day_04/train_problem_1.inp")?;
        let result = solve(&input)?;
        assert_eq!(result, P1_TRAIN_SOLUTION);
        Ok(())
    }

    #[test]
    fn solve_problem_1() -> Result<(), Box<dyn Error>> {
        use super::problem_1::solve;
        let input = read_to_string("resources/day_04/problem_1.inp")?;
        let result = solve(&input)?;
        println!("{result}");
        Ok(())
    }

    #[test]
    fn train_problem_2() -> Result<(), Box<dyn Error>> {
        use super::problem_2::solve;
        let input = read_to_string("resources/day_04/train_problem_2.inp")?;
        let result = solve(&input)?;
        assert_eq!(result, P2_TRAIN_SOLUTION);
        Ok(())
    }

    #[test]
    fn solve_problem_2() -> Result<(), Box<dyn Error>> {
        use super::problem_2::solve;
        let input = read_to_string("resources/day_04/problem_2.inp")?;
        let result = solve(&input)?;
        println!("{result}");
        Ok(())
    }
}
