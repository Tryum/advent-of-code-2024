use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt,
    rc::Rc,
};

advent_of_code::solution!(12);

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Position {
    pub fn mv(&self, dir: &(i32, i32)) -> Option<Position> {
        let x = self.x.checked_add_signed(dir.0 as isize);
        let y = self.y.checked_add_signed(dir.1 as isize);

        // Making clippy happy.
        if let Some(x) = x {
            y.map(|y| Position { x, y })
        } else {
            None
        }
    }
}

pub trait FromChar {
    fn from_char(c: char) -> Self;
}

#[derive(Clone)]
struct Cell {
    plant_type: char,
}

impl FromChar for Cell {
    fn from_char(c: char) -> Self {
        Cell { plant_type: c }
    }
}

struct Region<'a> {
    plot_type: char,
    plots: HashMap<Position, &'a Cell>,
}

struct Map<T> {
    cells: Vec<T>,
    width: usize,
    height: usize,
}

pub struct MapIterator<'a, T> {
    map: &'a Map<T>,
    current: usize,
}

impl<'a, T> Iterator for MapIterator<'a, T> {
    type Item = (Position, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.map.cells.len() {
            let item = &self.map.cells[self.current];
            let x = self.current % self.map.width;
            let y = self.current / self.map.width;
            self.current += 1;
            Some((Position { x, y }, item))
        } else {
            None
        }
    }
}

impl<T: FromChar + Clone> Map<T> {
    pub fn _new(width: usize, height: usize, default_char: char) -> Self {
        let cells = vec![T::from_char(default_char); width * height];
        Map {
            cells,
            width,
            height,
        }
    }

    pub fn new_from_input(input: &str) -> Self {
        let mut cells = Vec::new();
        let width = input.lines().next().map(|line| line.len()).unwrap_or(0);
        for line in input.lines() {
            for c in line.chars() {
                cells.push(T::from_char(c));
            }
        }
        let height = cells.len() / width;
        Map {
            cells,
            width,
            height,
        }
    }

    pub fn iter(&self) -> MapIterator<'_, T> {
        MapIterator {
            map: self,
            current: 0,
        }
    }

    pub fn get(&self, pos: &Position) -> Option<&T> {
        if pos.x < self.width && pos.y < self.height {
            Some(&self.cells[pos.y * self.width + pos.x])
        } else {
            None
        }
    }

    pub fn _get_mut(&mut self, pos: &Position) -> Option<&mut T> {
        if pos.x < self.width && pos.y < self.height {
            Some(&mut self.cells[pos.y * self.width + pos.x])
        } else {
            None
        }
    }

    pub fn _width(&self) -> usize {
        self.width
    }

    pub fn _height(&self) -> usize {
        self.height
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    //Create readonly VecVec map

    let map = Map::<Cell>::new_from_input(input);

    let mut region_map = HashMap::new();
    let mut regions = Vec::new();
    // scan for regions.

    for (pos, cell) in map.iter() {
        if region_map.contains_key(&pos) {
            continue;
        } else {
            let mut queue = vec![pos];
            let plot_type = cell.plant_type;
            let region = Rc::new(RefCell::new(Region {
                plot_type,
                plots: HashMap::new(),
            }));

            regions.push(Rc::clone(&region));

            while let Some(pos) = queue.pop() {
                let cell = map.get(&pos).unwrap();
                if region_map.contains_key(&pos) {
                    continue;
                }
                region_map.insert(pos, Rc::clone(&region));
                region.borrow_mut().plots.insert(pos, cell);
                for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    if let Some(new_pos) = pos.mv(&dir) {
                        if let Some(cell) = map.get(&new_pos) {
                            if cell.plant_type == plot_type {
                                queue.push(new_pos);
                            }
                        }
                    }
                }
            }
        }
    }

    let mut result = 0;
    for (i, r) in regions.iter().enumerate() {
        let region = r.borrow();
        println!("Region {} : {}", i, region.plot_type);
        let mut perimeter = region.plots.len() * 4;
        for c in &region.plots {
            println!(" - {}", c.0);
            for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                if let Some(new_pos) = c.0.mv(&dir) {
                    if let Some(cell) = map.get(&new_pos) {
                        if cell.plant_type == c.1.plant_type {
                            perimeter -= 1;
                        }
                    }
                }
            }
        }
        result += region.plots.len() * perimeter;
        println!("Area: {}, perimeter: {}", region.plots.len(), perimeter);
    }

    Some(result as u32)
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum SideType {
    Top,
    Right,
    Bottom,
    Left,
}

