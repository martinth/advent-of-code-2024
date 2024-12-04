use std::collections::HashSet;
use std::fmt::Debug;
use anyhow::{Result};

#[macro_use]
extern crate simple_log;

#[derive(Debug)]
pub struct Input {
    data: Vec<Vec<char>>
}

mod parse {
    use aoc_parse::{parser, prelude::*};
    use anyhow::{Result, Context};
    use std::fs::read_to_string;
    use crate::Input;

    pub fn parse_input(filename: &str) -> Result<Input> {
        let parser = parser!(lines(upper+));

        let raw_data = read_to_string(filename)?;
        let raw_parsed = parser.parse(&raw_data).context("parse error")?;

        Ok(Input {
            data: raw_parsed
        })
    }
}

/// Represents a sequence of chars we want to search in another sequence but forwards and backwards.
#[derive(Debug)]
struct Needle {
    needle: Vec<char>,
    needle_rev: Vec<char>,
}

impl Needle {
    fn new(s: &str) -> Self {
        assert!(s.len() > 0);

        let needle: Vec<char> = s.chars().collect();
        let mut needle_rev: Vec<char> = s.chars().collect();
        needle_rev.reverse();

        Needle { needle, needle_rev }
    }

    fn len(&self) -> usize {
        self.needle.len()
    }

    /// check if either direction matches the given slice
    fn eq(&self, other: &[char]) -> bool {
        self.needle == *other || self.needle_rev == *other
    }

    /// check if the needle starts or ends at the given position
    fn is_match(&self, haystack: &[&char], index: usize) -> bool {
        let slice: Option<Vec<char>> = haystack
            .get(index..index + self.len())
            .map(|char_slice| char_slice.into_iter().map(|c| **c).collect());

        slice
            .map(|slice| self.eq(&slice))
            .unwrap_or(false)
    }
}

/// find the number of (overlapping) matches of the needle in the haystack
fn count_matches(haystack: &[&char], needle: &Needle) -> u32 {
    let mut matches = 0u32;

    for idx in 0..haystack.len() {
        if needle.is_match(haystack, idx) {
            debug!("Real match {:?} at {} in {:?}", needle, idx, haystack);
            matches += 1
            // count it
        }
    }

    matches
}



/// transpose the vec of vec without cloning the elements
fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<&T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<&T>>()
        })
        .collect()
}

/// A direction in which we can generate verticals from a matrix
enum Direction {
    TopLeftToBottomRight,
    TopRightToBottomLeft,
}

impl Direction {
    fn offsets(&self) -> (i8, i8) {
        match self {
            Direction::TopLeftToBottomRight => (1, 1),
            Direction::TopRightToBottomLeft => (-1, 1),
        }
    }

}

/// Yield a vertical based on a start point and a direction from the matrix
fn create_vertical<'v, T>(v: &'v Vec<Vec<T>>, start: (usize, usize), direction: &Direction) -> Vec<&'v T> {
    let (mut x, mut y) = start;
    let (dx, dy) = direction.offsets();

    let mut result: Vec<&T> = Vec::new();
    while let Some(item) = v.get(y).and_then(|row| row.get(x)) {
        result.push(item);

        // check if the new index would go out of bounds
        if let Some(new_x) = x.checked_add_signed(dx as isize) {
            x = new_x;
        } else {
            break
        }
        if let Some(new_y) = y.checked_add_signed(dy as isize) {
            y = new_y;
        } else {
            break
        }
    }

    result
}

/// generate all verticals in the given direction
fn verticals<T>(v: &Vec<Vec<T>>, direction: Direction) -> Vec<Vec<&T>> {
    assert!(!v.is_empty());

    let mut verticals: Vec<Vec<&T>> = Vec::new();

    for y in 0..v.len() {
        let row = v.get(y).unwrap();
        if y == 0 {
            // we want to start at every item in the top row
            for x in 0..row.len() {
                verticals.push(create_vertical(v, (x, y), &direction));
            }
        } else {
            let start = match direction {
                Direction::TopLeftToBottomRight => (0, y),
                Direction::TopRightToBottomLeft => (row.len() - 1, y)
            };
            // every next row we only want the first and or the last
            verticals.push(create_vertical(v, start, &direction));
        }
    }

    verticals
}

/// Generate all verticals that cover the given matrix vertically (left-to-right and right-to-left)
fn all_verticals<T>(v: &Vec<Vec<T>>) -> Vec<Vec<&T>> {
    let mut all_verticals: Vec<Vec<&T>> = Vec::new();

    all_verticals.extend(verticals(v, Direction::TopLeftToBottomRight));
    all_verticals.extend(verticals(v, Direction::TopRightToBottomLeft));

    all_verticals
}

