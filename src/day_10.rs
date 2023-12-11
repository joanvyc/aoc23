use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Deref,
};

use anyhow::{bail, Context, Result};

#[derive(Debug)]
enum Dir {
    East,
    West,
    North,
    South,
}

impl Dir {
    fn flip(&self) -> Dir {
        use Dir::*;
        match self {
            East => West,
            West => East,
            North => South,
            South => North,
        }
    }
}

#[derive(Debug)]
struct Tile(char);

impl Tile {
    fn new(tile: char) -> Result<Tile> {
        let possible = "|-LJ7F.S";
        possible
            .contains(tile)
            .then_some(Tile(tile))
            .context("Tile {tile} is not part of the dataset")
    }

    fn redirect(&self, from: &Dir) -> Result<Dir> {
        use Dir::*;
        // Flip from the sender perspective
        let from = from.flip();

        // Map for each tile type
        let to = match self.0 {
            '|' => match from {
                North | South => from.flip(),
                _ => bail!("Wrong redirect for {} from {from:?}", self.0),
            },
            '-' => match from {
                East | West => from.flip(),
                _ => bail!("Wrong redirect for {} from {from:?}", self.0),
            },
            'L' => match from {
                North => East,
                East => North,
                _ => bail!("Wrong redirect for {} from {from:?}", self.0),
            },
            'J' => match from {
                North => West,
                West => North,
                _ => bail!("Wrong redirect for {} from {from:?}", self.0),
            },
            '7' => match from {
                West => South,
                South => West,
                _ => bail!("Wrong redirect for {} from {from:?}", self.0),
            },
            'F' => match from {
                East => South,
                South => East,
                _ => bail!("Wrong redirect for {} from {from:?}", self.0),
            },
            '.' => bail!("Cant redirect empty tile"),
            'S' => bail!("Cant redirect starting point"),
            _ => unreachable!("Thiles with different value aborted at creation"),
        };
        Ok(to)
    }
}

impl Deref for Tile {
    type Target = char;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct Space {
    w: usize,
    h: usize,
    tiles: Vec<Vec<Tile>>,
}

impl Space {
    fn move_position(&self, position: &(usize, usize), dir: &Dir) -> Option<(usize, usize)> {
        use Dir::*;
        let moved = match dir {
            East => (position.0, position.1 + 1),
            West => (position.0, position.1.checked_sub(1)?),
            North => (position.0.checked_sub(1)?, position.1),
            South => (position.0 + 1, position.1),
        };

        if moved.0 >= self.h || moved.1 >= self.w {
            return None;
        }

        Some(moved)
    }

    fn get(&self, position: &(usize, usize)) -> Option<&Tile> {
        self.tiles
            .get(position.0)
            .and_then(|row| row.get(position.1))
    }
}

struct Input {
    start: (usize, usize),
    space: Space,
}

fn input_generator(input: &str) -> Result<Input> {
    let space: Result<Vec<Vec<Tile>>> = input
        .lines()
        .map(|row| {
            row.chars()
                .map(|tile| Tile::new(tile))
                .collect::<Result<Vec<Tile>>>()
        })
        .collect();
    let space = space?;
    let h = space.len();
    let w = space.first().unwrap().len();
    let space = Space { tiles: space, h, w };

    let mut start = None;
    'start_row: for (r, row) in space.tiles.iter().enumerate() {
        for (c, tile) in row.iter().enumerate() {
            if **tile == 'S' {
                start = Some((r, c));
                break 'start_row;
            }
        }
    }
    let start = start.context("Looking for start position")?;

    Ok(Input { start, space })
}

pub fn solve_1(input: &str) -> Result<usize> {
    use Dir::*;
    let input = input_generator(input)?;
    let space = input.space;

    let mut pos = vec![North, East, South, West]
        .into_iter()
        .map(|dir| {
            space
                .move_position(&input.start, &dir)
                .and_then(|next| Some((dir, next)))
        })
        .flatten()
        .filter(|(dir, position)| {
            space
                .get(position)
                .and_then(|tile| tile.redirect(dir).ok())
                .is_some()
        });

    let (mut dir, mut pos) = pos.next().unwrap();

    let mut steps: usize = 1;

    while let Some(tile) = space.get(&pos) {
        if let &Tile('S') = tile {
            break;
        }

        dir = tile
            .redirect(&dir)
            .context(format! {"At step {steps}"})
            .unwrap();
        pos = space.move_position(&pos, &dir).unwrap();
        steps += 1;
    }

    let furthest = steps.div_ceil(2);

    Ok(furthest)
}

