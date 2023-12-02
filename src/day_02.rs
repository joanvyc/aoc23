use anyhow::{bail, Context, Result};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{digit1, space1},
    multi::many0,
    sequence::{terminated, tuple},
};

#[derive(Default, Debug, Clone)]
struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

fn parse_game(game: &str) -> Result<Game> {
    let parse_game = tag::<_, _, nom::error::Error<_>>("Game ");
    // This forces game to outlive static because of the ? after the Result.
    // I would be nice to find a way to use ? without requiring game to outlive
    // parse_game.
    //
    // let (game, _) = parse_game(game).with_context(|| format!(r#"line should start with "Game""#))?;
    //
    // for that reason form now on we unwrap:
    let (game, _) = parse_game(game).unwrap();

    let mut parse_id = terminated(
        take_till::<_, _, nom::error::Error<_>>(|c| c == ':'),
        tag(":"),
    );
    let (game, id) = parse_id(game).unwrap();
    let id: usize = id.parse().context("id is not a number")?;

    let mut rounds = vec![];
    let mut parse_round = tuple((
        space1::<_, nom::error::Error<_>>,
        digit1,
        space1,
        alt((tag("red"), tag("green"), tag("blue"))),
        many0(alt((tag(","), tag(";")))),
    ));

    let mut round = Round::default();
    let mut game = game;

    loop {
        let (game_rest, (_, count, _, color, terminator)) = parse_round(game).unwrap();
        let count: usize = count.parse().context("count is not a number")?;
        match color {
            "red" => round.red = count,
            "green" => round.green = count,
            "blue" => round.blue = count,
            _ => bail!("Unknown color: {color}"),
        }

        match terminator.first() {
            Some(&",") => (),
            Some(&";") => {
                rounds.push(round);
                round = Round::default();
            }
            None => {
                rounds.push(round);
                break;
            }
            _ => unreachable!("guaranteed by parse_round"),
        }

        if game_rest.is_empty() {
            break;
        }

        game = game_rest;
    }

    Ok(Game { id, rounds })
}

pub mod problem_1 {

    use super::{parse_game, Round};
    use anyhow::{Context, Result};

    const COUNT_RED: usize = 12;
    const COUNT_GREEN: usize = 13;
    const COUNT_BLUE: usize = 14;

    pub fn solve(input: &str) -> Result<usize> {
        Ok(input
            .lines()
            .map(|line| {
                parse_game(line)
                    .context("could not parse the game")
                    .unwrap()
            })
            .filter_map(|game| {
                let rounds: Vec<Round> = game.rounds;
                if rounds.iter().any(|&Round { red, green, blue }| {
                    red > COUNT_RED || green > COUNT_GREEN || blue > COUNT_BLUE
                }) {
                    return None;
                }

                Some(game.id)
            })
            .sum())
    }
}

pub mod problem_2 {

    use super::{parse_game, Round};
    use anyhow::{Context, Result};

    pub fn solve(input: &str) -> Result<usize> {
        Ok(input
            .lines()
            .map(|line| {
                parse_game(line)
                    .context("could not parse the game")
                    .unwrap()
            })
            .map(|game| {
                let min = game
                    .rounds
                    .into_iter()
                    .reduce(|min, round| Round {
                        red: std::cmp::max(min.red, round.red),
                        green: std::cmp::max(min.green, round.green),
                        blue: std::cmp::max(min.blue, round.blue),
                    })
                    .unwrap();
                min.red * min.green * min.blue
            })
            .sum())
    }
}

#[cfg(test)]
mod test {

    use std::error::Error;
    use std::fs::read_to_string;

    #[test]
    fn train_problem_1() -> Result<(), Box<dyn Error>> {
        use super::problem_1::solve;
        let input = read_to_string("resources/day_02/train_problem_1.inp")?;
        let result = solve(&input)?;

        assert_eq!(result, 8);

        Ok(())
    }

    #[test]
    fn solve_problem_1() -> Result<(), Box<dyn Error>> {
        use super::problem_1::solve;
        let input = read_to_string("resources/day_02/problem_1.inp")?;
        let result = solve(&input)?;

        println!("{result}");

        Ok(())
    }

    #[test]
    fn train_problem_2() -> Result<(), Box<dyn Error>> {
        use super::problem_2::solve;
        let input = read_to_string("resources/day_02/train_problem_2.inp")?;
        let result = solve(&input)?;

        assert_eq!(result, 2286);

        Ok(())
    }

    #[test]
    fn solve_problem_2() -> Result<(), Box<dyn Error>> {
        use super::problem_2::solve;
        let input = read_to_string("resources/day_02/problem_2.inp")?;
        let result = solve(&input)?;

        println!("{result}");

        Ok(())
    }
}
