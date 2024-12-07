use std::cmp::PartialEq;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};
use anyhow::{Result, bail, anyhow};
use itertools::Itertools;
use rayon::prelude::*;

#[macro_use]
extern crate simple_log;

type Position = (usize, usize);

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum ViewDirection {
    Up,
    Right,
    Down,
    Left
}

impl ViewDirection {
    fn turn_clockwise(&self) -> ViewDirection {
        match self {
            ViewDirection::Up => ViewDirection::Right,
            ViewDirection::Right => ViewDirection::Down,
            ViewDirection::Down => ViewDirection::Left,
            ViewDirection::Left => ViewDirection::Up,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Empty,
    Item,
    Guard(ViewDirection),
    Visited,
    Blockage
}

impl Default for Object {
    fn default() -> Self {
        Object::Empty
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    objects: Vec<Vec<Object>>,
    max_x: usize,
    max_y: usize,
}

impl Map {

    /// get reference to item at given position
    fn get(&self, (x, y): (usize, usize)) -> Option<&Object> {
        self.objects.get(y)
            .and_then(|row| row.get(x))
    }

    /// get mutable reference to item at given position
    fn get_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut Object> {
        self.objects.get_mut(y)
            .and_then(|row| row.get_mut(x))
    }

    /// Calculate the new position if walking from the given position in given direction.
    /// Returns None if the step would leave the map
    fn new_position(&self, position: Position, view_direction: &ViewDirection) -> Option<Position> {
        let delta = match view_direction {
            ViewDirection::Up => (0, -1),
            ViewDirection::Right => (1, 0),
            ViewDirection::Down => (0, 1),
            ViewDirection::Left => (-1, 0)
        };

        // calculate new position based on direction but respect map boundaries
        let new_x = position.0.checked_add_signed(delta.0 as isize);
        let new_y = position.1.checked_add_signed(delta.1 as isize);
        return match (new_x, new_y) {
            (Some(x), Some(y)) if x <= self.max_x && y <= self.max_y => Some((x, y)),
            _ => None
        }
    }
}

// nice map display
impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('\n')?;
        for row in &self.objects {
            for object in row {
                object.fmt(f)?
            }
            f.write_char('\n')?
        }
        Ok(())
    }
}
impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Empty => f.write_char('.')?,
            Object::Item => f.write_char('#')?,
            Object::Guard(direction) => match direction {
                ViewDirection::Up => f.write_char('^')?,
                ViewDirection::Right => f.write_char('>')?,
                ViewDirection::Down => f.write_char('v')?,
                ViewDirection::Left => f.write_char('<')?,
            }
            Object::Visited => f.write_char('X')?,
            Object::Blockage => f.write_char('O')?,
        }
        Ok(())
    }
}

mod parse {
    use aoc_parse::{parser, prelude::*};
    use anyhow::{Result, Context};
    use std::fs::read_to_string;
    use crate::{Map, Object, ViewDirection};

    pub fn parse_input(filename: &str) -> Result<Map> {
        let parser = parser!(lines({
            "." => Object::Empty,
            "#" => Object::Item,
            "^" => Object::Guard(ViewDirection::Up),
            ">" => Object::Guard(ViewDirection::Right),
            "<" => Object::Guard(ViewDirection::Left),
            "v" => Object::Guard(ViewDirection::Down),
            "X" => Object::Visited
        }+));

        let raw_data = read_to_string(filename)?;
        let objects = parser.parse(&raw_data).context("parse error")?;

        let max_x = objects.get(0).unwrap().len() - 1;
        let max_y = objects.len();

        Ok(Map {
            objects,
            max_x,
            max_y,
        })
    }
}


/// Find the guard on the map and the direction it's looking at.
fn find_guard(map: &Map) -> Result<(Position, ViewDirection)> {
    for (y, row) in map.objects.iter().enumerate() {
        for (x, object) in row.iter().enumerate() {
            match object {
                Object::Guard(view_direction) => {
                    return Ok(((x, y), view_direction.clone()))
                }
                _ => continue
            }
        }
    }

    bail!("No guard found")
}


/// Find the position at which the guard will run into an object (not the position of the object itself!).
/// Returns None if there is nothing blocking the guard.
fn find_blocking_object(start_position: Position, view_direction: &ViewDirection, map: &Map) -> Option<Position> {
     let mut current: Position = start_position.clone();

    // cast ray in that position
    while let Some(next_position) = map.new_position(current, view_direction) {

        // if there is an item there, the next guard position is the current one
        if let Some(object_at) = map.get(next_position) {
            if *object_at == Object::Item || *object_at == Object::Blockage {
                return Some(current);
            }
            current = next_position;
        } else {
            return None
        }

    }

    None
}

