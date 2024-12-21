use core::fmt;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(15);

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum EntityType {
    Wall,
    Box,
}

impl fmt::Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityType::Wall => write!(f, "#"),
            EntityType::Box => write!(f, "O"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    pub x: usize,
    pub y: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Position {
    fn mv(&self, mov: (i32, i32)) -> Position {
        Position {
            x: self.x.wrapping_add_signed(mov.0 as isize),
            y: self.y.wrapping_add_signed(mov.1 as isize),
        }
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Right => write!(f, ">"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
        }
    }
}

impl Direction {
    fn dir(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

fn push_and_move(map: &mut HashMap<Position, EntityType>, bot_pos: &mut Position, dir: Direction) {
    let dest = bot_pos.mv(dir.dir());
    if let Some(e) = map.get_mut(&dest) {
        match e {
            EntityType::Wall => {
                return;
            }
            EntityType::Box => {
                let mut next_pos = dest;
                loop {
                    next_pos = next_pos.mv(dir.dir());
                    if let Some(e) = map.get(&next_pos) {
                        match e {
                            EntityType::Wall => {
                                return;
                            }
                            EntityType::Box => {}
                        }
                    } else {
                        // free space !
                        map.remove(&dest);
                        map.insert(next_pos, EntityType::Box);
                        break;
                    }
                }
            }
        }
    }

    bot_pos.x = dest.x;
    bot_pos.y = dest.y;
}

fn _check_for_box(map: &HashMap<Position, EntityType>, p: &Position) -> Option<Position> {
    let left_pos = Position {
        x: p.x.saturating_sub(1),
        y: p.y,
    };

    if let Some(EntityType::Box) = map.get(p) {
        Some(*p)
    } else if let Some(EntityType::Box) = map.get(&left_pos) {
        Some(left_pos)
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut parse_map = true;

    let mut bot = Position { x: 0, y: 0 };

    let mut map = HashMap::new();

    let mut directions = Vec::new();

    for (y, l) in input.lines().enumerate() {
        if parse_map {
            if l.is_empty() {
                parse_map = false;
            } else {
                for (x, c) in l.chars().enumerate() {
                    match c {
                        '#' => {
                            map.insert(Position { x, y }, EntityType::Wall);
                        }
                        'O' => {
                            map.insert(Position { x, y }, EntityType::Box);
                        }
                        '@' => {
                            bot.x = x;
                            bot.y = y;
                        }
                        _ => {}
                    }
                }
            }
        } else {
            for c in l.chars() {
                match c {
                    '^' => directions.push(Direction::Up),
                    '>' => directions.push(Direction::Right),
                    'v' => directions.push(Direction::Down),
                    '<' => directions.push(Direction::Left),
                    _ => {}
                }
            }
        }
    }

    for d in directions {
        push_and_move(&mut map, &mut bot, d);
    }

    let mut result = 0;

    for (pos, e) in map {
        if e == EntityType::Box {
            result += 100 * pos.y + pos.x;
        }
    }

    Some(result as u32)
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Entity {
    t: EntityType,
    p: Position,
}

impl Entity {
    pub fn collide(&self, p: &Position) -> bool {
        match self.t {
            EntityType::Wall => *p == self.p,
            EntityType::Box => {
                let left = p.mv(Direction::Left.dir());
                *p == self.p || left == self.p
            }
        }
    }

    pub fn _render(&self, p: &Position) {
        match self.t {
            EntityType::Wall => print!("#"),
            EntityType::Box => {
                if *p == self.p {
                    print!("[");
                } else {
                    print!("]");
                }
            }
        }
    }
}

fn _render2(width: usize, height: usize, entities: &[Entity], bot: &Position) {
    print!("   ");
    for i in 0..width {
        let x = i / 10;
        print!("{}", x);
    }
    println!();
    print!("   ");
    for i in 0..width {
        let x = i % 10;
        print!("{}", x);
    }
    println!();

    for y in 0..height {
        print!("{:0>2}:", y);
        'row: for x in 0..width {
            for e in entities {
                if e.collide(&Position { x, y }) {
                    e._render(&Position { x, y });
                    continue 'row;
                }
            }
            if bot == (&Position { x, y }) {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn cascade_boxes(entities: &[Entity], p: Position, dir: &Direction) -> Result<HashSet<Entity>, ()> {
    let mut colliders = Vec::new();

    let mut result = HashSet::new();

    for e in entities {
        if e.collide(&p) {
            colliders.push(e);
        }
    }

    for c in colliders {
        if c.t == EntityType::Wall {
            return Err(());
        } else {
            result.insert(*c);

            let left_cell = c.p;
            let right_cell = c.p.mv(Direction::Right.dir());
            match dir {
                Direction::Up => {
                    result.extend(cascade_boxes(entities, left_cell.mv(dir.dir()), dir)?);
                    result.extend(cascade_boxes(entities, right_cell.mv(dir.dir()), dir)?);
                }
                Direction::Right => {
                    result.extend(cascade_boxes(entities, right_cell.mv(dir.dir()), dir)?);
                }
                Direction::Down => {
                    result.extend(cascade_boxes(entities, left_cell.mv(dir.dir()), dir)?);
                    result.extend(cascade_boxes(entities, right_cell.mv(dir.dir()), dir)?);
                }
                Direction::Left => {
                    result.extend(cascade_boxes(entities, left_cell.mv(dir.dir()), dir)?);
                }
            }
        }
    }

    Ok(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut parse_map = true;

    let mut _width;
    let mut _height;

    let mut bot = Position { x: 0, y: 0 };

    let mut directions = Vec::new();

    let mut entities = Vec::new();

    for (y, l) in input.lines().enumerate() {
        if parse_map {
            if l.is_empty() {
                parse_map = false;
                _height = y;
            } else {
                _width = 2 * l.len();
                for (x, c) in l.chars().enumerate() {
                    match c {
                        '#' => {
                            entities.push(Entity {
                                t: EntityType::Wall,
                                p: Position { x: 2 * x, y },
                            });
                            entities.push(Entity {
                                t: EntityType::Wall,
                                p: Position { x: 2 * x + 1, y },
                            });
                        }
                        'O' => {
                            entities.push(Entity {
                                t: EntityType::Box,
                                p: Position { x: 2 * x, y },
                            });
                        }
                        '@' => {
                            bot.x = 2 * x;
                            bot.y = y;
                        }
                        _ => {}
                    }
                }
            }
        } else {
            for c in l.chars() {
                match c {
                    '^' => directions.push(Direction::Up),
                    '>' => directions.push(Direction::Right),
                    'v' => directions.push(Direction::Down),
                    '<' => directions.push(Direction::Left),
                    _ => {}
                }
            }
        }
    }

    //println!("Initial State: ");
    //_render2(_width, _height, &entities, &bot);

    #[allow(clippy::unused_enumerate_index)]
    'instr: for (_i, d) in directions.iter().enumerate() {
        //println!("{}, Move {}:", _i, &d);
        let next_pos = bot.mv(d.dir());
        let boxes_to_move = match cascade_boxes(&entities, next_pos, d) {
            Ok(boxes) => boxes,
            Err(_) => {
                //_render2(_width, _height, &entities, &bot);
                continue 'instr;
            }
        };
        bot = next_pos;
        for b in boxes_to_move {
            let index = entities.iter().position(|e| e == &b).unwrap();
            entities.remove(index);
            let new_pos = b.p.mv(d.dir());
            entities.push(Entity { t: b.t, p: new_pos });
        }

        //_render2(width, height, &entities, &bot);
    }

    let mut result = 0;
    for e in entities {
        if e.t == EntityType::Box {
            result += e.p.x + e.p.y * 100;
        }
    }
    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_snall() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
