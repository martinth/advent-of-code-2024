use std::collections::{HashMap};
use std::hash::{Hash, Hasher};
use anyhow::{Result};
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

// A Rule violation at a given index in a print job
#[derive(Debug)]
pub struct Violation<'r> {
    at_index: usize,
    rule: &'r Rule
}

/// The page rules per page number where the page is either in the "before" or in the "after" part.
/// This is used to only check the rules that are relevant for a print job.
#[derive(Debug)]
pub struct PageRules<'r> {
    before: HashMap<u32, Vec<&'r Rule>>,
    after: HashMap<u32, Vec<&'r Rule>>,
}

/// Build the PageRules object from the given input rules.
fn build_page_rules(rules: &Vec<Rule>) -> PageRules {
    let mut page_rules = PageRules {
        before: HashMap::new(),
        after: HashMap::new(),
    };

    rules.iter().for_each(|rule| {
        page_rules.before.entry(rule.before)
            .and_modify(|rules| rules.push(rule))
            .or_insert(vec![rule]);
        page_rules.after.entry(rule.after)
            .and_modify(|rules| rules.push(rule))
            .or_insert(vec![rule]);
    });

    page_rules
}

/// Find first rule violation of a print job given at set of rules.
fn find_violation<'r>(print_job: &Vec<u32>, rules: &'r PageRules) -> Option<Violation<'r>> {
    let empty_rules: Vec<&Rule> = Vec::new();

    // iterate over each page
    for (page_idx, page) in print_job.iter().enumerate() {

        // for checking, we only need the rules where the page is actually a part of
        let rules_where_page_before = rules.before.get(page).unwrap_or(&empty_rules);
        let rules_where_page_after = rules.after.get(page).unwrap_or(&empty_rules);

        // check all pages before the current page for rule violations
        for page_before in &print_job[0..page_idx] {

            // rules_where_page_before already contains only the rules where "page" is the "before" page
            // so if we find a rule that states that a page before it should actually after it, this is a
            // rule violation
            let before_violation = rules_where_page_before.iter()
                .filter_map(|rule| if rule.after == *page_before {
                    Some(Violation {
                        at_index: page_idx,
                        rule: *rule
                    })
                } else {
                    None
                }).next();

            if before_violation.is_some() {
                return before_violation
            }

        }

        // same as previous loop, but we check all pages after the page
        for page_after in &print_job[page_idx + 1..] {
            let after_violation = rules_where_page_after.iter()
                .filter_map(|rule| if rule.before == *page_after {
                    Some(Violation {
                        at_index: page_idx,
                        rule: *rule
                    })
                } else {
                    None
                }).next();

            if after_violation.is_some() {
                return after_violation
            }
        }
    }

    None
}


/// Fix a single rule violation by swapping the two pages that are in the wrong order.
fn fix_violation(print_job: &mut Vec<u32>, violation: &Violation) {
    let page_at_index = print_job[violation.at_index];
    let other_page = if page_at_index == violation.rule.after {
        violation.rule.before
    } else {
        violation.rule.after
    };
    let other_page_index = print_job.into_iter().position(|page| *page == other_page).unwrap();

    debug!("fix: swap index {} with {}", violation.at_index, other_page_index);

    print_job.swap(violation.at_index, other_page_index)
}

/// Fix a single print job by repeatedly fixing the first violation.
fn fix_violations(print_job: &Vec<u32>, page_rules: &PageRules) -> Option<Vec<u32>> {
    let mut fixed_job = print_job.clone();

    let mut needed_fixing = false;

    // There is some big optimization potential here since we only ever use the first violation.
    // But the code is fast enough as is, so I won't do it.
    while let Some(violation) = find_violation(&fixed_job, &page_rules) {
        needed_fixing = true;
        debug!("Job has violation: {violation:?}");
        fix_violation(&mut fixed_job, &violation);
    }

    // We only return Some if we actually fixed something so the calling code can differentiate.
    if needed_fixing {
        Some(fixed_job.clone())
    } else {
        None
    }
}

fn get_middle_page(print_job: &Vec<u32>) -> u32 {
    *print_job.get(print_job.len() / 2).unwrap()
}

fn solve_part_1(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename)?;

    let page_rules = build_page_rules(&input.rules);

    let mut total = 0u32;
    for (job_idx, print_job) in input.print_jobs.iter().enumerate() {

        if let Some(_) = find_violation(print_job, &page_rules) {
            debug!("Job {job_idx} is bad");
        } else {
            debug!("Job {job_idx} is good");
            total += get_middle_page(&print_job)
        }

    }

    Ok(total)
}

fn solve_part_2(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename)?;
    let page_rules = build_page_rules(&input.rules);

    let mut total = 0u32;
    for print_job in input.print_jobs.iter() {
        if let Some(fixed_job) = fix_violations(print_job, &page_rules) {
            total += get_middle_page(&fixed_job)
        }
    }

    Ok(total)
}

fn main() -> Result<()> {
    simple_log::quick!("info");

    info!("Result part 1: {}", solve_part_1("src/day_05/input.txt")?);
    info!("Result part 2: {}", solve_part_2("src/day_05/input.txt")?);
    Ok(())
}

// part 2: 9985 too high

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
        let result = solve_part_1("src/day_05/test_input.txt").unwrap();
        assert_eq!(result, 143);
    }

    #[test]
    fn solve_test_input_2() {
        let result = solve_part_2("src/day_05/test_input.txt").unwrap();
        assert_eq!(result, 123);
    }
}