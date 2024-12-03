use anyhow::{Result};

#[macro_use]
extern crate simple_log;

#[derive(Debug)]
pub enum Instruction {
    Mul(u64, u64)
}

#[derive(Debug)]
pub struct Input {
    instructions: Vec<Instruction>
}


mod parse {
    use anyhow::{Result};
    use std::fs::read_to_string;
    use crate::{Input, Instruction};
    use regex::Regex;


    pub fn parse_input(filename: &str) -> Result<Input> {
        let raw_data = read_to_string(filename)?;
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

        let instructions = re.captures_iter(&raw_data)
            .map(|cap| (cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str()))
            .map(|(parma1, param2)| Instruction::Mul(parma1.parse().unwrap(), param2.parse().unwrap()))
            .collect::<Vec<Instruction>>();

        debug!("{:?}", instructions);

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
                Instruction::Mul(a, b) => total + (a * b)
            }
        });

    Ok(result)
}

fn solve_part_2(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename)?;
    debug!("{:?}", input);

    todo!()
}

fn main() -> Result<()> {
    simple_log::quick!("info");

    info!("Result part 1: {}", solve_part_1("src/day_03/input.txt")?);
    info!("Result part 2: {}", solve_part_2("src/day_03/input.txt")?);
    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::{solve_part_1, solve_part_2};

    #[test]
    fn solve_test_input_1() {
        simple_log::quick!("debug");

        let result = solve_part_1("src/day_03/test_input.txt").unwrap();
        assert_eq!(result, 161);
    }

    #[test]
    fn solve_test_input_2() {
        simple_log::quick!("debug");

        let result = solve_part_2("src/day_03/test_input.txt").unwrap();
        assert_eq!(result, 42);
    }
}