pub fn solve_2(input: &str) -> Result<usize> {
    use Dir::*;
    let input = input_generator(input)?;
    let space = input.space;

    let mut pos = vec![North, East, South, West]
        .into_iter()
        .map(|dir| {
            space
                .move_position(&input.start, &dir)
                .and_then(|next| Some((dir, next)))
        })
        .flatten()
        .filter(|(dir, position)| {
            space
                .get(position)
                .and_then(|tile| tile.redirect(dir).ok())
                .is_some()
        });

    let (mut dir, mut pos) = pos.next().unwrap();

    let mut mask = HashSet::new();
    mask.insert(input.start);

    while let Some(tile) = space.get(&pos) {
        if let &Tile('S') = tile {
            break;
        }

        mask.insert(pos);

        dir = tile.redirect(&dir).unwrap();
        pos = space.move_position(&pos, &dir).unwrap();
    }

    let mut tile_inside = HashSet::new();
    let mut tile_outside = HashSet::new();
    for r in 0..space.h {
        'tiles: for c in 0..space.w {
            if tile_outside.contains(&(r, c))
                || tile_inside.contains(&(r, c))
                || mask.contains(&(r, c))
            {
                continue;
            }

            let mut neigbours = HashSet::new();
            let mut to_visit: VecDeque<(usize, usize)> = vec![(r, c)].into();

            while let Some(pos) = to_visit.pop_front() {

                if neigbours.contains(&pos) || mask.contains(&pos) {
                    continue;
                }

                neigbours.insert(pos);

                if tile_outside.contains(&pos) {
                    neigbours.iter().cloned().for_each(|position| {
                        tile_outside.insert(position);
                    });
                    continue 'tiles;
                }

                let next_positions = vec![North, East, South, West]
                    .into_iter()
                    .map(|dir| space.move_position(&pos, &dir))
                    .flatten()
                    .collect::<Vec<_>>();

//                next_positions.iter().cloned().for_each(|position| {
//                    neigbours.insert(position);
//                });
                next_positions.iter().cloned().for_each(|position| {
                    to_visit.push_back(position);
                });

                //println!("From {pos:?} we go to {next_positions:?}");
                // Means we have reached the edge, so this nodes are outside
                if tile_outside.contains(&pos) || next_positions.len() != 4 {
                    neigbours.iter().cloned().for_each(|position| {
                        tile_outside.insert(position);
                    });
                    continue 'tiles;
                }
            }

            // If we have eventually ran out of nodes to visit means we are inside
            neigbours.into_iter().for_each(|n| {
                tile_inside.insert(n);
            });
        }
    }

//    for r in 0..space.h {
//        for c in 0..space.w {
//            if tile_inside.contains(&(r,c)) {
//                print!("I");
//                continue;
//            }
//            if tile_outside.contains(&(r,c)) {
//                print!("O");
//                continue;
//            }
//            print!("{}", space.tiles[r][c].0);
//        }
//
//        println!("");
//    }

    Ok(tile_inside.len())
}

#[cfg(test)]
mod test {

    #[allow(unused_imports)]
    use std::error::Error;
    #[allow(unused_imports)]
    use std::fs::read_to_string;
    #[cfg(feature = "problem_1")]
    const P1_TRAIN_SOLUTION: usize = 8;

    #[cfg(feature = "problem_2")]
    const P2_TRAIN_SOLUTION: usize = 10;

    #[test]
    #[cfg(feature = "problem_1")]
    fn train_problem_1() -> Result<(), Box<dyn Error>> {
        use super::solve_1;
        let input = read_to_string("resources/day_10/train_problem_1.inp")?;
        let result = solve_1(&input)?;
        assert_eq!(result, P1_TRAIN_SOLUTION);
        Ok(())
    }

    #[test]
    #[cfg(feature = "problem_1")]
    fn solve_problem_1() -> Result<(), Box<dyn Error>> {
        use super::solve_1;
        let input = read_to_string("resources/day_10/problem_1.inp")?;
        let result = solve_1(&input)?;
        println!("{result}");
        Ok(())
    }

    #[test]
    #[cfg(feature = "problem_2")]
    fn train_problem_2() -> Result<(), Box<dyn Error>> {
        use super::solve_2;
        let input = read_to_string("resources/day_10/train_problem_2.inp")?;
        let result = solve_2(&input)?;
        assert_eq!(result, P2_TRAIN_SOLUTION);
        Ok(())
    }

    #[test]
    #[cfg(feature = "problem_2")]
    fn solve_problem_2() -> Result<(), Box<dyn Error>> {
        use super::solve_2;
        let input = read_to_string("resources/day_10/problem_2.inp")?;
        let result = solve_2(&input)?;
        println!("{result}");
        Ok(())
    }
}
