use core::fmt;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(20);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position {
    pub x: usize,
    pub y: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Position {
    pub fn mv(&self, dir: (i32, i32)) -> Position {
        let x = self.x.wrapping_add_signed(dir.0 as isize);
        let y = self.y.wrapping_add_signed(dir.1 as isize);
        Position { x, y }
    }
}

fn get_cheats(
    visits: &HashMap<Position, usize>,
    cheat_len: usize,
) -> HashMap<(Position, Position), usize> {
    let path: Vec<_> = visits
        .iter()
        .sorted_by(|a, b| Ord::cmp(&a.1, &b.1))
        .collect();

    let mut shortcuts = HashMap::<(Position, Position), usize>::new();

    for i in 0..path.len() {
        for j in i + 1..path.len() {
            let (p1, s1) = path[i];
            let (p2, s2) = path[j];
            let manathan_dist = p1.x.abs_diff(p2.x) + p1.y.abs_diff(p2.y);
            if manathan_dist <= cheat_len && s2 - s1 > manathan_dist {
                shortcuts.insert((*p1, *p2), s2 - s1 - manathan_dist);
            }
        }
    }

    shortcuts
}

pub fn part_one(input: &str) -> Option<u32> {
    let _ = input;

    let save_condition = if cfg!(test) { 20 } else { 100 };

    let mut start_pos = Position { x: 0, y: 0 };
    let mut exit_pos = Position { x: 0, y: 0 };

    let mut walls = HashSet::<Position>::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start_pos = Position { x, y };
                }
                'E' => {
                    exit_pos = Position { x, y };
                }
                '.' => {}
                '#' => {
                    walls.insert(Position { x, y });
                }
                _ => panic!("unexpected character !"),
            }
        }
    }

    println!("Start pos : {}", start_pos);
    println!("Exit pos : {}", exit_pos);

    let mut queue = Vec::new();
    queue.push((start_pos, 0));

    let mut visits = HashMap::new();
    visits.insert(start_pos, 0);

    'main: while let Some((pos, steps)) = queue.pop() {
        for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let new_pos = pos.mv(dir);
            if new_pos == exit_pos {
                visits.insert(new_pos, steps + 1);
                break 'main;
            }
            if !walls.contains(&new_pos) && !visits.contains_key(&new_pos) {
                visits.insert(new_pos, steps + 1);
                queue.push((new_pos, steps + 1));
            }
        }
    }

    let mut result = 0;

    let cheats = get_cheats(&visits, 2);

    let mut savings = HashMap::<usize, u32>::new();
    for (_, s) in cheats {
        let entry = savings.entry(s).or_default();
        *entry += 1;
        if s >= save_condition {
            result += 1;
        }
    }

    for s in savings.iter().sorted() {
        println!("There are {} cheats that save {} picoseconds", s.1, s.0);
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let _ = input;

    let save_condition = if cfg!(test) { 70 } else { 100 };

    let mut start_pos = Position { x: 0, y: 0 };
    let mut exit_pos = Position { x: 0, y: 0 };

    let mut walls = HashSet::<Position>::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start_pos = Position { x, y };
                }
                'E' => {
                    exit_pos = Position { x, y };
                }
                '.' => {}
                '#' => {
                    walls.insert(Position { x, y });
                }
                _ => panic!("unexpected character !"),
            }
        }
    }

    println!("Start pos : {}", start_pos);
    println!("Exit pos : {}", exit_pos);

    let mut queue = Vec::new();
    queue.push((start_pos, 0));

    let mut visits = HashMap::new();
    visits.insert(start_pos, 0);

    'main: while let Some((pos, steps)) = queue.pop() {
        for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let new_pos = pos.mv(dir);
            if new_pos == exit_pos {
                visits.insert(new_pos, steps + 1);
                break 'main;
            }
            if !walls.contains(&new_pos) && !visits.contains_key(&new_pos) {
                visits.insert(new_pos, steps + 1);
                queue.push((new_pos, steps + 1));
            }
        }
    }

    let mut result = 0;

    let cheats = get_cheats(&visits, 20);

    let mut savings = HashMap::<usize, u32>::new();
    for (_, s) in cheats {
        if cfg!(test) {
            let entry = savings.entry(s).or_default();
            *entry += 1;
        }
        if s >= save_condition {
            result += 1;
        }
    }

    if cfg!(test) {
        for s in savings {
            println!("There are {} cheats that save {} picoseconds", s.1, s.0);
        }
    }

    if cfg!(test) {
        println!(
            "There are {} cheats that save at least {} picoseconds",
            result, save_condition
        );
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }
}
