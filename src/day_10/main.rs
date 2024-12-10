use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter, Write};
use std::future::poll_fn;
use anyhow::{Result};
use aoc_parse::prelude::aoc_parse;
use aoc_utils::map::{Direction, Map, Position};

#[macro_use]
extern crate simple_log;

type InputMap = Map<usize>;
type SearchMap = Map<Object>;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Unvisited,
    Visited,
    Tile(usize),
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Unvisited => f.write_char('.')?,
            Object::Visited => f.write_char('x')?,
            Object::Tile(height) => std::fmt::Display::fmt(&height, f)?,
        }
        Ok(())
    }
}

mod parse {
    use aoc_parse::{parser, prelude::*};
    use anyhow::{Result, Context};
    use std::fs::read_to_string;
    use aoc_utils::map::Map;
    use crate::{InputMap};

    pub fn parse_input (filename: &str) -> Result<InputMap> {
        let parser = parser!(lines(digit+));

        let raw_data = read_to_string(filename)?;
        let objects = parser.parse(&raw_data).context("parse error")?;

        Ok(Map::from_nested_vecs(objects))
    }

}

fn solve_part_1(filename: &str) -> Result<u32> {
    let map = parse::parse_input(filename)?;
    debug!("Start map: {map}");

    // search all start positions
    let mut trail_heads: Vec<Position> = map.iter_objects()
        .filter_map(|(position, item)| {
            if *item == 0 {
                Some(position)
            } else {
                None
            }
        })
        .collect();

    let sum_of_scores = trail_heads.into_iter()
        .fold(0usize,|total, start| {
            let head_score = compute_head_score(&map, start);
            debug!("Head score for {:?} is {}", start, head_score);
            total + head_score
        });

    Ok(sum_of_scores as u32)
}

fn compute_head_score(map: &InputMap, start: Position) -> usize {
    let mut search_map = Map::with_size(map.max_x + 1, map.max_y + 1, Object::Unvisited);
    search_map.set(&start, Object::Tile(0));

    let mut to_explore: Vec<Position> = vec![start];
    let mut reachable_peaks: HashSet<Position> = HashSet::new();

    let directions = vec![Direction::Up, Direction::Right, Direction::Down, Direction::Left];

    while let Some(current) = to_explore.pop() {
        let current_height = match map.get(&current) {
            None => panic!("Nothing in map at {:?}", current),
            Some(height) => height
        };

        for direction in &directions {
            if let Some(neighbor_position) = map.new_position(&current, direction) {
                let neighbor_height = map.get(&neighbor_position).expect("all map items have a height");
                if *neighbor_height == *current_height + 1 {
                    // this is a path "up"
                    search_map.set(&neighbor_position, Object::Tile(*neighbor_height));
                     if *neighbor_height < 9 {
                         // we have not reached the peak, need to explore further from there
                         to_explore.insert(0, neighbor_position);
                     } else if *neighbor_height == 9 {
                         // found a peak, insert into set dor deduplication
                         reachable_peaks.insert(neighbor_position);
                     }
                }

            }

        }
    }

    debug!("Search map: {search_map}");

    reachable_peaks.len()
}

fn solve_part_2(filename: &str) -> Result<u32> {
    // let map: = parse::parse_input(filename)?;
    // debug!("{:?}", input);

    todo!()
}

fn main() -> Result<()> {
    simple_log::quick!("info");

    info!("Result part 1: {}", solve_part_1("src/day_10/input.txt")?);
    info!("Result part 2: {}", solve_part_2("src/day_10/input.txt")?);
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
        let result = solve_part_1("src/day_10/test_input.txt").unwrap();
        assert_eq!(result, 36);
    }

    #[test]
    fn solve_test_input_2() {
        let result = solve_part_2("src/day_10/test_input.txt").unwrap();
        assert_eq!(result, 42);
    }
}