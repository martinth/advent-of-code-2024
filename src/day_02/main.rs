use anyhow::{Result};
use itertools::Itertools;

#[macro_use]
extern crate simple_log;

#[derive(Debug)]
pub struct Input {
    reports: Vec<Vec<i32>>
}

mod parse {
    use aoc_parse::{parser, prelude::*};
    use anyhow::{Result, Context};
    use std::fs::read_to_string;
    use crate::Input;

    pub fn parse_input(filename: &str) -> Result<Input> {
        let parser = parser!(lines(repeat_sep(i32, " ")));

        let raw_data = read_to_string(filename)?;
        let raw_parsed = parser.parse(&raw_data).context("parse error")?;

        Ok(Input {
            reports: raw_parsed
        })
    }
}

fn solve_part_1(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename)?;
    debug!("Input: {:?}", input);

    let mut safe_reports = 0;
    for report in input.reports.into_iter() {

        let is_safe = is_safe(&report, None);
        debug!("{:?} safe={}", report, is_safe);
        if is_safe {
            safe_reports += 1
        }
    }

    Ok(safe_reports)
}

fn solve_part_2(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename)?;
    debug!("Input: {:?}", input);

    let mut safe_reports = 0;
    for report in input.reports.into_iter() {
        if is_safe_with_one_removed(report) {
            safe_reports += 1
        }
    }

    Ok(safe_reports)
}

#[derive(Debug, Eq, PartialEq)]
pub enum ExpectLevelChanges {
    Increasing,
    Decreasing,
}

/// Check if a report is safe. If ignore_index is given that one index is ignored when checking.
///
/// The method is a bit long compared to a naive implementation because it avoids any allocations.
fn is_safe(report: &[i32], ignore_index: Option<usize>) -> bool {

    let mut item_pairs = report.iter().enumerate()
        // if we want to ignore an index we skip it, if we don't want to ignore anything we let everything trough
        .filter(|(index, _)| ignore_index.map_or(true, |to_ignore| to_ignore != *index))
        // discard index and deref item
        .map(|(_, item)| *item)
        // create window of 2-tuples
        .tuple_windows::<(_,_)>()
        .peekable();

    // figure out if we expect increasing or decreasing orders
    let first_two = item_pairs.peek().expect("needs at least two elements");
    let expected_change = if first_two.1 > first_two.0 {
        ExpectLevelChanges::Increasing
    } else {
        ExpectLevelChanges::Decreasing
    };

    // check pairs and return as soon as we know something is wrong
    for (first, second) in item_pairs {
        let delta = second - first;
        let delta_okay = if expected_change == ExpectLevelChanges::Increasing {
            delta >= 1 && delta <= 3
        } else {
            delta <= -1 && delta >= -3
        };
        if !delta_okay {
            return false
        }
    }
    return true
}

fn is_safe_with_one_removed(report: Vec<i32>) -> bool {
    if is_safe(&report, None) {
        return true
    }

    for (idx, _) in report.iter().enumerate() {
        if is_safe(&report, Some(idx)) {
            return true
        }
    }

    return false
}


fn main() -> Result<()> {
    simple_log::quick!("info");

    info!("Result part 1: {}", solve_part_1("src/day_02/input.txt")?);
    info!("Result part 2: {}", solve_part_2("src/day_02/input.txt")?);
    Ok(())
}


#[cfg(test)]
mod tests {
    use ctor::ctor;
    use crate::{is_safe_with_one_removed, is_safe, solve_part_1, solve_part_2};

    #[ctor]
    fn init() {
        simple_log::quick!("debug");
    }

    #[test]
    fn solve_test_input_1() {
        let result = solve_part_1("src/day_02/test_input.txt").unwrap();
        assert_eq!(result, 2);
    }
    #[test]
    fn solve_test_input_2() {
        let result = solve_part_2("src/day_02/test_input.txt").unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn detect_safe() {
        let result = is_safe(&vec![38, 41, 40, 42, 45, 47, 50, 52], None);
        assert_eq!(result, false);
    }

    #[test]
    fn detect_safe_with_skip() {
        let result = is_safe(&vec![38, 41, 40, 42, 45, 47, 50, 52], Some(2));
        assert_eq!(result, true);
    }

    #[test]
    fn check_removals_needed() {
        let result = is_safe_with_one_removed(vec![38, 41, 40, 42, 45, 47, 50, 52]);
        assert_eq!(result, true);
    }

    #[test]
    #[cfg(feature = "count-allocations")]
    fn verify_no_allocations() {
        let data = &vec![38, 41, 40, 42, 45, 47, 50, 52];
        let info = allocation_counter::measure(|| {
            is_safe(data, Some(2));
        });
        assert_eq!(info.count_total, 0);
    }

}