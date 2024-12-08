use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

advent_of_code::solution!(6);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy)]
struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Direction {
    pub fn rotate_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    pub fn value(&self) -> Vec2 {
        match self {
            Direction::Up => Vec2 { x: 0, y: -1 },
            Direction::Right => Vec2 { x: 1, y: 0 },
            Direction::Down => Vec2 { x: 0, y: 1 },
            Direction::Left => Vec2 { x: -1, y: 0 },
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

impl Add<Vec2> for Position {
    type Output = Position;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x.wrapping_add_signed(rhs.x as isize),
            y: self.y.wrapping_add_signed(rhs.y as isize),
        }
    }
}

struct Map {
    width: usize,
    height: usize,
    start_pos: Position,
    obstacles: HashSet<(usize, usize)>,
}

fn parse_map(input: &str) -> Map {
    let lines = input.lines().collect::<Vec<&str>>();

    let height = lines.len();
    let width = lines[0].len();

    let mut pos = Position { x: 0, y: 0 };

    let mut obstacles = HashSet::new();

    for i in 0..lines.len() {
        let line = lines[i];
        for j in 0..line.len() {
            match line.chars().nth(j) {
                Some('#') => {
                    obstacles.insert((j, i));
                }
                Some('^') => {
                    pos.x = j;
                    pos.y = i;
                }
                _ => {}
            }
        }
    }
    Map {
        width,
        height,
        start_pos: pos,
        obstacles,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_map(input);

    let positions = run_map(&map).unwrap();

    Some(positions.len() as u32)
}

fn run_map(map: &Map) -> Option<HashSet<(usize, usize)>> {
    let mut positions = HashMap::<(usize, usize), HashSet<Direction>>::new();
    let mut old_positions = HashSet::<(usize, usize)>::new();

    let mut pos = map.start_pos;
    let mut dir = Direction::Up;

    loop {
        old_positions.insert(pos.tuple());
        let entry = positions.entry(pos.tuple()).or_default();
        if entry.contains(&dir) {
            break None;
        }
        entry.insert(dir);
        let next_pos = pos + dir.value();
        if map.obstacles.contains(&next_pos.tuple()) {
            dir = dir.rotate_right();
        } else {
            pos = next_pos;
        }
        if pos.x >= map.width || pos.y >= map.height {
            break Some(old_positions);
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_map(input);
    let mut loops = 0;

    let old_positions = run_map(&map).unwrap();

    for blocker in old_positions.iter() {
        //println!("Simulating block at {:?}", blocker);
        if map.start_pos.tuple() == *blocker {
            continue;
        }
        let mut new_obstacles = map.obstacles.clone();
        new_obstacles.insert(*blocker);
        if run_map(&Map {
            width: map.width,
            height: map.height,
            start_pos: map.start_pos,
            obstacles: new_obstacles,
        })
        .is_none()
        {
            loops += 1;
        }
    }

    Some(loops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
