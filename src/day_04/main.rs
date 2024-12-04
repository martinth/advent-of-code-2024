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

        Needle { needle: needle, needle_rev: needle_rev }
    }

    fn len(&self) -> usize {
        self.needle.len()
    }

    fn eq(&self, other: &[char]) -> bool {
        self.needle == *other || self.needle_rev == *other
    }

    fn eq_start_or_end(&self, c: &char) -> bool {
        c == self.needle.first().unwrap() || c == self.needle_rev.first().unwrap()
    }

    fn is_match(&self, haystack: &[&char], index: usize) -> bool {
        let slice: Option<Vec<char>> = haystack
            .get(index..index + self.len())
            .map(|char_slice| char_slice.into_iter().map(|c| **c).collect());

        debug!("is_match for {:?}", slice);

        slice
            .map(|slice| self.eq(&slice))
            .unwrap_or(false)
    }
}

fn count_matches(haystack: &[&char], needle: &Needle) -> u32 {
    let mut matches = 0u32;

    let mut base_index = 0;
    while base_index < haystack.len() {
        let idx = base_index;
        // debug!("Checking haystack at {}", idx);
        let end = haystack.get(idx);
        if end.is_none() {
            break
        }

        // check for potential match
        if needle.eq_start_or_end(end.unwrap()) {
            debug!("Potential match {:?} at {}", end, idx);
            // check if truly found match
            if needle.is_match(haystack, idx) {
                debug!("Real match {:?} at {}", end, idx);
                matches += 1
                // count it
            }

        }
        // fallen through: move search forward
        base_index += 1;
    }


    matches
}

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

