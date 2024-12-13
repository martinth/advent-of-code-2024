use std::cmp::PartialEq;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};
use anyhow::{Result};
use aoc_utils::map::{Direction, Map, Position};

#[macro_use]
extern crate simple_log;

#[derive(Debug, PartialEq)]
pub struct Plant(char);

impl Display for Plant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.0)
    }
}

type InputMap = Map<Plant>;

#[derive(Debug)]
pub struct Input {
    map: InputMap
}

mod parse {
    use aoc_parse::{parser, prelude::*};
    use anyhow::{Result, Context};
    use std::fs::read_to_string;
    use aoc_utils::map::Map;
    use crate::{Input, Plant};

    pub fn parse_input(filename: &str) -> Result<Input> {
        let parser = parser!(lines({
            plant_type:upper => Plant(plant_type)
        }+));

        let raw_data = read_to_string(filename)?;
        let objects = parser.parse(&raw_data).context("parse error")?;
        let map = Map::from_nested_vecs(objects);

        Ok(Input {
            map
        })
    }
}


fn solve_part_1(filename: &str) -> Result<usize> {
    let input = parse::parse_input(filename)?;
    debug!("Input map: {}", input.map);

    let mut all_different_region: HashSet<Region> = HashSet::new();
    let mut all_positions_seen:  HashSet<Position> = HashSet::new();
    for (position, _) in input.map.iter_objects() {
        if all_positions_seen.contains(&position) {
            // shortcut
            continue;
        }

        let region = determine_region(&input.map, position);

        if !all_different_region.contains(&region) {
            debug!("New region for {:?}: {:?}", position, region);
            all_positions_seen.extend(&region.positions);
            all_different_region.insert(region);
        }
    }

    debug!("Found {} regions", all_different_region.len());

    let total = all_different_region.iter()
        .map(|region: &Region| calc_area(region) * calc_perimeter_length(&input.map, region))
        .sum();

    Ok(total)
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Region {
    plant_type: char,
    positions: Vec<Position>
}

impl Region {
    fn new<'p>(plant_type: char, mut positions: Vec<Position>) -> Region {
        positions.sort();
        Region {
            plant_type,
            positions
        }
    }
}

fn determine_region(map: &InputMap, start_position: Position) -> Region {
    let current_plant = map.get(&start_position).unwrap();
    let mut positions: HashSet<Position> = HashSet::new();
    map.breath_first_search(start_position).explore(|pos, item| {
        if item == current_plant {
            positions.insert(pos.clone());
            true
        } else {
            false
        }
    });
    
    Region::new(current_plant.0, positions.into_iter().collect())
}

fn calc_area(region: &Region) -> usize {
    region.positions.len()
}

/// For each block we can determine how many perimeters it contributes:
/// The perimeter_contribution is always neighbors_count - 4
fn calc_perimeter_length(map: &InputMap, region: &Region) -> usize {

    let mut perimeters = 0usize;
    for region_point in &region.positions {

        let mut neighbors_same_region = 0usize;
        for direction in &vec![Direction::Up, Direction::Right, Direction::Down, Direction::Left] {
            if let Some(neighbor) = map.new_position(&region_point, direction) {
                if let Some(plant) = map.get(&neighbor) {
                    if plant.0 == region.plant_type {
                        neighbors_same_region += 1;
                    }
                }
            }
        }
        assert!(neighbors_same_region <= 4);

        perimeters += 4 - neighbors_same_region
    }

    perimeters
}

fn solve_part_2(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename)?;
    debug!("{:?}", input);

    todo!()
}

fn main() -> Result<()> {
    simple_log::quick!("info");

    info!("Result part 1: {}", solve_part_1("src/day_12/input.txt")?);
    info!("Result part 2: {}", solve_part_2("src/day_12/input.txt")?);
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
        let result = solve_part_1("src/day_12/test_input.txt").unwrap();
        assert_eq!(result, 1930);
    }

    #[test]
    fn solve_test_input_2() {
        let result = solve_part_2("src/day_12/test_input.txt").unwrap();
        assert_eq!(result, 42);
    }

}