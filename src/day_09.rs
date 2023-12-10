pub mod problem_1 {

    use anyhow::{Context, Result};

    fn construct_deltas(history: Vec<isize>) -> Result<Vec<isize>> {
        let mut deltas = Vec::new();
        deltas.push(history);

        loop {
            let base = deltas.last().unwrap();

            if base.iter().all(|&value| value == 0) {
                break;
            }

            deltas.push(base.windows(2).map(|a| a[1] - a[0]).collect::<Vec<_>>());
        }

        deltas
            .into_iter()
            .map(|deltas| {
                deltas
                    .last()
                    .map(|value| value.clone())
                    .context("level is empty")
            })
            .collect::<Result<Vec<_>>>()
    }

    pub fn solve(input: &str) -> Result<isize> {
        let histories = input
            .lines()
            .map(|history| {
                history
                    .split_whitespace()
                    .map(|value| value.parse().context("parsing value"))
                    .collect::<Result<Vec<isize>>>()
            })
            .collect::<Result<Vec<Vec<isize>>>>()?;

        let result = histories
            .into_iter()
            .map(|history| Ok(construct_deltas(history)?.into_iter().rev().sum::<isize>()))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .sum();
        Ok(result)
    }
}

#[cfg(feature = "problem_2")]
pub mod problem_2 {

    use anyhow::Result;

    pub fn solve(input: &str) -> Result<isize> {
        let histories = input
            .lines()
            .map(|history| {
                history
                    .split_whitespace()
                    .map(|value| value.parse().unwrap())
                    .collect::<Vec<isize>>()
            })
            .collect::<Vec<Vec<isize>>>();
        let histories: isize = histories
            .into_iter()
            .map(|h| {
                let mut deltas: Vec<Vec<isize>> = Vec::new();
                deltas.push(h);

                loop {
                    let base = deltas.last().unwrap();
                    if base.iter().all(|&a| a == 0) {
                        break;
                    }
                    deltas.push(base.windows(2).map(|s| s[1] - s[0]).collect::<Vec<_>>());
                }

                deltas
                    .into_iter()
                    .map(|f| f.first().unwrap().clone())
                    .collect::<Vec<_>>()
            })
            .map(|e| e.into_iter().rev().fold(0, |r, v| v - r))
            .sum();

        Ok(histories)
    }
}

#[cfg(test)]
mod test {

    #[allow(unused_imports)]
    use std::error::Error;
    #[allow(unused_imports)]
    use std::fs::read_to_string;
    #[cfg(feature = "problem_1")]
    const P1_TRAIN_SOLUTION: isize = 114;

    #[cfg(feature = "problem_2")]
    const P2_TRAIN_SOLUTION: isize = 2;

    #[test]
    #[cfg(feature = "problem_1")]
    fn train_problem_1() -> Result<(), Box<dyn Error>> {
        use super::problem_1::solve;
        let input = read_to_string("resources/day_09/train_problem_1.inp")?;
        let result = solve(&input)?;
        assert_eq!(result, P1_TRAIN_SOLUTION);
        Ok(())
    }

    #[test]
    #[cfg(feature = "problem_1")]
    fn solve_problem_1() -> Result<(), Box<dyn Error>> {
        use super::problem_1::solve;
        let input = read_to_string("resources/day_09/problem_1.inp")?;
        let result = solve(&input)?;
        println!("{result}");
        Ok(())
    }

    #[test]
    #[cfg(feature = "problem_2")]
    fn train_problem_2() -> Result<(), Box<dyn Error>> {
        use super::problem_2::solve;
        let input = read_to_string("resources/day_09/train_problem_2.inp")?;
        let result = solve(&input)?;
        assert_eq!(result, P2_TRAIN_SOLUTION);
        Ok(())
    }

    #[test]
    #[cfg(feature = "problem_2")]
    fn solve_problem_2() -> Result<(), Box<dyn Error>> {
        use super::problem_2::solve;
        let input = read_to_string("resources/day_09/problem_2.inp")?;
        let result = solve(&input)?;
        println!("{result}");
        Ok(())
    }
}
