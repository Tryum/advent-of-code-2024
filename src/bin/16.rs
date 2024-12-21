use std::collections::{HashMap, HashSet};

use strum::{EnumIter, IntoEnumIterator};

advent_of_code::solution!(16);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn mv(&self, v: (isize, isize)) -> Position {
        Position {
            x: self.x.saturating_add_signed(v.0),
            y: self.y.saturating_add_signed(v.1),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    pub fn left(&self) -> Direction {
        match self {
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::North => Direction::West,
        }
    }

    pub fn right(&self) -> Direction {
        match self {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
        }
    }

    pub fn dir(&self) -> (isize, isize) {
        match self {
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::North => (0, -1),
        }
    }
}

#[derive(EnumIter)]
enum Actions {
    Forward,
    Left,
    Right,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut start = Position { x: 0, y: 0 };
    let mut exit = Position { x: 0, y: 0 };

    let mut walls = HashSet::new();

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == 'S' {
                start = Position { x, y };
            } else if c == 'E' {
                exit = Position { x, y };
            } else if c == '#' {
                walls.insert(Position { x, y });
            }
        }
    }

    let mut visits = HashMap::new();
    let mut queue = Vec::new();

    let mut result = u32::MAX;

    visits.insert((start, Direction::East), 0);
    queue.push((start, Direction::East, 0));
    while let Some((pos, dir, score)) = queue.pop() {
        for a in Actions::iter() {
            let (pos, dir, score) = match a {
                Actions::Forward => {
                    let score = score + 1;
                    let pos = pos.mv(dir.dir());
                    (pos, dir, score)
                }
                Actions::Left => {
                    let score = score + 1000;
                    let dir = dir.left();
                    (pos, dir, score)
                }
                Actions::Right => {
                    let score = score + 1000;
                    let dir = dir.right();
                    (pos, dir, score)
                }
            };

            if walls.contains(&pos) {
                continue;
            }

            if score > result {
                continue;
            }

            if let Some(v) = visits.get(&(pos, dir)) {
                if *v <= score {
                    continue;
                }
            }
            if pos == exit {
                if score < result {
                    result = score;
                }
                continue;
            }
            visits.insert((pos, dir), score);
            queue.push((pos, dir, score));
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut start = Position { x: 0, y: 0 };
    let mut exit = Position { x: 0, y: 0 };

    let mut walls = HashSet::new();

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == 'S' {
                start = Position { x, y };
            } else if c == 'E' {
                exit = Position { x, y };
            } else if c == '#' {
                walls.insert(Position { x, y });
            }
        }
    }

    let mut visits = HashMap::new();
    let mut queue = Vec::new();
    let mut sits = HashSet::<Position>::new();

    let mut result = u32::MAX;

    visits.insert((start, Direction::East), 0);
    queue.push((start, Direction::East, 0, vec![start]));
    while let Some((pos, dir, score, trail)) = queue.pop() {
        for a in Actions::iter() {
            let (pos, dir, score) = match a {
                Actions::Forward => {
                    let score = score + 1;
                    let pos = pos.mv(dir.dir());
                    (pos, dir, score)
                }
                Actions::Left => {
                    let score = score + 1000;
                    let dir = dir.left();
                    (pos, dir, score)
                }
                Actions::Right => {
                    let score = score + 1000;
                    let dir = dir.right();
                    (pos, dir, score)
                }
            };

            if walls.contains(&pos) {
                continue;
            }

            if score > result {
                continue;
            }

            if let Some(v) = visits.get(&(pos, dir)) {
                if *v < score {
                    continue;
                }
            }
            let mut trail = trail.clone();
            trail.push(pos);
            if pos == exit {
                match score.cmp(&result) {
                    std::cmp::Ordering::Less => {
                        sits = HashSet::new();
                        sits.extend(&trail);
                        result = score;
                    }
                    std::cmp::Ordering::Equal => {
                        sits.extend(&trail);
                    }
                    std::cmp::Ordering::Greater => {}
                }
                continue;
            }

            visits.insert((pos, dir), score);
            queue.push((pos, dir, score, trail));
        }
        queue.sort_by(|a, b| b.2.cmp(&a.2));
    }
    Some(sits.len().try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
