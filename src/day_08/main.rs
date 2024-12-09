use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};
use anyhow::{Result};
use itertools::Itertools;
use aoc_utils::map::Position;

#[macro_use]
extern crate simple_log;

type Map = aoc_utils::map::Map<Object>;

#[derive(Debug, Clone)]
pub enum Object {
    Empty,
    Antenna { frequency: char },
    AntiNode
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Empty => f.write_char('.')?,
            Object::Antenna { frequency} => f.write_char(*frequency)?,
            Object::AntiNode => f.write_char('#')?,
        }
        Ok(())
    }
}


mod parse {
    use aoc_parse::{parser, prelude::*};
    use anyhow::{Result, Context};
    use std::fs::read_to_string;
    use crate::{Map, Object};

    pub fn parse_input(filename: &str) -> Result<Map> {
        let parser = parser!(lines({
            "." => Object::Empty,
            "#" => Object::AntiNode,
            freq:alnum => Object::Antenna { frequency: freq },
        }+));

        let raw_data = read_to_string(filename)?;
        let objects = parser.parse(&raw_data).context("parse error")?;

        Ok(Map::from_nested_vecs(objects))
    }
}

fn solve_part_1(filename: &str) -> Result<u32> {
    let map = parse::parse_input(filename)?;

    let antennas_by_frequency = find_all_antennas(&map);

    // we use a set to de-duplicate
    let antinodes: HashSet<Position> = antennas_by_frequency.into_iter()
        .flat_map(|(_, antennas)| {
            compute_all_antinodes(antennas, &map)
        }).collect();

    Ok(antinodes.len() as u32)
}

fn find_all_antennas(map: &Map) -> HashMap<&char, Vec<Position>> {
    map.objects.iter().enumerate()
        .flat_map(|(y, row)|
        row.iter().enumerate().map(move |(x, obj)| (x, y, obj))
        )
        .filter_map(|(x, y, obj)| {
            match obj {
                Object::Antenna { frequency} => Some((frequency, (x, y))),
                _ => None
            }
        }).into_group_map()
}

fn compute_all_antinodes(antennas: Vec<Position>, map: &Map) -> Vec<Position> {
      antennas.iter().combinations(2)
        .flat_map(|pair| compute_antinodes(pair.get(0).unwrap(), pair.get(1).unwrap(), map))
        .collect()
}

fn compute_antinodes(a: &Position, b: &Position, map: &Map) -> Vec<Position> {

    // compute the next antinode position for one axis to valid map coordinates
    let checked_compute_axis = |first: usize, second: usize, max: usize| -> Option<usize> {
        // checked_add makes sure it's > 0 and the following check makes sure it on the map
        second.checked_add_signed(second as isize - first as isize)
            .and_then(|new_val| if new_val > max { None } else { Some(new_val) })
    };

    // compute the next antinode position for two positions to valid map coordinates
    let checked_compute = |a: &Position, b: &Position| -> Option<Position> {
        // position is only valid if both axis values are
        match (checked_compute_axis(a.0, b.0, map.max_x), checked_compute_axis(a.1, b.1, map.max_y)) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None
        }
    };

    let mut v: Vec<Position> = Vec::new();
    if let Some(an_0) = checked_compute(a, b) {
        v.push(an_0);
    }
    if let Some(an_1) = checked_compute(b, a) {
        v.push(an_1);
    }

    return v;
}

fn solve_part_2(filename: &str) -> Result<u32> {

    let map = parse::parse_input(filename)?;

    let antennas_by_frequency = find_all_antennas(&map);

    // we use a set to de-duplicate
    let harmonics: HashSet<Position> = antennas_by_frequency.into_iter()
        .flat_map(|(_, antennas)| {
             compute_all_harmonics(antennas, &map)
        }).collect();

    Ok(harmonics.len() as u32)

}

fn compute_all_harmonics(antennas: Vec<Position>, map: &Map) -> Vec<Position> {
    antennas.iter().combinations(2)
        .flat_map(|pair| compute_harmonics(pair.get(0).unwrap(), pair.get(1).unwrap(), map))
        .collect()
}

fn compute_harmonics(a: &Position, b: &Position, map: &Map) -> Vec<Position> {

    // compute the next harmonic given a position and a delta if it is valid
    let checked_pos_compute = |position: &Position, delta: &(isize, isize)| -> Option<Position> {
        let (new_x, new_y) = (
            position.0.checked_add_signed(delta.0).and_then(|new_val| if new_val > map.max_x { None } else { Some(new_val) }),
            position.1.checked_add_signed(delta.1).and_then(|new_val| if new_val > map.max_y { None } else { Some(new_val) }),
        );
        match (new_x, new_y) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None
        }

    };

    let mut harmonics: Vec<Position> = Vec::new();
    // the two antennas are also harmonics
    harmonics.push(a.clone());
    harmonics.push(b.clone());

    let delta_a_b = (a.0 as isize - b.0 as isize, a.1 as isize - b.1 as isize);
    let mut current = a.clone();
    while let Some(harmonic) = checked_pos_compute(&current, &delta_a_b) {
        harmonics.push(harmonic);
        current = harmonic
    }

    let delta_b_a = (b.0 as isize - a.0 as isize, b.1 as isize - a.1 as isize);
    let mut current = b.clone();
    while let Some(harmonic) = checked_pos_compute(&current, &delta_b_a) {
        harmonics.push(harmonic);
        current = harmonic
    }

    return harmonics;
}

fn main() -> Result<()> {
    simple_log::quick!("info");

    info!("Result part 1: {}", solve_part_1("src/day_08/input.txt")?);
    info!("Result part 2: {}", solve_part_2("src/day_08/input.txt")?);
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
        let result = solve_part_1("src/day_08/test_input.txt").unwrap();
        assert_eq!(result, 14);
    }

    #[test]
    fn solve_test_input_2() {
        let result = solve_part_2("src/day_08/test_input.txt").unwrap();
        assert_eq!(result, 34);
    }

    #[test]
    fn solve_test_input_2_example() {
        let result = solve_part_2("src/day_08/test_input_part_2_example.txt").unwrap();
        assert_eq!(result, 9);
    }

}