use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(5);

fn is_update_valid(update: &[u32], rules: &HashMap<u32, Vec<u32>>) -> bool {
    let mut valid_update = true;
    'outer: for i in 0..update.len() {
        let page = update[i];
        if rules.contains_key(&page) {
            let print_before = rules.get(&page).unwrap();
            for page in &update[0..i] {
                if print_before.contains(page) {
                    // println!("update : {:?}", update);
                    // println!("{}|{:?} violated", &page, &print_before);
                    valid_update = false;
                    break 'outer;
                }
            }
        }
    }
    valid_update
}

struct PrintQueue {
    pub good: Vec<Vec<u32>>,
    pub bad: Vec<Vec<u32>>,
    pub rules: HashMap<u32, Vec<u32>>,
}

fn filter_good_bad_updates(input: &str) -> PrintQueue {
    let mut parse_rules = true;
    let mut rules = HashMap::<u32, Vec<u32>>::new();

    let mut good = Vec::new();
    let mut bad = Vec::new();
    for l in input.lines() {
        if l.is_empty() {
            parse_rules = false;
            continue;
        }
        if parse_rules {
            let (l, r) = l
                .split('|')
                .map(|x| x.parse::<u32>().unwrap())
                .next_tuple()
                .unwrap();
            if let Some(v) = rules.get_mut(&l) {
                v.push(r);
            } else {
                rules.insert(l, vec![r]);
            }
        } else {
            let update: Vec<u32> = l.split(',').map(|x| x.parse::<u32>().unwrap()).collect();

            if is_update_valid(&update, &rules) {
                good.push(update);
            } else {
                bad.push(update);
            }
        }
    }
    PrintQueue { good, bad, rules }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;

    let print_queue = filter_good_bad_updates(input);

    for u in print_queue.good {
        let middle_index = u.len() / 2;
        let middle_page = u[middle_index];
        result += middle_page;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;

    let print_queue = filter_good_bad_updates(input);

    for mut update in print_queue.bad {
        println!("update : {:?}", update);
        while !is_update_valid(&update, &print_queue.rules) {
            'outer: for i in 0..update.len() {
                let page = update[i];
                if print_queue.rules.contains_key(&page) {
                    let print_before = print_queue.rules.get(&page).unwrap();
                    for j in 0..i {
                        if print_before.contains(&update[j]) {
                            let removed_page = update.remove(j);
                            update.insert(i, removed_page);
                            break 'outer;
                        }
                    }
                }
            }
        }
        let middle_index = update.len() / 2;
        let middle_page = update[middle_index];
        result += middle_page;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
