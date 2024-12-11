use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use anyhow::Result;

#[macro_use]
extern crate simple_log;

#[derive(Debug)]
pub struct Input {
    numbers: Vec<u64>
}

mod parse {
    use aoc_parse::{parser, prelude::*};
    use anyhow::{Result, Context};
    use std::fs::read_to_string;
    use crate::Input;

    pub fn parse_input(filename: &str) -> Result<Input> {
        let parser = parser!(repeat_sep(u64, " "));

        let raw_data = read_to_string(filename)?;
        let numbers = parser.parse(&raw_data).context("parse error")?;

        Ok(Input {
            numbers
        })
    }
}


#[derive(Debug, Default)]
pub struct PartialResult {
    zeros: usize,
    ones: usize,
    numbers: HashMap<u64, usize>
}

impl PartialResult {
    fn add_number(&mut self, number: u64, increment: usize) {
        self.numbers.entry(number)
            .and_modify(|total| *total += increment)
            .or_insert(increment);
    }

    fn total(&self) -> usize {
        self.zeros + self.ones + self.numbers.values().sum::<usize>()
    }
}

impl Display for PartialResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("0={}, 1={}, numbers={}", self.zeros, self.ones, self.numbers.len()).as_str())
    }
}

fn solve(filename: &str, blink_count: usize) -> Result<usize> {
    let input = parse::parse_input(filename)?;

    // put input into partial result
    let mut state = PartialResult::default();
    for start_number in input.numbers {
        match start_number {
            0 => { state.zeros += 1; },
            1 => { state.zeros += 1; },
            _ => { state.add_number(start_number, 1); }
        };
    }

    debug!("{}", state);
    for idx in 0..blink_count {
        let mut new_state = PartialResult::default();

        // zeros become ones
        new_state.ones = state.zeros;

        // ones become 2024
        new_state.add_number(2024, state.ones);

        for (number, count) in &state.numbers {
            let digits = number.ilog10() + 1;
            if digits % 2 == 0 {
                // number has event digits: split into two numbers
                let split_at = digits / 2;
                let modifier = 10u64.pow(split_at);
                let first_half = number / modifier;
                let second_half = number % modifier;

                // first half can't be null
                new_state.add_number(first_half, *count);

                // second might be null, but maybe not
                if second_half == 0 {
                    new_state.zeros += count
                } else {
                    new_state.add_number(second_half, *count);
                }

            } else {
                // number has off digits: multiply by 2024
                let new_number = number * 2024;
                new_state.add_number(new_number, *count);
            }
        }

        let _ = std::mem::replace(&mut state, new_state);

        debug!("Iteration {}: {}", idx, state);
    }

    Ok(state.total())
}


fn main() -> Result<()> {
    simple_log::quick!("info");

    info!("Result part 1: {}", solve("src/day_11/input.txt", 25)?);
    info!("Result part 2: {}", solve("src/day_11/input.txt", 75)?);
    Ok(())
}


#[cfg(test)]
mod tests {
    use ctor::ctor;
    use crate::{solve};

    #[ctor]
    fn init() {
        simple_log::quick!("debug");
    }

    #[test]
    fn solve_test_input() {
        assert_eq!(solve("src/day_11/test_input.txt", 6).unwrap(), 22);
    }

}