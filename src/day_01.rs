use anyhow::Context;

pub fn problem_1(input: String) -> usize {
    let calibration_lines = input
        .lines()
        .map(|line| {
            line.chars()
                .fold(None, |value, c| match value {
                    Some((first, last)) => Some((first, if c.is_ascii_digit() { c } else { last })),
                    None => {
                        if c.is_ascii_digit() {
                            Some((c, c))
                        } else {
                            None
                        }
                    }
                })
                .context("All lines should contain a digit.")
                .unwrap()
        })
        .map(|(first, last)| format!("{}{}", first, last))
        .map(|value| value.parse::<usize>().unwrap());

    calibration_lines.sum()
}

pub fn problem_2(input: String) -> usize {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
    };

    use nom::IResult;

    fn spelled(s: &str) -> IResult<&str, &str> {
        alt((
            tag("one"),
            tag("two"),
            tag("three"),
            tag("four"),
            tag("five"),
            tag("six"),
            tag("seven"),
            tag("eight"),
            tag("nine"),
        ))(s)
    }

    fn take1(s: &str) -> IResult<&str, &str> {
        take(1usize)(s)
    }

    let calibration_lines = input
        .lines()
        .map(|mut line| {
            let mut calibration = None;
            loop {
                let value = if let Ok((_, value)) = spelled(line) {
                    Some(match value {
                        "one" => "1",
                        "two" => "2",
                        "three" => "3",
                        "four" => "4",
                        "five" => "5",
                        "six" => "6",
                        "seven" => "7",
                        "eight" => "8",
                        "nine" => "9",
                        x => unreachable!("Guaranteed by the spelled function: {}", x),
                    })
                } else if let Ok((_, value)) = take1(line) {
                    if value.chars().next().unwrap().is_ascii_digit() {
                        Some(value)
                    } else {
                        None
                    }
                } else {
                    break;
                };

                //line = value.0;
                (line, _) = take1(line).unwrap();

                if let Some(value) = value {
                    calibration = match calibration {
                        Some((first, _)) => Some((first, value)),
                        None => Some((value, value)),
                    }
                }
            }
            calibration.unwrap()
        })
        .map(|(first, last)| format!("{}{}", first, last))
        .map(|value| value.parse::<usize>().unwrap());

    calibration_lines.sum()
}

#[cfg(test)]
mod test {
    use std::error::Error;
    use std::fs::read_to_string;

    use super::problem_1;
    use super::problem_2;

    #[test]
    fn train_problem_1() -> Result<(), Box<dyn Error>> {
        let calibration_lines = read_to_string("resources/day_01/problem_1_train.inp")?;
        let result = problem_1(calibration_lines);
        assert_eq!(result, 142);
        Ok(())
    }

    #[test_log::test]
    fn solve_problem_1() -> Result<(), Box<dyn Error>> {
        let calibration_lines = read_to_string("resources/day_01/problem_1.inp")?;
        let result = problem_1(calibration_lines);

        println!("{result}");
        Ok(())
    }

    #[test]
    fn train_problem_2() -> Result<(), Box<dyn Error>> {
        let calibration_lines = read_to_string("resources/day_01/problem_2_train.inp")?;
        let result = problem_2(calibration_lines);
        assert_eq!(result, 281);
        Ok(())
    }

    #[test]
    fn solve_problem_2() -> Result<(), Box<dyn Error>> {
        let calibration_lines = read_to_string("resources/day_01/problem_2.inp")?;
        let result = problem_2(calibration_lines);

        println!("{result}");
        Ok(())
    }
}
