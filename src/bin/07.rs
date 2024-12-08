use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Mul,
    Concat,
}

impl Operation {
    fn op(self, lhs: &u64, rhs: &u64) -> u64 {
        match self {
            Operation::Add => lhs + rhs,
            Operation::Mul => lhs * rhs,
            Operation::Concat => lhs * 10u64.pow(rhs.checked_ilog10().unwrap_or(0) + 1) + rhs,
        }
    }
}

fn solve(input: &str, ops: &[Operation]) -> Option<u64> {
    let mut total = 0;
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let (op_result, operands) = line.split_once(":").unwrap();
        let op_result = op_result.parse::<u64>().unwrap();
        let operands: Vec<u64> = operands
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let operations = ops;
        let operation_count = operands.len() - 1;

        let combinations = (0..operation_count)
            .map(|_| operations)
            .multi_cartesian_product();

        for ops in combinations {
            let mut it = operands.iter();
            let mut result = *it.next().unwrap();
            for op in ops {
                result = op.op(&result, it.next().unwrap());
            }
            if result == op_result {
                total += result;
                break;
            }
        }
    }
    Some(total)
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, &[Operation::Add, Operation::Mul])
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, &[Operation::Add, Operation::Mul, Operation::Concat])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
