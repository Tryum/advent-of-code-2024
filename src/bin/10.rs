use std::collections::HashMap;
use std::fmt;

advent_of_code::solution!(10);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Position {
    fn mv(self, dir: (i32, i32)) -> Option<Position> {
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

fn _print_map(pos: &Position, map: &[Vec<u32>]) {
    println!("\n");
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if pos.x == x && pos.y == y {
                print!("X");
            } else {
                print!("{}", cell);
            }
        }
        println!();
    }
    println!("\n");
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Vec::new();

    let mut start_points = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let cell = c.to_digit(10).unwrap();
            if cell == 0 {
                let start_pos = Position { x, y };
                start_points.push(start_pos);
            }
            row.push(cell);
        }
        map.push(row);
    }

    let height = map.len();
    let width = map[0].len();

    let mut result = 0;

    for starting_point in start_points {
        let mut visited_cells = HashMap::new();
        let mut queue = Vec::new();
        visited_cells.insert(starting_point, 0);
        queue.push(starting_point);
        let mut sub_result = 0;
        while let Some(pos) = queue.pop() {
            //print_map(&pos, &map);
            let cell_value = map[pos.y][pos.x];
            //println!("{} : {}", pos, cell_value);

            for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                if let Some(new_pos) = pos.mv(dir) {
                    if new_pos.x >= width || new_pos.y >= height {
                        continue;
                    }
                    let new_cell_value = map[new_pos.y][new_pos.x];
                    let trail = visited_cells[&pos];
                    if new_cell_value > cell_value
                        && new_cell_value.abs_diff(cell_value) == 1
                        && trail + 1 < *visited_cells.entry(new_pos).or_insert(u32::MAX)
                    {
                        visited_cells.insert(new_pos, trail + 1);
                        if new_cell_value == 9 {
                            sub_result += 1;
                        } else {
                            //println!("Adding {} : {}", new_pos, new_cell_value);
                            queue.push(new_pos);
                        }
                    } else {
                        //println!("Discarding {} : {}", new_pos, new_cell_value);
                    }
                }
            }
        }
        result += sub_result;

        //println!(" Starting point {}, score :{}", starting_point, sub_result);
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = Vec::new();

    let mut start_points = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let cell = c.to_digit(10).unwrap();
            if cell == 0 {
                let start_pos = Position { x, y };
                start_points.push(start_pos);
            }
            row.push(cell);
        }
        map.push(row);
    }

    let height = map.len();
    let width = map[0].len();

    let mut result = 0;

    for starting_point in start_points {
        let mut queue = Vec::new();
        queue.push((starting_point, vec![starting_point]));
        let mut sub_result = 0;
        while let Some((pos, path)) = queue.pop() {
            //print_map(&pos, &map);
            let cell_value = map[pos.y][pos.x];
            //println!("{} : {}", pos, cell_value);

            for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                if let Some(new_pos) = pos.mv(dir) {
                    if new_pos.x >= width || new_pos.y >= height {
                        continue;
                    }
                    let new_cell_value = map[new_pos.y][new_pos.x];
                    if new_cell_value > cell_value
                        && new_cell_value.abs_diff(cell_value) == 1
                        && !path.contains(&new_pos)
                    {
                        if new_cell_value == 9 {
                            sub_result += 1;
                        } else {
                            let mut path = path.clone();
                            path.push(new_pos);
                            //println!("Adding {} : {}", new_pos, new_cell_value);
                            queue.push((new_pos, path));
                        }
                    } else {
                        //println!("Discarding {} : {}", new_pos, new_cell_value);
                    }
                }
            }
        }
        result += sub_result;

        //println!(" Starting point {}, score :{}", starting_point, sub_result);
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
