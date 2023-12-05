use std::{str::FromStr, num::ParseIntError};

use anyhow::{Context, Result, bail};

struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Vec<Map>>,
}

impl FromStr for Almanac {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Almanac> {
        let mut lines = s.lines();

        let (_, seeds) = lines
            .next()
            .and_then(|line| line.split_once(":"))
            .context("Parsing seeds line")?;
        let seeds = seeds
            .split_whitespace()
            .map(|seed| seed.parse::<usize>())
            .collect::<Result<Vec<usize>, ParseIntError>>()
            .context("Parsing seeds line elements")?;

        let mut map = Vec::new();
        let mut maps = Vec::new();

        if let Some(line) = lines.next() {
            if ! line.is_empty() {
                bail!("Expected empty line after seeds");
            }
        }

        while let Some(line) = lines.next() {
            if line.is_empty() {
                maps.push(map);
                map = Vec::new();
                continue;
            }

            if line.contains(':') {
                continue;
            }

            map.push(line.parse()?);
        }

        Ok(Almanac { seeds, maps })
    }
}

struct Almanac2 {
    seed_ranges: Vec<(usize, usize)>,
    maps: Vec<Vec<Map>>,
}

impl TryFrom<Almanac> for Almanac2 {
    type Error = anyhow::Error;

    fn try_from(value: Almanac) -> Result<Almanac2> {
        let pairs = value.seeds.chunks_exact(2);

        if ! pairs.remainder().is_empty() {
            bail!("Seeds has an odd number of values: {:?}", pairs.remainder());
        }

        let seed_ranges: Vec<_> =pairs.map(|pair| (pair[0], pair[1])).collect();

        Ok(Almanac2{seed_ranges, maps: value.maps})
    }

}


#[derive(Debug)]
struct Map {
    source: usize,
    destination: usize,
    size: usize,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Map> {
        let mut map = s.split_whitespace();

        let destination: usize = map.next().context("")?.parse().context("")?;
        let source: usize = map.next().context("")?.parse().context("")?;
        let size: usize = map.next().context("")?.parse().context("")?;

        if map.next().is_some() {
            anyhow::bail!("The row didn't contain 3 elements");
        }

        Ok(Map {
            source,
            destination,
            size,
        })
    }
}

#[cfg(feature = "problem_1")]
pub mod problem_1 {

    use anyhow::{Context, Result};
    use super::Almanac;

    pub(super) fn solve_almanac(almanac: Almanac) -> Result<usize> {
        almanac.seeds.into_iter().map(|seed| {
            almanac.maps.iter().fold(seed, |source, map| {
                let destin = map.iter().find_map(|map| {
                    if source >= map.source && source < (map.source+map.size) {
                        return Some(map.destination + (source - map.source));
                    }
                    None
                }).unwrap_or(source);
                destin
            })
        }).min().context("Finding minimum location")
    }


    pub fn solve(input: &str) -> Result<usize> {
        let almanac: Almanac = input.parse()?;
        solve_almanac(almanac)
    }
}

#[cfg(feature = "problem_2")]
pub mod problem_2 {

    use anyhow::{Result, Context};
    use super::{Almanac, Almanac2, Map};

    fn next(source:(usize, usize), map: &Vec<Map>) -> Vec<(usize, usize)> {
        let mut dest = vec![];
        let mut current = source.0;
        let end = source.1;
        'sources: loop {
            if current > end {
                break;
            }

            let mut min_start_range = None;
            for map in map {
                if current >= map.source {
                    if current < map.source+map.size {
                        let range_end = std::cmp::min(end, map.source+map.size-1);
                        dest.push((
                            map.destination + (current - map.source) , 
                            map.destination + (range_end - map.source),
                        ));
                        current = range_end+1;
                        continue 'sources;
                    } 
                }

                if map.source > current {
                    min_start_range = match min_start_range {
                        Some(value) => Some(std::cmp::min(value, map.source)),
                        None => Some(map.source),
                    };
                }
            }


            if let Some(min_start) = min_start_range {
                dest.push((current, min_start-1));
                current = min_start;
            } else {
                dest.push((current, end));
                break;
            }
        }

        dest
    }

    pub(super) fn solve_almanac(almanac: Almanac2) -> Result<usize> {
        almanac.seed_ranges.into_iter().map(|(start, size)| {
            almanac.maps.iter().fold(vec![(start, start+size-1)], |source, map| {
                source.into_iter().map(|(start, end)| next((start, end), map)).flatten().collect()
            }).into_iter().map(|(f, _)| f).min()
        }).flatten().min().context("Finding minimum location")
    }

    pub fn solve(input: &str) -> Result<usize> {
        let almanac: Almanac = input.parse().context("Parsing input")?;
        let almanac: Almanac2 = almanac.try_into().context("Converting for almanac2")?;
        solve_almanac(almanac)
    }
}

#[cfg(test)]
mod test {

    #[allow(unused_imports)]
    use std::error::Error;
    #[allow(unused_imports)]
    use std::fs::read_to_string;

    #[cfg(feature = "problem_1")]
    const P1_TRAIN_SOLUTION: usize = 35;

    #[cfg(feature = "problem_2")]
    const P2_TRAIN_SOLUTION: usize = 46;

    #[test]
    #[cfg(feature = "problem_1")]
    fn train_problem_1() -> Result<(), Box<dyn Error>> {
        use super::problem_1::solve;
        let input = read_to_string("resources/day_05/train_problem_1.inp")?;
        let result = solve(&input)?;
        assert_eq!(result, P1_TRAIN_SOLUTION);
        Ok(())
    }

    #[test]
    #[cfg(feature = "problem_1")]
    fn solve_problem_1() -> Result<(), Box<dyn Error>> {
        use super::problem_1::solve;
        let input = read_to_string("resources/day_05/problem_1.inp")?;
        let result = solve(&input)?;
        println!("{result}");
        Ok(())
    }

    #[test]
    #[cfg(feature = "problem_2")]
    fn train_problem_2() -> Result<(), Box<dyn Error>> {
        use super::problem_2::solve;
        let input = read_to_string("resources/day_05/train_problem_2.inp")?;
        let result = solve(&input)?;
        assert_eq!(result, P2_TRAIN_SOLUTION);
        Ok(())
    }

    #[test]
    #[cfg(feature = "problem_2")]
    fn solve_problem_2() -> Result<(), Box<dyn Error>> {
        use super::problem_2::solve;
        let input = read_to_string("resources/day_05/problem_2.inp")?;
        let result = solve(&input)?;
        println!("{result}");
        Ok(())
    }
}
