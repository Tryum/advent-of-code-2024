use regex::Regex;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(
        r"Button A: X\+(\d*), Y\+(\d*)\s*Button B: X\+(\d*), Y\+(\d*)\s*Prize: X=(\d*), Y=(\d*)",
    )
    .unwrap();

    let mut result = 0;

    for [ax, ay, bx, by, px, py] in re
        .captures_iter(input)
        .map(|c| c.extract().1.map(|x| x.parse::<i32>().unwrap()))
    {
        let dt = ax * by - ay * bx;

        if (px * by - py * bx) % dt != 0 || (ax * py - ay * px) % dt != 0 {
            continue;
        }

        let x = (px * by - py * bx) / dt;
        let y = (ax * py - ay * px) / dt;

        if x < 0 || y < 0 {
            continue;
        }

        result += x * 3 + y;
    }

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = Regex::new(
        r"Button A: X\+(\d*), Y\+(\d*)\s*Button B: X\+(\d*), Y\+(\d*)\s*Prize: X=(\d*), Y=(\d*)",
    )
    .unwrap();

    let mut result = 0;

    for [ax, ay, bx, by, px, py] in re
        .captures_iter(input)
        .map(|c| c.extract().1.map(|x| x.parse::<i64>().unwrap()))
    {
        let px = 10000000000000 + px;
        let py = 10000000000000 + py;
        let dt = ax * by - ay * bx;

        if (px * by - py * bx) % dt != 0 || (ax * py - ay * px) % dt != 0 {
            continue;
        }

        let x = (px * by - py * bx) / dt;
        let y = (ax * py - ay * px) / dt;

        if x < 0 || y < 0 {
            continue;
        }

        result += x * 3 + y;
    }

    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
