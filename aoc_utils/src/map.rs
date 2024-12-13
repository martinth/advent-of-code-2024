use std::collections::VecDeque;
use std::fmt::{Display, Formatter, Write};

pub type Position = (usize, usize);

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    pub fn turn_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Map<T> {
    pub objects: Vec<Vec<T>>,
    pub max_x: usize,
    pub max_y: usize,
}

impl <T> Map<T> {

    pub fn from_nested_vecs(objects: Vec<Vec<T>>) -> Map<T> {
        let max_x = objects.get(0).unwrap().len() - 1;
        let max_y = objects.len() - 1;

        Map {
            objects,
            max_x,
            max_y,
        }
    }

    pub fn with_size(size_x: usize, size_y: usize, fill_item: T) -> Map<T> where T: Clone {
        let mut nested_vecs: Vec<Vec<T>> = Vec::with_capacity(size_y);
        for _y in 0..nested_vecs.capacity() {
            nested_vecs.push(vec![fill_item.clone(); size_x]);
        }
        Map::from_nested_vecs(nested_vecs)
    }

    /// get reference to item at given position
    pub fn get(&self, (x, y): &Position) -> Option<&T> {
        self.objects.get(*y)
            .and_then(|row| row.get(*x))
    }

    /// get mutable reference to item at given position
    pub fn get_mut(&mut self, (x, y):&Position) -> Option<&mut T> {
        self.objects.get_mut(*y)
            .and_then(|row| row.get_mut(*x))
    }

    pub fn set(&mut self, position: &Position, item: T)  {
        let cell_ref = self.get_mut(position).unwrap();
        *cell_ref = item
    }

    /// Calculate the new position if walking from the given position in given direction.
    /// Returns None if the step would leave the map
    pub fn new_position(&self, position: &Position, direction: &Direction) -> Option<Position> {
        let delta = match direction {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0)
        };

        // calculate new position based on direction but respect map boundaries
        let new_x = position.0.checked_add_signed(delta.0 as isize);
        let new_y = position.1.checked_add_signed(delta.1 as isize);
        return match (new_x, new_y) {
            (Some(x), Some(y)) if x <= self.max_x && y <= self.max_y => Some((x, y)),
            _ => None
        }
    }

    /// Create an iterator that walks the map in reading order
    pub fn iter_objects(&self) -> PositionIterator<T> {
        PositionIterator::for_map(self)
    }

    pub fn breath_first_search(&self, start_position: Position) -> BreathFirstSearch<T> {
        BreathFirstSearch::for_map(self, start_position)
    }
}

// nice map display
impl <T> Display for Map<T> where T: Display {
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

pub struct PositionIterator<'m, T> {
    map: &'m Map<T>,
    cur_y: usize,
    cur_x: usize
}

impl <T> PositionIterator<'_, T> {
    fn for_map(map: & Map<T>) -> PositionIterator<'_, T> {
        PositionIterator {
            map,
            cur_y: 0,
            cur_x: 0
        }
    }
}

impl <'m, T> Iterator for PositionIterator<'m, T> {
    type Item = (Position, &'m T);

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.map.objects.get(self.cur_y);
        let item = row.and_then(|row| row.get(self.cur_x));

        if item.is_none() {
            None
        } else {
            let result = Some(((self.cur_x, self.cur_y), item.unwrap()));
            self.cur_x += 1;
            if self.cur_x >= row.unwrap().len() {
                self.cur_x = 0;
                self.cur_y += 1;
            }
            result
        }
    }
}

pub struct BreathFirstSearch<'m, T> {
    map: &'m Map<T>,
    visited: Map<bool>,
    to_explore: VecDeque<Position>,
}

impl <T> BreathFirstSearch<'_, T> {
    fn for_map(map: & Map<T>, start_position: Position) -> BreathFirstSearch<'_, T> {
        let mut to_explore = VecDeque::new();
        to_explore.push_back(start_position);
        BreathFirstSearch {
            map,
            visited: Map::with_size(map.max_x + 1, map.max_y + 1, false),
            to_explore
        }
    }

    pub fn explore<F>(&mut self, mut accept_fn: F)
        where F: FnMut(&Position, &T) -> bool
    {
        while let Some(current) = self.to_explore.pop_front() {
            // if the position has been checked, we can skip it
            if *self.visited.get(&current).unwrap() {
                continue
            }

            if let Some(item) = self.map.get(&current) {

                // check with callback and mark as visited
                let is_accepted = accept_fn(&current, item);
                self.visited.set(&current, true);

                if is_accepted {
                    // if the current node is okay, we also add the neighbors
                    for direction in &vec![Direction::Up, Direction::Right, Direction::Down, Direction::Left] {
                        if let Some(neighbor_position) = self.map.new_position(&current, direction) {

                            // add neighbors but only if they have not been visited
                            if *self.visited.get(&neighbor_position).unwrap() {
                                continue
                            }
                            self.to_explore.push_back(neighbor_position)
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc_parse::macros::lines;
    use aoc_parse::{parser, Parser};
    use anyhow::{Result, Context};
    use super::*;

    #[derive(Debug)]
    pub enum Object {
        Empty,
        Something
    }

    impl Display for Object {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Object::Empty => f.write_char('.')?,
                Object::Something => f.write_char('#')?,
            }
            Ok(())
        }
    }

    fn parse(raw_data: &str) -> Result<Map<Object>> {
        let parser = parser!(lines({
            "." => Object::Empty,
            "#" => Object::Something,
        }+));

        let objects = parser.parse(&raw_data).context("parse error")?;

        Ok(Map::from_nested_vecs(objects))
    }

    fn get_test_map() -> &'static str {
       ".....\n\
       .#.#.\n\
       ..#..\n\
       .#.#.\n\
       ....."
    }

    #[test]
    fn round_trip_parsing_and_display() {
        let result = parse(get_test_map()).expect("should parse");
        assert_eq!(format!("{}", result), format!("\n{}\n", get_test_map()));
    }

}
