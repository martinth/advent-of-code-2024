use std::collections::BTreeMap;
use anyhow::{Result};
use const_format::formatcp;

#[macro_use]
extern crate simple_log;

const DAY: &str = "day_01";

#[derive(Debug)]
pub struct Input {
    col0: Vec<u32>,
    col1: Vec<u32>
}

mod parse {
    use aoc_parse::{parser, prelude::*};
    use anyhow::{Result, Context};
    use std::fs::read_to_string;
    use crate::Input;

    pub fn parse_input(filename: &str) -> Result<Input> {
        let parser = parser!(lines(u32 "   " u32));

        let raw_data = read_to_string(filename)?;
        let raw_parsed: Vec<(u32, u32)> = parser.parse(&raw_data).context("parse error")?;

        let mut col0: Vec<u32> = Vec::with_capacity(raw_parsed.len());
        let mut col1: Vec<u32> = Vec::with_capacity(raw_parsed.len());

        for (i0, i1) in raw_parsed {
            col0.push(i0);
            col1.push(i1);
        }

        Ok(Input {
            col0,
            col1,
        })
    }
}

fn solve_part_1(filename: &str) -> Result<u32> {
    let mut input = parse::parse_input(filename)?;

    input.col0.sort();
    input.col1.sort();

    let mut total_distance: u32 = 0;
    let sorted_pairs = input.col0.iter().zip(input.col1);
    for (i0, i1) in sorted_pairs {
        total_distance += i0.abs_diff(i1);
    }

    Ok(total_distance)
}

fn solve_part_2(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename)?;

    let frequencies_right_list: BTreeMap<&u32, u32> = input.col1
        .iter().fold(BTreeMap::new(), |mut freq, item| {
            freq.entry(item)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            freq
        });

    debug!("frequencies_right_list: {:?}", frequencies_right_list);

    let mut similarity_score: u32 = 0;
    for i0 in input.col0 {
        let occurrences_right = frequencies_right_list.get(&i0).unwrap_or(&(0u32));
        similarity_score += i0 * occurrences_right;
    }

    Ok(similarity_score)
}

fn main() -> Result<()> {
    simple_log::quick!("info");

    info!("Result part 1: {}", solve_part_1(formatcp!("src/{}/input.txt", DAY))?);
    info!("Result part 2: {}", solve_part_2(formatcp!("src/{}/input.txt", DAY))?);
    Ok(())
}


#[cfg(test)]
mod tests {
    use ctor::ctor;
    use const_format::formatcp;
    use crate::{solve_part_1, solve_part_2, DAY};

    #[ctor]
    fn init() {
        simple_log::quick!("debug");
    }

    #[test]
    fn solve_test_input_1() {
        let result = solve_part_1(formatcp!("src/{}/test_input.txt", DAY)).unwrap();
        assert_eq!(result, 11);
    }

    #[test]
    fn solve_test_input_2() {
        let result = solve_part_2(formatcp!("src/{}/test_input.txt", DAY)).unwrap();
        assert_eq!(result, 31);
    }
}