use std::str::FromStr;

use anyhow::Result;

#[derive(PartialEq, Eq)]
enum SchematicCell {
    Blank,
    Symbol(char),
    Number(usize),
}

#[derive(Default)]
struct Schematic {
    width: usize,
    height: usize,
    cells: Vec<Vec<SchematicCell>>,
}

#[derive(Default)]
struct SchematicMap {
    parts: Vec<(isize, isize, char)>,
    part_numbers: Vec<(isize, isize, isize, usize)>,
}

impl FromStr for SchematicMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = vec![];
        let mut part_numbers = vec![];

        let mut number = String::default();
        let mut number_start = None;
        for (row, line) in s.lines().enumerate() {
            if let Some(start) = number_start {
                part_numbers.push((
                    row as isize,
                    start as isize,
                    (line.len() as isize) - 1,
                    number.parse::<usize>()?,
                ));
            }
            number = String::default();
            number_start = None;
            for (col, cell) in line.chars().enumerate() {
                match cell {
                    cell if cell.is_ascii_digit() => {
                        let _ = number_start.get_or_insert(col);
                        number.push(cell);
                    }
                    cell => {
                        if let Some(start) = number_start {
                            part_numbers.push((
                                row as isize,
                                start as isize,
                                (col as isize) - 1,
                                number.parse::<usize>()?,
                            ));
                        }
                        number = String::default();
                        number_start = None;

                        if cell != '.' {
                            parts.push((row as isize, col as isize, cell));
                        }
                    }
                }
            }
        }

        Ok(SchematicMap {
            parts,
            part_numbers,
        })
    }
}

impl Schematic {
    fn get(&self, row: isize, col: isize) -> Option<&SchematicCell> {
        let row: usize = row.try_into().ok()?;
        let col: usize = col.try_into().ok()?;
        self.cells.get(row).and_then(|row| row.get(col))
    }
}

fn parse_schematic(schematic: &str) -> Result<Schematic> {
    let mut parsed_schematic = Schematic::default();

    for (row, line) in schematic.lines().enumerate() {
        let mut parsed_schematic_row = Vec::with_capacity(line.len());
        for c in line.chars() {
            if c.is_ascii_digit() {
                parsed_schematic_row.push(SchematicCell::Number(format!("{c}").parse::<usize>()?));
            } else if c != '.' {
                parsed_schematic_row.push(SchematicCell::Symbol(c));
            } else {
                parsed_schematic_row.push(SchematicCell::Blank);
            }
        }
        parsed_schematic.cells.push(parsed_schematic_row);
        parsed_schematic.height = row + 1;
        parsed_schematic.width = std::cmp::max(parsed_schematic.width, line.len());
    }

    Ok(parsed_schematic)
}

pub mod problem_1 {

    use super::{parse_schematic, Schematic, SchematicCell};
    use anyhow::Result;

    #[allow(dead_code)]
    fn get_number(schematic: &Schematic, row: isize, col: isize) -> Option<usize> {
        // If its not a number return none.
        match schematic.get(row, col) {
            Some(SchematicCell::Number(_)) => (),
            _ => return None,
        }

        let mut num_start = col;
        while let Some(SchematicCell::Number(_)) = schematic.get(row, num_start - 1) {
            num_start -= 1;
        }

        let mut number = 0usize;
        while let Some(SchematicCell::Number(ref d)) = schematic.get(row, num_start) {
            number = number * 10 + d;
            num_start += 1;
        }

        Some(number)
    }

    fn touches_symbol(schematic: &Schematic, col: usize, row: usize) -> bool {
        let mut col_s = (col as isize) - 1;
        let mut row_s = (row as isize) - 1;

        let col_e = (col as isize) + 1;
        let row_e = (row as isize) + 1;

        loop {
            if row_s > row_e {
                break;
            }
            loop {
                if col_s > col_e {
                    break;
                }
                if let Some(SchematicCell::Symbol(_)) = schematic.get(row_s, col_s) {
                    return true;
                }
                col_s += 1;
            }
            row_s += 1;
            col_s = (col as isize) - 1;
        }

        false
    }

    pub fn solve(input: &str) -> Result<usize> {
        let schematic = parse_schematic(input)?;

        let mut sum: usize = 0;

        enum State {
            Number(usize),
            Part(usize),
            Other(usize),
        }

        let mut state = State::Other(0);
        for (row, line) in schematic.cells.iter().enumerate() {
            for (col, cell) in line.iter().enumerate() {
                match cell {
                    SchematicCell::Number(ref d) => {
                        state = match state {
                            State::Number(n) | State::Other(n) => {
                                if touches_symbol(&schematic, col, row) {
                                    State::Part(n * 10 + d)
                                } else {
                                    State::Number(n * 10 + d)
                                }
                            }
                            State::Part(n) => State::Part(n * 10 + d),
                        };
                    }
                    _ => {
                        if let State::Part(n) = state {
                            sum += n;
                        }
                        state = State::Other(0)
                    }
                }
            }
            if let State::Part(n) = state {
                sum += n;
            }
            state = State::Other(0)
        }
        if let State::Part(n) = state {
            sum += n;
        }

        Ok(sum)
    }
}

pub mod problem_2 {
    use super::SchematicMap;
    use anyhow::Result;

    pub fn solve(input: &str) -> Result<usize> {
        let map: SchematicMap = input.parse()?;

        let mut gears = 0;

        for (row, col, _) in map.parts.iter().filter(|(_, _, part)| *part == '*') {
            let mut part = Vec::new();
            for part_number in map.part_numbers.iter() {
                let within_row_range = *row >= part_number.0 - 1 && *row <= part_number.0 + 1;
                let within_col_range = *col >= part_number.1 - 1 && *col <= part_number.2 + 1;

                if within_row_range && within_col_range {
                    part.push(part_number.3);
                }
            }

            if part.len() == 2 {
                gears += part.into_iter().reduce(|prod, num| prod * num).unwrap();
            }
        }

        Ok(gears)
    }
}

#[cfg(test)]
mod test {

    use std::error::Error;
    use std::fs::read_to_string;

    const P1_TRAIN_SOLUTION: usize = 4361;
    const P2_TRAIN_SOLUTION: usize = 467835;

    #[test]
    fn train_problem_1() -> Result<(), Box<dyn Error>> {
        use super::problem_1::solve;
        let input = read_to_string("resources/day_03/train_problem_1.inp")?;
        let result = solve(&input)?;
        assert_eq!(result, P1_TRAIN_SOLUTION);
        Ok(())
    }

    #[test]
    fn solve_problem_1() -> Result<(), Box<dyn Error>> {
        use super::problem_1::solve;
        let input = read_to_string("resources/day_03/problem_1.inp")?;
        let result = solve(&input)?;
        println!("{result}");
        Ok(())
    }

    #[test]
    fn train_problem_2() -> Result<(), Box<dyn Error>> {
        use super::problem_2::solve;
        let input = read_to_string("resources/day_03/train_problem_2.inp")?;
        let result = solve(&input)?;
        assert_eq!(result, P2_TRAIN_SOLUTION);
        Ok(())
    }

    #[test]
    fn solve_problem_2() -> Result<(), Box<dyn Error>> {
        use super::problem_2::solve;
        let input = read_to_string("resources/day_03/problem_2.inp")?;
        let result = solve(&input)?;
        println!("{result}");
        Ok(())
    }
}
