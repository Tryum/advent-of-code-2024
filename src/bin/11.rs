use std::collections::HashMap;

advent_of_code::solution!(11);

fn blink_stone(s: u64, blink_count: u32, memo: &mut HashMap<(u64, u32), u64>) -> u64 {
    let result;
    if memo.contains_key(&(s, blink_count)) {
        return memo[&(s, blink_count)];
    } else if blink_count == 1 {
        let stone_as_string = s.to_string();
        if stone_as_string.len() % 2 == 0 {
            result = 2;
        } else {
            result = 1;
        }
    } else if s == 0 {
        result = blink_stone(1, blink_count - 1, memo);
    } else {
        let stone_as_string = s.to_string();
        if stone_as_string.len() % 2 == 0 {
            let (lhs, rhs) = stone_as_string.split_at(stone_as_string.len() / 2);
            result = blink_stone(lhs.parse().unwrap(), blink_count - 1, memo)
                + blink_stone(rhs.parse().unwrap(), blink_count - 1, memo);
        } else {
            result = blink_stone(s * 2024, blink_count - 1, memo);
        }
    }
    memo.insert((s, blink_count), result);
    result
}

fn solve_day11(input: &str, blink_count: u32) -> Option<u64> {
    let stones: Vec<u64> = input
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut memo = HashMap::new();

    let mut result = 0;

    for s in &stones {
        result += blink_stone(*s, blink_count, &mut memo);
    }

    Some(result)
}

pub fn part_one(input: &str) -> Option<u64> {
    solve_day11(input, 25)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve_day11(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blink() {
        let initial_arrangement = "125 17";

        assert_eq!(solve_day11(initial_arrangement, 1), Some(3));
        assert_eq!(solve_day11(initial_arrangement, 2), Some(4));
        assert_eq!(solve_day11(initial_arrangement, 3), Some(5));
        assert_eq!(solve_day11(initial_arrangement, 4), Some(9));
        assert_eq!(solve_day11(initial_arrangement, 5), Some(13));
        assert_eq!(solve_day11(initial_arrangement, 6), Some(22));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
