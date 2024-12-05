use std::collections::HashSet;

advent_of_code::solution!(4);

fn find_word(
    word: &str,
    pos: (usize, usize),
    dir: (i32, i32),
    size: (usize, usize),
    map: &Vec<&str>,
) -> bool {
    if map[pos.1].chars().nth(pos.0) == word.chars().nth(0) {
        let next_slice = &word[1..];
        if next_slice.len() > 0 {
            let x = pos.0 as i32 + dir.0 as i32;
            let y = pos.1 as i32 + dir.1 as i32;
            if x >= 0 && x < size.0 as i32 && y >= 0 && y < size.1 as i32 {
                find_word(next_slice, (x as usize, y as usize), dir, size, map)
            } else {
                false
            }
        } else {
            true
        }
    } else {
        false
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let width = lines[0].len();
    let height = lines.len();

    let mut total = 0;

    for y in 0..height {
        for x in 0..width {
            for dy in [-1, 0, 1] {
                for dx in [-1, 0, 1] {
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    if find_word("XMAS", (x, y), (dx, dy), (width, height), &lines) {
                        total += 1;
                    }
                }
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let width = lines[0].len();
    let height = lines.len();

    let mut total = 0;

    let mut mas_set = HashSet::new();

    for y in 0..height {
        for x in 0..width {
            for dy in [-1, 1] {
                for dx in [-1, 1] {
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    if find_word("MAS", (x, y), (dx, dy), (width, height), &lines) {
                        let a_pos = (x as i32 + dx, y as i32 + dy);

                        if mas_set.contains(&a_pos) {
                            total += 1;
                        } else {
                            mas_set.insert(a_pos);
                        }
                    }
                }
            }
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