/// Yield a vec based on a start point and a direction from the matrix
fn get_elements<T>(v: &Vec<Vec<T>>, start: (usize, usize), direction: Direction) -> Vec<&T>
where
    T: Debug
{
    let mut x = start.0;
    let mut y = start.1;
    let mut result: Vec<&T> = Vec::new();
    let (dx, dy) = direction.offsets();

    while let Some(item) = v.get(y).and_then(|row| row.get(x)) {
        result.push(item);

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

/// Generate all vectors that cover the given matrix vertically (left-to-right and right-to-left)
fn verticals<T>(v: &Vec<Vec<T>>) -> Vec<Vec<&T>> where T: Debug  {
    assert!(!v.is_empty());

    let mut verticals: Vec<Vec<&T>> = Vec::new();

    for y in 0..v.len() {
        let row = v.get(y).unwrap();
        if y == 0 {
            // we want to start at every item in the top row
            for x in 0..row.len() {
                debug!("start at {}/{}: {:?}", x, y, &v[y][x]);
                verticals.push(get_elements(v, (x, y), Direction::TopLeftToBottomRight));
                verticals.push(get_elements(v, (x, y), Direction::TopRightToBottomLeft));
            }
        } else {
            // every next row we only want the first and the last
            debug!("start at {}/{}: {:?}", 0, y, &v[y][0]);
            verticals.push(get_elements(v, (0, y), Direction::TopLeftToBottomRight));
            verticals.push(get_elements(v, (row.len() - 1, y), Direction::TopRightToBottomLeft));
        }
    }


    verticals
}

fn solve_part_1(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename)?;

    let needle = Needle::new("XMAS");
    let mut total= 0u32;

    for row in &input.data {
        let row: Vec<&char> = row.iter().collect();
        total += count_matches(&row, &needle);
    }

    let transposed = transpose(&input.data);
    for col in transposed {
        total += count_matches(&col, &needle);
    }

    let verticals = verticals(&input.data);
    for vertical in verticals {
        total += count_matches(&vertical, &needle);
    }

    Ok(total)
}

fn solve_part_2(filename: &str) -> Result<u32> {
    let input = parse::parse_input(filename)?;
    debug!("{:?}", input);
    
    // generate all the verticals ltr and rtl
    
    // search for all XMAS and mark their middle index (relative to org matrix)
    
    // check where the middle indices in both lists match and count how many there are

    todo!()
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
    use crate::{count_matches, Direction, Needle, solve_part_1, solve_part_2, verticals, get_elements};

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
        assert_eq!(result, 42);
    }

    #[test]
    fn check_simple_cases() {
        let needle = Needle::new("XMAS");

        assert_eq!(count_matches(&"XMAS".chars().collect::<Vec<char>>(), &needle), 1);
        assert_eq!(count_matches(&"SAMX".chars().collect::<Vec<char>>(), &needle), 1);
        assert_eq!(count_matches(&"MMMSXXMASM".chars().collect::<Vec<char>>(), &needle), 1);
        assert_eq!(count_matches(&"XMASAMXAMM".chars().collect::<Vec<char>>(), &needle), 2);
        assert_eq!(count_matches(&"MAMMMXMMMM".chars().collect::<Vec<char>>(), &needle), 0);
    }

    #[test]
    fn generate_verticals_square() {
        let input = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ];

        let verticals = verticals(&input);
        for v in &verticals {
            debug!("{:?}", v)
        }


        // left to right
        assert_eq!(verticals.contains(&vec!['a', 'e', 'i']), true);
        assert_eq!(verticals.contains(&vec!['b', 'f']), true);
        assert_eq!(verticals.contains(&vec!['c']), true);
        assert_eq!(verticals.contains(&vec!['d', 'h']), true);
        assert_eq!(verticals.contains(&vec!['g']), true);

        // right to left
        assert_eq!(verticals.contains(&vec!['c', 'e', 'g']), true);
        assert_eq!(verticals.contains(&vec!['b', 'd']), true);
        assert_eq!(verticals.contains(&vec!['a']), true);
        assert_eq!(verticals.contains(&vec!['f', 'h']), true);
        assert_eq!(verticals.contains(&vec!['i']), true);

        assert_eq!(verticals.len(), 10);
    }

    #[test]
    fn generate_verticals_rectangle() {
        let input = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
        ];

        let verticals = verticals(&input);

        // left to right
        assert_eq!(verticals.contains(&vec!['a', 'e']), true);
        assert_eq!(verticals.contains(&vec!['b', 'f']), true);
        assert_eq!(verticals.contains(&vec!['c']), true);
        assert_eq!(verticals.contains(&vec!['d']), true);

        // right to left
        assert_eq!(verticals.contains(&vec!['b', 'd']), true);
        assert_eq!(verticals.contains(&vec!['c', 'e']), true);
        assert_eq!(verticals.contains(&vec!['a']), true);
        assert_eq!(verticals.contains(&vec!['f']), true);

        assert_eq!(verticals.len(), 8);
    }

    #[test]
    fn check_yield_verticals() {
        let input = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ];

        assert_eq!(vec!['a', 'e', 'i'], get_elements(&input, (0, 0), &Direction::TopLeftToBottomRight));
        assert_eq!(vec!['b', 'f'], get_elements(&input, (1, 0), &Direction::TopLeftToBottomRight));
        assert_eq!(vec!['c'], get_elements(&input, (2, 0), &Direction::TopLeftToBottomRight));
        assert_eq!(vec!['d', 'h'], get_elements(&input, (0, 1), &Direction::TopLeftToBottomRight));
        assert_eq!(vec!['e', 'i'], get_elements(&input, (1, 1), &Direction::TopLeftToBottomRight));
        assert_eq!(vec!['g'], get_elements(&input, (0, 2), &Direction::TopLeftToBottomRight));

        assert_eq!(vec!['a'], get_elements(&input, (0, 0), &Direction::TopRightToBottomLeft));
        assert_eq!(vec!['b', 'd'], get_elements(&input, (1, 0), &Direction::TopRightToBottomLeft));
        assert_eq!(vec!['c', 'e', 'g'], get_elements(&input, (2, 0), &Direction::TopRightToBottomLeft));
        assert_eq!(vec!['d'], get_elements(&input, (0, 1), &Direction::TopRightToBottomLeft));
        assert_eq!(vec!['e', 'g'], get_elements(&input, (1, 1), &Direction::TopRightToBottomLeft));

        assert_eq!(vec!['a', 'b', 'c'], get_elements(&input, (0, 0), &Direction::Right));
        assert_eq!(vec!['d', 'e', 'f'], get_elements(&input, (0, 1), &Direction::Right));

        assert_eq!(vec!['a', 'd', 'g'], get_elements(&input, (0, 0), &Direction::Down));
    }
}