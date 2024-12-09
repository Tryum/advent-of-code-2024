use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

fn get_antinodes(a1: &(usize, usize), a2: &(usize, usize)) -> ((usize, usize), (usize, usize)) {
    let dx = a2.0 as isize - a1.0 as isize;
    let dy = a2.1 as isize - a1.1 as isize;

    let antinode1 = (a1.0.wrapping_add_signed(-dx), a1.1.wrapping_add_signed(-dy));

    let antinode2 = (a2.0.wrapping_add_signed(dx), a2.1.wrapping_add_signed(dy));

    (antinode1, antinode2)
}

fn get_antinodes2(
    a1: &(usize, usize),
    a2: &(usize, usize),
    width: usize,
    height: usize,
) -> Vec<(usize, usize)> {
    let dx = a2.0 as isize - a1.0 as isize;
    let dy = a2.1 as isize - a1.1 as isize;

    let mut result = Vec::new();

    let mut factor = 1;
    loop {
        let antinode = (
            a1.0.wrapping_add_signed(-factor * dx),
            a1.1.wrapping_add_signed(-factor * dy),
        );
        if antinode.0 < width && antinode.1 < height {
            result.push(antinode);
        } else {
            break;
        }
        factor += 1;
    }

    let mut factor = 1;
    loop {
        let antinode = (
            a1.0.wrapping_add_signed(factor * dx),
            a1.1.wrapping_add_signed(factor * dy),
        );
        if antinode.0 < width && antinode.1 < height {
            result.push(antinode);
        } else {
            break;
        }
        factor += 1;
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();

    let height = lines.len();
    let width = lines[0].len();

    let mut antennas = HashMap::<char, HashSet<(usize, usize)>>::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                antennas.entry(c).or_default().insert((x, y));
            }
        }
    }

    let mut antinodes = HashSet::<(usize, usize)>::new();

    for (_, set) in antennas {
        for c in set.iter().combinations(2) {
            let (a1, a2) = c.iter().next_tuple().unwrap();
            let (antinode1, antinode2) = get_antinodes(a1, a2);

            if antinode1.0 < width && antinode1.1 < height {
                antinodes.insert(antinode1);
            }
            if antinode2.0 < width && antinode2.1 < height {
                antinodes.insert(antinode2);
            }
        }
    }

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();

    let height = lines.len();
    let width = lines[0].len();

    let mut antennas = HashMap::<char, HashSet<(usize, usize)>>::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                antennas.entry(c).or_default().insert((x, y));
            }
        }
    }

    let mut antinodes = HashSet::<(usize, usize)>::new();

    for (_, set) in antennas {
        if set.len() > 1 {
            antinodes.extend(set.clone());
        }

        for c in set.iter().combinations(2) {
            let (a1, a2) = c.iter().next_tuple().unwrap();
            let new_antinodes = get_antinodes2(a1, a2, width, height);

            antinodes.extend(new_antinodes);
        }
    }

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_axial_symetry() {
        // 4,3 / 5,5
        let a1 = (4, 3);
        let a2 = (5, 5);
        let antinodes = get_antinodes(&a1, &a2);
        let antinodes = [antinodes.0, antinodes.1];
        assert!(antinodes.contains(&(3, 1)));
        assert!(antinodes.contains(&(6, 7)));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }

    #[test]
    fn test_part_two_lite() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(9));
    }
}
