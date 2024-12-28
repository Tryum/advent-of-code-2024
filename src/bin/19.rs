use std::collections::{HashMap, HashSet};

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let towels = lines
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.trim())
        .collect::<Vec<_>>();

    lines.next();

    let mut result = 0;

    'main: for design in lines {
        let mut queue = Vec::new();
        let mut visits = HashSet::new();
        queue.push(design);
        while let Some(design) = queue.pop() {
            for t in &towels {
                if design == *t {
                    result += 1;
                    continue 'main;
                }
                if let Some(stripped) = design.strip_prefix(t) {
                    if !visits.contains(stripped) {
                        queue.push(stripped);
                        visits.insert(stripped);
                    }
                }
            }
        }
    }

    Some(result)
}

fn find_arrangements(design: &str, towels: &Vec<String>, memo: &mut HashMap<String, u64>) -> u64 {
    if memo.contains_key(design) {
        memo[design]
    } else {
        let mut result = 0;
        for t in towels {
            if let Some(stripped) = design.strip_prefix(t) {
                if stripped.is_empty() {
                    // end of string
                    result += 1;
                } else {
                    result += find_arrangements(stripped, towels, memo);
                }
            }
        }
        memo.insert(design.to_string(), result);
        result
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let towels = lines
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.trim().to_string())
        .collect::<Vec<_>>();

    lines.next();

    let mut result = 0;

    let mut memo = HashMap::<String, u64>::new();

    for design in lines {
        let arr = find_arrangements(design, &towels, &mut memo);
        println!("{} : {}", design, arr);
        result += arr;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
