use core::fmt;
use std::collections::{HashMap, HashSet};

use regex::Regex;

advent_of_code::solution!(18);

fn get_world_size() -> (usize, usize) {
    if cfg!(test) {
        (7, 7)
    } else {
        (71, 71)
    }
}

fn get_exit() -> Position {
    if cfg!(test) {
        Position { x: 6, y: 6 }
    } else {
        Position { x: 70, y: 70 }
    }
}

fn get_bytes() -> usize {
    if cfg!(test) {
        12
    } else {
        1024
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    fn mv(&self, dir: (isize, isize)) -> Option<Position> {
        let x = self.x.checked_add_signed(dir.0);
        let y = self.y.checked_add_signed(dir.1);
        if let Some(x) = x {
            if let Some(y) = y {
                return Some(Position { x, y });
            }
        }
        None
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"(\d*),(\d*)").unwrap();
    let mut bytes = Vec::new();
    for [x, y] in re
        .captures_iter(input)
        .map(|c| c.extract().1.map(|x| x.parse::<usize>().unwrap()))
    {
        bytes.push(Position { x, y });
    }

    let _bytes: HashSet<_> = bytes[0..get_bytes()].iter().cloned().collect();

    let exit = get_exit();

    let world_size = get_world_size();

    let start_pos = Position { x: 0, y: 0 };
    let mut visits = HashMap::new();
    visits.insert(start_pos, 0);
    let mut queue = Vec::new();
    queue.push((start_pos, 0));
    while let Some((pos, steps)) = queue.pop() {
        if pos == exit {
            continue;
        }
        let new_steps = steps + 1;
        for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            if let Some(new_pos) = pos.mv(dir) {
                if new_pos.x < world_size.0
                    && new_pos.y < world_size.1
                    && !_bytes.contains(&new_pos)
                {
                    let entry = visits.entry(new_pos).or_insert(u32::MAX);
                    if new_steps < *entry {
                        *entry = new_steps;
                        queue.push((new_pos, new_steps));
                    }
                }
            }
        }
    }

    visits.get(&exit).cloned()
}

pub fn part_two(input: &str) -> Option<String> {
    let re = Regex::new(r"(\d*),(\d*)").unwrap();
    let mut bytes = Vec::new();
    for [x, y] in re
        .captures_iter(input)
        .map(|c| c.extract().1.map(|x| x.parse::<usize>().unwrap()))
    {
        bytes.push(Position { x, y });
    }

    let mut wall_bytes: HashSet<_> = bytes[0..get_bytes()].iter().cloned().collect();

    let exit = get_exit();

    let world_size = get_world_size();

    'falling_bytes: for b in &bytes[get_bytes()..] {
        wall_bytes.insert(*b);
        let start_pos = Position { x: 0, y: 0 };
        let mut visits = HashMap::new();
        visits.insert(start_pos, 0);
        let mut queue = Vec::new();
        queue.push((start_pos, 0));
        while let Some((pos, steps)) = queue.pop() {
            if pos == exit {
                continue 'falling_bytes;
            }
            let new_steps = steps + 1;
            for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                if let Some(new_pos) = pos.mv(dir) {
                    if new_pos.x < world_size.0
                        && new_pos.y < world_size.1
                        && !wall_bytes.contains(&new_pos)
                    {
                        let entry = visits.entry(new_pos).or_insert(u32::MAX);
                        if new_steps < *entry {
                            *entry = new_steps;
                            queue.push((new_pos, new_steps));
                        }
                    }
                }
            }
        }
        return Some(b.to_string());
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