impl SideType {
    pub fn dir(&self) -> (i32, i32) {
        match self {
            SideType::Top => (0, -1),
            SideType::Right => (1, 0),
            SideType::Bottom => (0, 1),
            SideType::Left => (-1, 0),
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    //Create readonly VecVec map

    let map = Map::<Cell>::new_from_input(input);

    let mut region_map = HashMap::new();
    let mut regions = Vec::new();
    // scan for regions.

    for (pos, cell) in map.iter() {
        if region_map.contains_key(&pos) {
            continue;
        } else {
            let mut queue = vec![pos];
            let plot_type = cell.plant_type;
            let region = Rc::new(RefCell::new(Region {
                plot_type,
                plots: HashMap::new(),
            }));

            regions.push(Rc::clone(&region));

            while let Some(pos) = queue.pop() {
                let cell = map.get(&pos).unwrap();
                if region_map.contains_key(&pos) {
                    continue;
                }
                region_map.insert(pos, Rc::clone(&region));
                region.borrow_mut().plots.insert(pos, cell);
                for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    if let Some(new_pos) = pos.mv(&dir) {
                        if let Some(cell) = map.get(&new_pos) {
                            if cell.plant_type == plot_type {
                                queue.push(new_pos);
                            }
                        }
                    }
                }
            }
        }
    }

    let mut result = 0;
    for (i, r) in regions.iter().enumerate() {
        let region = r.borrow();
        println!("Region {} : {}", i, region.plot_type);

        let mut cell_sides_set = HashSet::new();
        for (pos, cell) in &region.plots {
            println!(" - {}", pos);
            for side in [
                SideType::Top,
                SideType::Right,
                SideType::Bottom,
                SideType::Left,
            ] {
                if let Some(new_pos) = pos.mv(&side.dir()) {
                    if let Some(side_cell) = map.get(&new_pos) {
                        if side_cell.plant_type == cell.plant_type {
                            continue;
                        }
                    }
                }
                cell_sides_set.insert((*pos, side));
            }
        }
        let mut sides = 0;
        while let Some(item) = cell_sides_set.iter().next() {
            sides += 1;
            let (pos, side_type) = *item;
            cell_sides_set.remove(&(pos, side_type));

            let dirs = match side_type {
                SideType::Top => ((-1, 0), (1, 0)),
                SideType::Right => ((0, -1), (0, 1)),
                SideType::Bottom => ((-1, 0), (1, 0)),
                SideType::Left => ((0, -1), (0, 1)),
            };

            let mut queue = vec![(pos, dirs.0), (pos, dirs.1)];

            while let Some(item) = queue.pop() {
                if let Some(neighbour) = item.0.mv(&item.1) {
                    if cell_sides_set.contains(&(neighbour, side_type)) {
                        cell_sides_set.remove(&(neighbour, side_type));
                        queue.push((neighbour, item.1));
                    }
                }
            }
        }
        result += region.plots.len() * sides;
        println!("Area: {}, sides: {}", region.plots.len(), sides);
    }

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(80));

        // let result = part_two(&advent_of_code::template::read_file_part(
        //     "examples", DAY, 1,
        // ));
        // assert_eq!(result, Some(368));
    }
}
