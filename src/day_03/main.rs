use anyhow::{Result};

#[macro_use]
extern crate simple_log;

#[derive(Debug)]
pub enum Instruction {
    Mul(u64, u64),
    Do,
    Dont
}

#[derive(Debug)]
pub struct Input {
    instructions: Vec<Instruction>
}


mod parse {
    use anyhow::{Result};
    use std::fs::read_to_string;
    use aoc_parse::{parser, Parser};
    use aoc_parse::prelude::u64;
    use crate::{Input, Instruction};
    use regex::Regex;


    pub fn parse_input(filename: &str) -> Result<Input> {
        let raw_data = read_to_string(filename)?;
        let re = Regex::new(r":mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();

        let instruction_parser = parser!({
            "mul(" p1:u64 "," p2:u64 ")" => Instruction::Mul(p1, p2),
            "do()" => Instruction::Do,
            "don't()" => Instruction::Dont,
        });

        let instructions = re.find_iter(&raw_data)
            .map(|m| m.as_str())
            .map(|raw| instruction_parser.parse(raw).unwrap())
            .collect::<Vec<Instruction>>();

        Ok(Input {
            instructions
        })
    }
}

fn solve_part_1(filename: &str) -> Result<u64> {
    let input = parse::parse_input(filename)?;
    debug!("{:?}", input);

    let result = input.instructions.iter()
        .fold(0, |total, inst| {
            match inst {
                Instruction::Mul(a, b) => total + (a * b),
                _ => total // ignored
            }
        });

    Ok(result)
}


fn solve_part_2(filename: &str) -> Result<u64> {
    let input = parse::parse_input(filename)?;
    debug!("{:?}", input);

    let mut total = 0;
    let mut enabled = true;
    for instruction in input.instructions {
        match instruction {
            Instruction::Mul(a, b) if enabled => total += a * b,
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
            _ => () // do nothing
        }
    }

    Ok(total)
}

fn main() -> Result<()> {
    simple_log::quick!("info");

    info!("Result part 1: {}", solve_part_1("src/day_03/input.txt")?);
    info!("Result part 2: {}", solve_part_2("src/day_03/input.txt")?);
    Ok(())
}


#[cfg(test)]
mod tests {
    use ctor::ctor;
    use crate::{solve_part_1, solve_part_2};

    #[ctor]
    fn init() {
        simple_log::quick!("debug");
    }

    #[test]
    fn solve_test_input_1() {
        let result = solve_part_1("src/day_03/test_input_part_01.txt").unwrap();
        assert_eq!(result, 161);
    }

    #[test]
    fn solve_test_input_2() {
        let result = solve_part_2("src/day_03/test_input_part_02.txt").unwrap();
        assert_eq!(result, 48);
    }
}