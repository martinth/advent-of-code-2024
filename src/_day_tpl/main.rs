use anyhow::{Result, Context};
use const_format::formatcp;

#[macro_use]
extern crate simple_log;

const DAY: &str = "day_xx";

#[derive(Debug)]
pub struct Input {

}

mod parse {
    use aoc_parse::{parser, prelude::*};
    use anyhow::{Result, Context};
    use std::fs::read_to_string;
    use crate::Input;

    pub fn parse_input(filename: &str) -> Result<Input> {
        let parser = parser!(lines(""));

        let raw_data = read_to_string(filename)?;
        let raw_parsed = parser.parse(&raw_data).context("parse error")?;

        Ok(Input {

        })
    }
}

fn solve_part_1(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename)?;
    debug!("{:?}", input);

    todo!()
}

fn solve_part_2(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename)?;
    debug!("{:?}", input);

    todo!()
}

fn main() -> Result<()> {
    simple_log::quick!("info");

    info!("Result part 1: {}", solve_part_1(formatcp!("src/{}/input.txt", DAY))?);
    info!("Result part 2: {}", solve_part_2(formatcp!("src/{}/input.txt", DAY))?);
    Ok(())
}


#[cfg(test)]
mod tests {
    use const_format::formatcp;
    use crate::{solve_part_1, solve_part_2, DAY};

    #[test]
    fn solve_test_input_1() {
        simple_log::quick!("debug");

        let result = solve_part_1(formatcp!("src/{}/test_input.txt", DAY)).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn solve_test_input_2() {
        simple_log::quick!("debug");

        let result = solve_part_2(formatcp!("src/{}/test_input.txt", DAY)).unwrap();
        assert_eq!(result, 42);
    }
}