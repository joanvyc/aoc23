use std::str::FromStr;

use anyhow::Context;

#[derive(Debug)]
struct Race {
    duration: usize,
    record: usize,
}

struct Races {
    races: Vec<Race>,
}

impl FromStr for Race {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut lines = s.lines();
        let duration = lines
            .next()
            .context("Reading time")?
            .split_once(':')
            .context("Spliting time")?
            .1
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .parse::<usize>()
            .context("Parsing time")?;

        let record = lines
            .next()
            .context("Reading record")?
            .split_once(':')
            .context("Spliting record")?
            .1
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .parse::<usize>()
            .context("Parsing record")?;

        Ok(Race { duration, record })
    }
}

impl FromStr for Races {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut lines = s.lines();

        let duration = lines
            .next()
            .context("Reading times")?
            .split_once(':')
            .context("Spliting times")?
            .1
            .split_whitespace()
            .map(|time| time.parse::<usize>().unwrap());
        let records = lines
            .next()
            .context("Reading records")?
            .split_once(':')
            .context("Spliting records")?
            .1
            .split_whitespace()
            .map(|time| time.parse::<usize>().unwrap());

        Ok(Races {
            races: duration
                .zip(records)
                .map(|(duration, record)| Race { duration, record })
                .collect(),
        })
    }
}

fn find_possible_solutions(race: Race) -> usize {
    // distance(h, d) = h*(d-h) = h^2 - hd - r
    //     -d +/- sqrt(d^2 -4*1*r)
    // h = -----------------------
    //               2
    let d = race.duration as f64;
    let r = race.record as f64;

    let sq: f64 = d * d - 4f64 * r;
    let sq = sq.sqrt();

    let lower: f64 = (d - sq) / 2f64;
    let upper: f64 = (d + sq) / 2f64;

    let lower = (lower + 1f64).floor() as usize;
    let upper = (upper - 1f64).ceil() as usize;

    1 + (upper - lower)
}

#[cfg(feature = "problem_1")]
pub mod problem_1 {

    use super::Races;
    use anyhow::{Context, Result};

    pub fn solve(input: &str) -> Result<usize> {
        let races: Races = input.parse().context("Parsing input")?;

        let prod = races
            .races
            .into_iter()
            .map(super::find_possible_solutions)
            .product();

        Ok(prod)
    }
}

#[cfg(feature = "problem_2")]
pub mod problem_2 {

    use super::Race;
    use anyhow::{Context, Result};

    pub fn solve(input: &str) -> Result<usize> {
        let race: Race = input.parse().context("Parsing input")?;

        let result = super::find_possible_solutions(race);

        Ok(result)
    }
}

#[cfg(test)]
mod test {

    #[allow(unused_imports)]
    use std::error::Error;
    #[allow(unused_imports)]
    use std::fs::read_to_string;

    #[cfg(feature = "problem_1")]
    const P1_TRAIN_SOLUTION: usize = 288;

    #[cfg(feature = "problem_2")]
    const P2_TRAIN_SOLUTION: usize = 71503;

    #[test]
    #[cfg(feature = "problem_1")]
    fn train_problem_1() -> Result<(), Box<dyn Error>> {
        use super::problem_1::solve;
        let input = read_to_string("resources/day_06/train_problem_1.inp")?;
        let result = solve(&input)?;
        assert_eq!(result, P1_TRAIN_SOLUTION);
        Ok(())
    }

    #[test]
    #[cfg(feature = "problem_1")]
    fn solve_problem_1() -> Result<(), Box<dyn Error>> {
        use super::problem_1::solve;
        let input = read_to_string("resources/day_06/problem_1.inp")?;
        let result = solve(&input)?;
        println!("{result}");
        Ok(())
    }

    #[test]
    #[cfg(feature = "problem_2")]
    fn train_problem_2() -> Result<(), Box<dyn Error>> {
        use super::problem_2::solve;
        let input = read_to_string("resources/day_06/train_problem_1.inp")?;
        let result = solve(&input)?;
        assert_eq!(result, P2_TRAIN_SOLUTION);
        Ok(())
    }

    #[test]
    #[cfg(feature = "problem_2")]
    fn solve_problem_2() -> Result<(), Box<dyn Error>> {
        use super::problem_2::solve;
        let input = read_to_string("resources/day_06/problem_1.inp")?;
        let result = solve(&input)?;
        println!("{result}");
        Ok(())
    }
}
