use anyhow::{anyhow, Context};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
enum Indication {
    Right,
    Left,
}

impl TryFrom<char> for Indication {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'R' => Ok(Indication::Right),
            'L' => Ok(Indication::Left),
            _ => Err(anyhow!("Unknown direction tag")),
        }
    }
}

#[derive(Clone)]
struct Directions(HashMap<String, Direction>);

impl Deref for Directions {
    type Target = HashMap<String, Direction>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Directions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Directions {
    fn new() -> Directions {
        Directions(HashMap::new())
    }

    fn add_direction(&mut self, from: &str, right: &str, left: &str) {
        self.insert(
            from.to_string(),
            Direction {
                right: right.to_string(),
                left: left.to_string(),
            },
        );
    }

    fn next(&self, indication: &Indication, from: &str) -> anyhow::Result<&str> {
        Ok(self.0.get(from).context("getting directions")?.to(indication))
    }
}

#[derive(Clone)]
struct Direction {
    right: String,
    left: String,
}

impl Direction {
    fn to(&self, indication: &Indication) -> &str {
        match indication {
            Indication::Right => &self.right,
            Indication::Left => &self.left,
        }
    }
}

pub mod problem_1 {

    use super::{Directions, Indication};
    use anyhow::{Context, Result};
    use regex::Regex;

    pub fn solve(input: &str) -> Result<usize> {
        let mut map = input.lines();
        let indications = map
            .next()
            .context("Getting first line")?
            .chars()
            .map(|c| c.try_into())
            .collect::<Result<Vec<Indication>>>()
            .context("Parsing indications")?;
        let map = map.skip(1);

        let parse = Regex::new(r#"(?<from>[A-Z]{3}) = \((?<left>[A-Z]{3}), (?<right>[A-Z]{3})\)"#)?;

        let directions = map.fold(Directions::new(), |mut dir, direction| {
            let caps = parse.captures(direction).unwrap();
            dir.add_direction(&caps["from"], &caps["right"], &caps["left"]);
            dir
        });

        let steps = indications
            .iter()
            .cycle()
            .scan("AAA", |from, indication| {
                (from == &"ZZZ").then(|| {
                    *from = directions.next(indication, from).unwrap();
                    Some(())
                })
            })
            .count();

        Ok(steps)
    }
}

#[cfg(feature = "problem_2")]
pub mod problem_2 {


    use super::{Directions, Indication};
    use anyhow::{Context, Result};
    use regex::Regex;
    use num::Integer;

    pub fn solve(input: &str) -> Result<usize> {
        let mut map = input.lines();
        let indications = map
            .next()
            .context("Getting first line")?
            .chars()
            .map(|c| c.try_into())
            .collect::<Result<Vec<Indication>>>()
            .context("Parsing indications")?;
        let map = map.skip(1);

        let parse = Regex::new(r#"(?<from>[A-Z0-9]{3}) = \((?<left>[A-Z0-9]{3}), (?<right>[A-Z0-9]{3})\)"#)?;

        let directions = map.fold(Directions::new(), |mut dir, direction| {
            let caps = parse.captures(direction).unwrap();
            dir.add_direction(&caps["from"], &caps["right"], &caps["left"]);
            dir
        });

        let starts: Vec<String> = directions.keys().filter(|start| start.chars().last() == Some('A')).map(|start| {
            start.to_string()
        }).collect();

        let mut cycles = vec![];
        for start in starts {
            let mut indications = indications.iter().cycle();
            let state = indications.clone().scan((0, start), |state, indication| {
                if state.1.chars().last() == Some('Z') {
                    return None;
                }
                *state = (state.0 + 1, directions.next(indication, &state.1).unwrap().to_string());
                Some(state.clone())
            }).last();

            println!{"{state:?}"};
            let (offset, position) = state.unwrap();

            let start_cycle = directions.next(indications.next().unwrap(), &position).unwrap();

            let state = indications.skip(offset).scan((0, start_cycle), |state, indication| {
                if state.1.chars().last() == Some('Z') {
                    return None;
                }
                *state = (state.0 + 1, directions.next(indication, &state.1).unwrap());
                Some(state.clone())
            }).last();

            let offset = (offset, position);
            println!{"Offset: {offset:?}, Cycle: {state:?}"};

            if let Some((cycle, _)) = state {
                cycles.push(cycle);
            } else {
                panic!("AAAAA");
            }

            4974466197329281024
        }

        cycles.into_iter().reduce(|lcm: usize, c| lcm.lcm(&c)).context("")


//        let steps = indications
//            .iter()
//            .cycle()
//            .scan(starts, |from, indication| {
//                let zcount = from.iter().filter(|position| position.chars().last() == Some('Z')).count();
//                if zcount > 2 {
//                    println!("{}/{}: {from:?}", zcount, from.len()); 
//                }
//                (!from.iter().all(|position| position.chars().last() == Some('Z'))).then(|| {
//                    from.iter_mut().for_each(|from| {
//                       *from = directions.next(indication, &from).unwrap().to_string();
//                    });
//                    Some(())
//                })
//            })
//            .count();

        //Ok(0)
    }
}

#[cfg(test)]
mod test {

    #[allow(unused_imports)]
    use std::error::Error;
    #[allow(unused_imports)]
    use std::fs::read_to_string;
    #[cfg(feature = "problem_1")]
    const P1_TRAIN_1_SOLUTION: usize = 2;
    const P1_TRAIN_2_SOLUTION: usize = 6;

    #[cfg(feature = "problem_2")]
    const P2_TRAIN_SOLUTION: usize = 6;

    #[test]
    #[cfg(feature = "problem_1")]
    fn train_problem_1_1() -> Result<(), Box<dyn Error>> {
        use super::problem_1::solve;
        let input = read_to_string("resources/day_08/train_problem_1_1.inp")?;
        let result = solve(&input)?;
        assert_eq!(result, P1_TRAIN_1_SOLUTION);
        Ok(())
    }

    #[test]
    #[cfg(feature = "problem_1")]
    fn train_problem_1_2() -> Result<(), Box<dyn Error>> {
        use super::problem_1::solve;
        let input = read_to_string("resources/day_08/train_problem_1_2.inp")?;
        let result = solve(&input)?;
        assert_eq!(result, P1_TRAIN_2_SOLUTION);
        Ok(())
    }

    #[test]
    #[cfg(feature = "problem_1")]
    fn solve_problem_1() -> Result<(), Box<dyn Error>> {
        use super::problem_1::solve;
        let input = read_to_string("resources/day_08/problem_1.inp")?;
        let result = solve(&input)?;
        println!("{result}");
        Ok(())
    }

    #[test]
    #[cfg(feature = "problem_2")]
    fn train_problem_2() -> Result<(), Box<dyn Error>> {
        use super::problem_2::solve;
        let input = read_to_string("resources/day_08/train_problem_2.inp")?;
        let result = solve(&input)?;
        assert_eq!(result, P2_TRAIN_SOLUTION);
        Ok(())
    }

    #[test]
    #[cfg(feature = "problem_2")]
    fn solve_problem_2() -> Result<(), Box<dyn Error>> {
        use super::problem_2::solve;
        let input = read_to_string("resources/day_08/problem_2.inp")?;
        let result = solve(&input)?;
        println!("{result}");
        Ok(())
    }
}
