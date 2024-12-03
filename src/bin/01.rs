use itertools::Itertools;
advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut l_list = Vec::new();
    let mut r_list = Vec::new();
    for list in input.lines() {
        let (l, r) = list
            .split_ascii_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .next_tuple()
            .unwrap();

        l_list.push(l);
        r_list.push(r);
    }
    (l_list, r_list)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut l_list, mut r_list) = parse_input(input);

    l_list.sort();
    r_list.sort();

    let mut total = 0;
    for (l, r) in l_list.iter().zip(r_list.iter()) {
        total += l.abs_diff(*r);
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (l_list, r_list) = parse_input(input);

    let mut total = 0;

    for i in &l_list {
        total += i * r_list.iter().filter(|&x| x == i).count() as u32;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(31));
    }
}