fn solve_part_1(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename)?;

    let needle = Needle::new("XMAS");
    let mut total= 0u32;

    // matches in horizontal direction
    for row in &input.data {
        let row: Vec<&char> = row.iter().collect();
        total += count_matches(&row, &needle);
    }

    // matches in vertical direction
    let transposed = transpose(&input.data);
    for col in transposed {
        total += count_matches(&col, &needle);
    }

    // matches on both verticals
    let verticals = all_verticals(&input.data);
    for vertical in verticals {
        total += count_matches(&vertical, &needle);
    }

    Ok(total)
}

/// A char from the input matrix together with it's original position.
#[derive(Debug)]
struct PosChar {
    c: char,
    position: (usize, usize)
}

/// find the middle positions of the needle and extract the original information
fn find_middle_positions<'h>(haystack: &'h[&PosChar], needle: &Needle) -> Vec<(usize, usize)> {
    assert_eq!(needle.len() % 2, 1);
    let middle_offset = needle.len() / 2;

    let mut middle_positions = vec![];
    for idx in 0..haystack.len() {
        let haystack_chars: Vec<&char> = haystack.iter().map(|pc| &pc.c).collect();
        if needle.is_match(&haystack_chars, idx) {
            let middle = haystack[idx + middle_offset].position;
            middle_positions.push(middle)
        }
    }

    middle_positions
}

fn solve_part_2(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename)?;

    let needle = Needle::new("MAS");

    // generate a matrix of PosChar so we can keep track of the original positions the chars
    // in the verticals came from
    let with_positions: Vec<Vec<PosChar>> = input.data
        .into_iter()
        .enumerate()
        .map(|(y, row)| {
            row.into_iter().enumerate().map(|(x, c)| {
                PosChar { c, position: (x, y)}
            }).collect()
        })
        .collect();


    // generate all the verticals ltr and rtl and find the middle position of the needle in all
    let middles_ltr: HashSet<(usize, usize)> = verticals(&with_positions, Direction::TopLeftToBottomRight)
        .into_iter()
        .flat_map(|vertical| find_middle_positions(&vertical, &needle))
        .collect();

    let middles_rtl: HashSet<(usize, usize)> = verticals(&with_positions, Direction::TopRightToBottomLeft)
        .into_iter()
        .flat_map(|vertical| find_middle_positions(&vertical, &needle))
        .collect();

    // check where the middles in both lists match and count how many there are
    let total = middles_ltr.intersection(&middles_rtl).count();

    Ok(total.try_into().unwrap())
}

fn main() -> Result<()> {
    simple_log::quick!("info");

    info!("Result part 1: {}", solve_part_1("src/day_04/input.txt")?);
    info!("Result part 2: {}", solve_part_2("src/day_04/input.txt")?);
    Ok(())
}


#[cfg(test)]
mod tests {
    use ctor::ctor;
    use crate::{Direction, solve_part_1, solve_part_2, create_vertical};

    #[ctor]
    fn init() {
        simple_log::quick!("debug");
    }

    #[test]
    fn solve_test_input_1() {
        let result = solve_part_1("src/day_04/test_input.txt").unwrap();
        assert_eq!(result, 18);
    }

    #[test]
    fn solve_test_input_2() {
        let result = solve_part_2("src/day_04/test_input.txt").unwrap();
        assert_eq!(result, 9);
    }

    #[test]
    fn check_yield_verticals() {
        let input = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ];

        assert_eq!("aei", get_elements_string(&input, (0, 0), Direction::TopLeftToBottomRight));
        assert_eq!("bf", get_elements_string(&input, (1, 0), Direction::TopLeftToBottomRight));
        assert_eq!("c", get_elements_string(&input, (2, 0), Direction::TopLeftToBottomRight));
        assert_eq!("dh", get_elements_string(&input, (0, 1), Direction::TopLeftToBottomRight));
        assert_eq!("ei", get_elements_string(&input, (1, 1), Direction::TopLeftToBottomRight));
        assert_eq!("g", get_elements_string(&input, (0, 2), Direction::TopLeftToBottomRight));

        assert_eq!("a", get_elements_string(&input, (0, 0), Direction::TopRightToBottomLeft));
        assert_eq!("bd", get_elements_string(&input, (1, 0), Direction::TopRightToBottomLeft));
        assert_eq!("ceg", get_elements_string(&input, (2, 0), Direction::TopRightToBottomLeft));
        assert_eq!("d", get_elements_string(&input, (0, 1), Direction::TopRightToBottomLeft));
        assert_eq!("eg", get_elements_string(&input, (1, 1), Direction::TopRightToBottomLeft));
    }

    fn get_elements_string(v: &Vec<Vec<char>>, start: (usize, usize), direction: Direction) -> String {
        let result = create_vertical(v, start, &direction);

        result.iter().map(|c| **c).collect()
    }
}