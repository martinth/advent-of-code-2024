use std::cmp::PartialEq;
use anyhow::{Result};
use itertools::{Itertools, repeat_n};
use crate::Operator::{Add, Multiply};

#[macro_use]
extern crate simple_log;


#[derive(Debug)]
pub struct Input {
    equations: Vec<Equation>
}

#[derive(Debug, Clone)]
pub struct Equation {
    desired_result: u64,
    numbers: Vec<u64>
}

#[derive(Debug)]
pub struct Solution {
    equation: Equation,
    operators: Vec<Operator>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Add,
    Multiply,
    Concat
}

#[derive(Debug)]
pub enum RecResult {
    Found(Solution),
    TooLarge
}

mod parse {
    use aoc_parse::{parser, prelude::*};
    use anyhow::{Result, Context};
    use std::fs::read_to_string;
    use crate::{Equation, Input};

    pub fn parse_input(filename: &str) -> Result<Input> {
        let parser = parser!(lines(result:u64 ": " numbers:repeat_sep(u64, " ") => Equation { desired_result: result, numbers }));

        let raw_data = read_to_string(filename)?;
        let equations = parser.parse(&raw_data).context("parse error")?;

        Ok(Input {
            equations
        })
    }
}

/// Find a solution for the input given a set of operators we can use
fn solve(input: Input, possible_operators: Vec<Operator>) -> Result<u64> {
    let mut total_results = 0u64;
    for equation in input.equations.iter() {
        if let Some(solution) = find_solution(&equation, &possible_operators) {
            total_results += solution.equation.desired_result;
        }
    }

    Ok(total_results)
}

/// Find a potential solution by checking all possible operator combinations.
fn find_solution(equation: &Equation, possible_operators: &Vec<Operator>) -> Option<Solution> {
    let all_operators= repeat_n(possible_operators, equation.numbers.len() - 1)
        .multi_cartesian_product();

    for operators in all_operators {
        let result = compute_result(equation, &operators);
        if result == equation.desired_result {
            let solution = Solution {
                equation: equation.clone(),
                operators: operators.iter().map(|o| (*o).clone()).collect()
            };
            debug!("Found solution for {equation:?} {operators:?}");
            return Some(solution);
        }
    }

    None
}

/// Computer the result of the equation using the given operators
fn compute_result(equation: &Equation, operators: &Vec<&Operator>) -> u64 {
    equation.numbers.iter().enumerate()
        .fold(None, |part_result: Option<u64>, (num_idx, number)| {
            if let Some(part_result) = part_result {
                let operator = operators.get(num_idx - 1).unwrap();
                match operator {
                    Add => Some(part_result + number),
                    Multiply => Some(part_result * number),
                    Operator::Concat => Some(format!("{}{}", part_result, number).parse().unwrap()) // could this be made more efficient?!

                }
            } else {
                Some(*number)
            }
        }).unwrap()
}

fn solve_part_1(filename: &str) -> Result<u64> {
    let input = parse::parse_input(filename)?;
    solve(input, vec![Add, Multiply])
}

fn solve_part_2(filename: &str) -> Result<u64> {
    let input = parse::parse_input(filename)?;
    solve(input, vec![Add, Multiply, Operator::Concat])
}

fn main() -> Result<()> {
    simple_log::quick!("info");

    info!("Result part 1: {}", solve_part_1("src/day_07/input.txt")?);
    info!("Result part 2: {}", solve_part_2("src/day_07/input.txt")?);
    Ok(())
}


#[cfg(test)]
mod tests {
    use ctor::ctor;
    use itertools::{Itertools, repeat_n};
    use crate::{Equation, find_solution, Operator, solve_part_1, solve_part_2};
    use crate::Operator::{Add, Multiply};

    #[ctor]
    fn init() {
        simple_log::quick!("debug");
    }

    #[test]
    fn solve_test_input_1() {
        let result = solve_part_1("src/day_07/test_input.txt").unwrap();
        assert_eq!(result, 3749);
    }


    #[test]
    fn solve_test_input_2() {
        let result = solve_part_2("src/day_07/test_input.txt").unwrap();
        assert_eq!(result, 11387);
    }

    #[test]
    fn solve_find_solution() {
        let equation = Equation {
            desired_result: 292,
            numbers: vec![11, 6, 16, 20]
        };
        let solution = find_solution(&equation, &vec![Add, Multiply]);

        match solution {
            Some(solution) => assert_eq!(solution.operators, vec![Add, Multiply, Add]),
            None => panic!("No solution found")
        }
    }
}