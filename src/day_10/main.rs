use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter, Write};
use anyhow::{Result};
use aoc_utils::map::{Direction, Map, Position};

#[macro_use]
extern crate simple_log;

type InputMap = Map<InputItem>;

#[derive(Debug, PartialEq, Clone)]
pub enum InputItem {
    Empty,
    Tile(usize),
}

impl Display for InputItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InputItem::Empty => f.write_char('.')?,
            InputItem::Tile(height) => std::fmt::Display::fmt(&height, f)?,
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SearchMapItem {
    Unvisited,
    Tile(usize),
}

impl Display for SearchMapItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchMapItem::Unvisited => f.write_char('.')?,
            SearchMapItem::Tile(height) => std::fmt::Display::fmt(&height, f)?,
        }
        Ok(())
    }
}

mod parse {
    use aoc_parse::{parser, prelude::*};
    use anyhow::{Result, Context};
    use std::fs::read_to_string;
    use aoc_utils::map::Map;
    use crate::{InputItem, InputMap};

    pub fn parse_input(filename: &str) -> Result<InputMap> {
        let parser = parser!(lines({
            d:digit => InputItem::Tile(d),
            "." => InputItem::Empty
        }+));

        let raw_data = read_to_string(filename)?;
        let objects = parser.parse(&raw_data).context("parse error")?;

        Ok(Map::from_nested_vecs(objects))
    }

}

fn solve_part_1(filename: &str) -> Result<u32> {
    let map = parse::parse_input(filename)?;
    debug!("Start map: {map}");

    // search all start positions
    let trail_heads: Vec<Position> = map.iter_objects()
        .filter_map(|(position, item)| {
            match item {
                InputItem::Tile(height) if *height == 0 => Some(position),
                _ => None
            }
        })
        .collect();

    let sum_of_scores: usize = trail_heads.into_iter()
        .map(|trail_head| find_reachable_peaks(&map, trail_head).len())
        .sum();

    Ok(sum_of_scores as u32)
}

fn find_reachable_peaks(map: &InputMap, start: Position) -> HashSet<Position> {
    let mut search_map = Map::with_size(map.max_x + 1, map.max_y + 1, SearchMapItem::Unvisited);
    search_map.set(&start, SearchMapItem::Tile(0));
    let mut to_explore: Vec<Position> = vec![start];

    let mut reachable_peaks: HashSet<Position> = HashSet::new();

    while let Some(current) = to_explore.pop() {
        let current_height = match map.get(&current).unwrap() {
            InputItem::Tile(height) => height,
            _ => panic!("Can't handle empty tile at  {:?}", current)
        };

        for direction in &vec![Direction::Up, Direction::Right, Direction::Down, Direction::Left] {
            if let Some(neighbor_position) = map.new_position(&current, direction) {
                let neighbor = map.get(&neighbor_position).expect("all map items have a height");
                match neighbor {
                    // we only follow the path "up"
                    InputItem::Tile(neighbor_height) if *neighbor_height == *current_height + 1 => {

                        // mark the tile as discovered as a valid step from current position
                        search_map.set(&neighbor_position, SearchMapItem::Tile(*neighbor_height));

                        if *neighbor_height < 9 {
                            // we have not reached the peak, we need to explore further from there
                            to_explore.insert(0, neighbor_position);
                        } else if *neighbor_height == 9 {
                            // found a peak, insert into set dor deduplication
                            reachable_peaks.insert(neighbor_position);
                        }
                    }
                    _ => continue
                }
            }

        }
    }

    reachable_peaks
}

fn solve_part_2(filename: &str) -> Result<u32> {
    let map = parse::parse_input(filename)?;
    debug!("Start map: {map}");

    // search all start positions
    let trail_heads: Vec<Position> = map.iter_objects()
        .filter_map(|(position, item)| {
            match item {
                InputItem::Tile(height) if *height == 0 => Some(position),
                _ => None
            }
        })
        .collect();

    // get all *reachable* peaks
    let reachable_peaks: HashSet<Position> = trail_heads.into_iter()
        .flat_map(|trail_head| find_reachable_peaks(&map, trail_head))
        .collect();

    debug!("Reachable peaks: {:?}", reachable_peaks);


    let mut total_ratings_by_head: HashMap<Position, usize> = HashMap::new();

    for peak in reachable_peaks {
        let partial_ratings = compute_rating(&map, peak);
        for (position, rating) in partial_ratings {
            total_ratings_by_head.entry(position)
                .and_modify(|r| *r += rating)
                .or_insert(rating);
        }
    }

    debug!("{:?}", total_ratings_by_head);
    let total = total_ratings_by_head.into_iter().map(|(_, rating)| rating as u32).sum();

    Ok(total)
}

fn compute_rating(map: &InputMap, peak: Position) -> HashMap<Position, usize> {
    // we start our search at the given peak
    let mut search_map = Map::with_size(map.max_x + 1, map.max_y + 1, SearchMapItem::Unvisited);
    search_map.set(&peak, SearchMapItem::Tile(1));
    let mut to_explore: Vec<Position> = vec![peak];

    // keep track of rations per trail head (since we can reach multiple heads from a peak)
    let mut ratings_per_trail_head: HashMap<Position, usize> = HashMap::new();

    while let Some(current) = to_explore.pop() {
        let current_height = match map.get(&current).unwrap() {
            InputItem::Tile(height) => height,
            _ => panic!("Can't handle empty tile at {:?}", current)
        };

        let current_rating = match search_map.get(&current).unwrap() {
            SearchMapItem::Tile(rating) => rating.clone(),
            _ => panic!("Can't handle empty tile at {:?}", current)
        };

        for direction in &vec![Direction::Up, Direction::Right, Direction::Down, Direction::Left] {

            if let Some(neighbor_position) = map.new_position(&current, direction) {
                let neighbor = map.get(&neighbor_position).expect("all map items have a height");
                match neighbor {

                    // we only follow the path "down"
                    InputItem::Tile(neighbor_height) if *neighbor_height == *current_height - 1 => {

                        // check if we have visited that tile or if it's new and computer new intermediate rating
                        let (search_map_item, needs_insert) = match search_map.get(&neighbor_position).unwrap() {
                            SearchMapItem::Unvisited => (SearchMapItem::Tile(current_rating), true),
                            SearchMapItem::Tile(existing_rating) => (SearchMapItem::Tile(current_rating + existing_rating), false)
                        };
                        search_map.set(&neighbor_position, search_map_item);

                        if *neighbor_height > 0 && needs_insert {
                            // we have not reached the trail head, we need to explore further from there
                            to_explore.insert(0, neighbor_position);
                        } else if *neighbor_height == 0 {
                            // found a trail head insert/update rating for this head
                            ratings_per_trail_head.entry(neighbor_position)
                                .and_modify(|existing_rating| *existing_rating += current_rating)
                                .or_insert(current_rating);
                        }
                    }
                    _ => continue
                }
            }
        }

    }

    ratings_per_trail_head
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
        assert_eq!(result, 81);
    }

    #[test]
    fn solve_test_input_2_simple_01() {
        let result = solve_part_2("src/day_10/test_input_simple_01.txt").unwrap();
        assert_eq!(result, 3);
    }
    #[test]
    fn solve_test_input_2_simple_02() {
        let result = solve_part_2("src/day_10/test_input_simple_02.txt").unwrap();
        assert_eq!(result, 13);
    }
}