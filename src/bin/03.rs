use regex::Regex;

advent_of_code::solution!(3);

fn parse_input(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total = 0;
    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        total += a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap();
    }
    total
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse_input(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    let split: Vec<&str> = input.split("don't()").collect();

    let mut total = 0;

    total += parse_input(split[0]);

    for s in &split[1..] {
        let split = s.split_once("do()");
        if let Some((_, p)) = split {
            total += parse_input(p);
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