/// Move the guard from a current position to the target.
fn move_guard(map: &mut Map, current_position: Position, new_position: Position) -> Result<ViewDirection> {
    // grab the guard from the map and put default in place, we need to do the replacement in two steps since
    // we can not have to mutable references into the map so we can not directly swap 1 for 1
    let guard = std::mem::take(map.get_mut(current_position).expect("position is valid"));

    let (new_guard, new_direction) = match guard {
        Object::Guard(current_direction) => {
            // create new guard that has already turned
            let new_direction = current_direction.turn_clockwise();
            let new_guard = Object::Guard(new_direction.clone());
            (new_guard, new_direction)
        },
        _ => bail!("Guard not currently at position {current_position:?}")
    };

    // place new guard on map
    let _ = std::mem::replace(map.get_mut(new_position).expect("position is valid"), new_guard);

    Ok(new_direction)
}

/// Mark the path from a starting position until we reach the guard. This traces the path it must
/// have walked.
fn mark_visited(map: &mut Map, start: Position, direction: &ViewDirection) {
    let mut current = start;
    loop {
        let object_at = map.get_mut(current).expect("position is valid");
        match object_at {
            Object::Guard(_) => break,
            _ => {
                let _ = std::mem::replace(map.get_mut(current).expect("position is valid"), Object::Visited);
                current = map.new_position(current, direction).unwrap()
            }
        }
    }
}

/// Mark the exit path of the guard from it's current position.
fn mark_exit(map: &mut Map, start: Position, direction: &ViewDirection) {
    let mut current = start;
    while let Some(pos_ref) = map.get_mut(current) {
        let _ = std::mem::replace(pos_ref, Object::Visited);
        if let Some(new_pos) = map.new_position(current, direction) {
            current = new_pos
        } else {
            break
        }
    }
}

/// Simulation step
fn do_step(map: &mut Map, current_position: Position, current_direction: &ViewDirection) -> Result<Option<(Position, ViewDirection)>> {
    return if let Some(new_guard_position) = find_blocking_object(current_position, current_direction, &map) {
        let new_direction = move_guard(map, current_position, new_guard_position)?;
        mark_visited(map, current_position, current_direction);
        //debug!("{}", map);
        Ok(Some((new_guard_position, new_direction)))
    } else {
        mark_exit(map, current_position, current_direction);
        Ok(None)
    }
}

/// Find an exit path but optionally check for endless loops.
fn find_exit_path(map: &mut Map, loop_detection: bool) -> Result<Option<Vec<Position>>> {
    let (mut position, mut view_direction) = find_guard(&map)?;

    let mut already_visited: HashSet<(Position, ViewDirection)> = HashSet::new();

    // simulate steps the guard takes until it leaved the map
    while let Some(next_step) =  do_step(map, position, &view_direction)? {
        if loop_detection {
            if already_visited.iter().contains(&next_step) {
                return Err(anyhow!("Loop detected, already visited {next_step:?}"))
            }
            already_visited.insert(next_step.clone());
        }

        position = next_step.0;
        view_direction = next_step.1;

    }

    let mut result: Vec<Position> = Vec::new();
    for (y, row) in map.objects.iter().enumerate() {
        for (x, object) in row.iter().enumerate() {
            match object {
                Object::Visited => result.push((x, y)),
                _ => continue
            }
        }
    }

    Ok(Some(result))
}

fn solve_part_1(filename: &str) -> Result<u32> {
    let mut map = parse::parse_input(filename)?;
    debug!("Starting map {}", map);

    let path = find_exit_path(&mut map, false).unwrap().unwrap();
    debug!("Done! {}", map);

    Ok(path.len() as u32)
}

fn solve_part_2(filename: &str) -> Result<u32> {
    let map = parse::parse_input(filename)?;

    // get the original exit path of the guard
    let mut original_map = map.clone();
    let exit_path = find_exit_path(&mut original_map, false).unwrap().unwrap();

    // loop over all positions (in parallel for speeeed)
    let looping_paths = exit_path.into_par_iter()
        .filter(|position| match map.get(*position) {
            Some(Object::Guard(_)) => false, // skip guard position
            Some(_) => true, // falls thought
            None => panic!("Exit path positions should always be valid")
        })
        .fold(|| 0_u32, |blockage_count: u32, position: Position| {
            // create a blockage at the position
            let mut map_with_blockage = map.clone();
            let _ = std::mem::replace(map_with_blockage.get_mut(position).expect("position is valid"), Object::Blockage);

            // if the path now loops that is a valid blockage
            if find_exit_path(&mut map_with_blockage, true).is_err() {
                blockage_count + 1
            } else {
                blockage_count
            }
        })
        .sum::<u32>();

    Ok(looping_paths)
}

fn main() -> Result<()> {
    simple_log::quick!("info");

    info!("Result part 1: {}", solve_part_1("src/day_06/input.txt")?);
    info!("Result part 2: {}", solve_part_2("src/day_06/input.txt")?);
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
        let result = solve_part_1("src/day_06/test_input.txt").unwrap();
        assert_eq!(result, 41);
    }

    #[test]
    fn solve_test_input_2() {
        let result = solve_part_2("src/day_06/test_input.txt").unwrap();
        assert_eq!(result, 6);
    }
}