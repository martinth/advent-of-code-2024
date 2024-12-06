use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use anyhow::{Result, Context};
use std::iter::Extend;

#[macro_use]
extern crate simple_log;

#[derive(Debug)]
pub struct Rule {
    index: usize,
    before: u32,
    after: u32
}

impl Hash for Rule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state)
    }
}

impl PartialEq<Self> for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for Rule {

}

#[derive(Debug)]
pub struct Input {
    rules: Vec<Rule>,
    print_jobs: Vec<Vec<u32>>
}

mod parse {
    use aoc_parse::{parser, prelude::*};
    use anyhow::{Result, Context};
    use std::fs::read_to_string;
    use crate::{Input, Rule};

    pub fn parse_input(filename: &str) -> Result<Input> {
        let parser = parser!(
            section(lines(before:u32 "|" after:u32  => (before, after)))
            section(lines(repeat_sep(u32, ",")))
        );

        let raw_data = read_to_string(filename)?;
        let raw_parsed = parser.parse(&raw_data).context("parse error")?;

        Ok(Input {
            rules: raw_parsed.0.into_iter().enumerate().map(|(index, (before, after))| Rule { index, before, after }).collect(),
            print_jobs: raw_parsed.1
        })
    }
}

/// Find the page ordering rules that are valid for a given print job of pages.
fn get_relevant_rules<'r>(rules_for_pages: &'r HashMap<u32, Vec<&Rule>>, print_job: &Vec<u32>) -> HashSet<&'r Rule> {
    print_job.iter()
        .flat_map(|page| rules_for_pages.get(page).unwrap()
            .into_iter()
            .map(|r| *r) // deref
        )
        .collect()
}

/// Check if a print job is valid for the given rule set
fn is_valid_job(print_job: &Vec<u32>, rules: &HashSet<&Rule>) -> bool {

    // this might be optimize able by checking from the page left and right in lockstep
    for (page_idx, page) in print_job.iter().enumerate() {
        // check forward
        for page_after in &print_job[page_idx + 1..] {
            let any_rule = rules.iter().any(|rule| rule.before == *page && rule.after == *page_after);
            if !any_rule {
                return false
            }
        }
        // check forward
        for page_before in &print_job[0..page_idx] {
            let any_rule = rules.iter().any(|rule| rule.before == *page_before && rule.after == *page);
            if !any_rule {
                return false
            }
        }

    }

    true
}

fn page_to_rules_map(rules: &Vec<Rule>) -> HashMap<u32, Vec<&Rule>> {
    let mut rules_for_pages: HashMap<u32, Vec<&Rule>> = HashMap::new();

    rules.iter().for_each(|rule| {
        rules_for_pages.entry(rule.before)
            .and_modify(|rules| rules.push(rule))
            .or_insert(vec![rule]);
        rules_for_pages.entry(rule.after)
            .and_modify(|rules| rules.push(rule))
            .or_insert(vec![rule]);
    });

    rules_for_pages
}

fn solve_part_1(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename)?;

    let rules_for_pages = page_to_rules_map(&input.rules);

    let mut total = 0u32;
    for (job_idx, print_job) in input.print_jobs.iter().enumerate() {

        let mut relevant_rules = get_relevant_rules(&rules_for_pages, print_job);
        let valid = is_valid_job(print_job, &relevant_rules);

        debug!("Job {}: {}/{} rules -> {}", job_idx, relevant_rules.len(), input.rules.len(), valid);
        if valid {
            let middle_page = print_job.get(print_job.len() / 2).unwrap();
            total += middle_page;
        }

    }

    Ok(total)
}

fn solve_part_2(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename)?;

    todo!()
}

fn main() -> Result<()> {
    simple_log::quick!("info");

    info!("Result part 1: {}", solve_part_1("src/day_05/input.txt")?);
    info!("Result part 2: {}", solve_part_2("src/day_05/input.txt")?);
    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::{solve_part_1, solve_part_2};

    #[test]
    fn solve_test_input_1() {
        simple_log::quick!("debug");

        let result = solve_part_1("src/day_05/test_input.txt").unwrap();
        assert_eq!(result, 143);
    }

    #[test]
    fn solve_test_input_2() {
        simple_log::quick!("debug");

        let result = solve_part_2("src/day_05/test_input.txt").unwrap();
        assert_eq!(result, 123);
    }
}