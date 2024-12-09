use std::{cmp, collections::HashMap};

use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result: u64 = 0;

    let mut blocks = Vec::new();

    for c in input.as_bytes() {
        if !c.is_ascii_digit() {
            continue;
        }
        blocks.push((c - b'0') as u64);
    }

    let mut index = 0;
    let mut r_index = blocks.len() - 1;
    let mut cursor = 0;
    while index <= r_index {
        if index % 2 == 0 {
            let size = blocks[index];
            for i in cursor..cursor + size {
                result += i * (index / 2) as u64;
            }
            cursor += size;
            index += 1;
        } else {
            while blocks[index] > 0 {
                let size = cmp::min(blocks[index], blocks[r_index]);
                for i in cursor..cursor + size {
                    result += i * (r_index / 2) as u64;
                }
                cursor += size;
                blocks[index] -= size;
                blocks[r_index] -= size;
                if blocks[r_index] == 0 {
                    r_index -= 2;
                }
            }
            index += 1;
        }
    }

    println!(
        "index : {}[{}], r_index : {}[{}]",
        index, blocks[index], r_index, blocks[r_index]
    );

    Some(result)
}

#[derive(Debug)]
struct BlockDescriptor {
    address: u32,
    size: u8,
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result: u64 = 0;

    let mut cursor = 0;

    let mut file_map = HashMap::new();
    let mut free_space = Vec::new();
    for (i, c) in input.as_bytes().iter().enumerate() {
        if !c.is_ascii_digit() {
            continue;
        }
        let size = c - b'0';
        let block = BlockDescriptor {
            address: cursor,
            size,
        };
        cursor += size as u32;
        if i % 2 == 0 {
            file_map.insert(i / 2, block);
        } else {
            free_space.push(block);
        }
    }

    let keys = file_map.keys().cloned().sorted().rev();

    for k in keys {
        let file = file_map.get_mut(&k).unwrap();
        for b in &mut free_space {
            if b.size >= file.size && b.address < file.address {
                //println!("move File {} to {}", k, b.address);
                file.address = b.address;
                b.size -= file.size;
                b.address += file.size as u32;
                break;
            }
        }
    }

    let files = file_map
        .iter()
        .sorted_by(|a, b| Ord::cmp(&a.1.address, &b.1.address));

    //dbg!(&files);

    for (index, b) in files {
        for i in b.address..b.address + b.size as u32 {
            result += (i as u64) * (*index as u64);
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
