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

pub struct Input {
    start: (usize, usize),
    space: Space,
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Result<Input> {
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

#[aoc(day10, part1)]
pub fn solve_1(input: &Input) -> Result<usize> {
    use Dir::*;
    let space = &input.space;

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

#[aoc(day10, part2)]
pub fn solve_2(input: &Input) -> Result<usize> {
    use Dir::*;
    let space = &input.space;

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

    //let dirs = pos.collect::<Vec<_>>();
    let (mut dir, mut pos) = pos.next().unwrap();
    //let (mut _dir_other, _) = dirs.get(1).unwrap();

    let mut mask = HashMap::new();
    mask.insert(input.start, 'S');

    while let Some(tile) = space.get(&pos) {
        if let &Tile('S') = tile {
            break;
        }

        mask.insert(pos, tile.0);

        dir = tile.redirect(&dir).unwrap();
        pos = space.move_position(&pos, &dir).unwrap();
    }

    let mut tiles_vertical = HashSet::new();
    let mut tiles_horizontal = HashSet::new();

    for r in 0..space.h {
        let mut in_vertical = false;
        let mut in_horizontal = false;
        for c in 0..space.w {
            match (in_horizontal, mask.get(&(r, c))) {
                (true, None) => {
                    tiles_horizontal.insert((r, c));
                }
                (_, Some('|' | 'J' | 'L')) => in_horizontal = !in_horizontal,
                _ => (),
            }
            match (in_vertical, mask.get(&(c, r))) {
                (true, None) => { tiles_vertical.insert((c, r)); },
                (_, Some('-' | '7' | 'J')) => in_vertical = !in_vertical,
                _ => (),
            }
        }
    }

    let tiles_inside = tiles_vertical
        .intersection(&tiles_horizontal)
        .collect::<HashSet<_>>();

    Ok(tiles_inside.len())
}

#[cfg(test)]
mod test {

    #[allow(unused_imports)]
    use std::error::Error;
    #[allow(unused_imports)]
    use std::fs::read_to_string;

    use crate::day_10::input_generator;
    const P1_TRAIN_SOLUTION: usize = 8;
    const P2_TRAIN_SOLUTION: usize = 10;

    #[test]
    fn train_problem_1() -> Result<(), Box<dyn Error>> {
        use super::solve_1;
        let input = read_to_string("resources/day_10/train_problem_1.inp")?;
        let input = input_generator(&input)?;
        let result = solve_1(&input)?;
        assert_eq!(result, P1_TRAIN_SOLUTION);
        Ok(())
    }

    #[test]
    fn train_problem_2() -> Result<(), Box<dyn Error>> {
        use super::solve_2;
        let input = read_to_string("resources/day_10/train_problem_2.inp")?;
        let input = input_generator(&input)?;
        let result = solve_2(&input)?;
        assert_eq!(result, P2_TRAIN_SOLUTION);
        Ok(())
    }
}
