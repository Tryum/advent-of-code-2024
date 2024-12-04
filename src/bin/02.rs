advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

#[derive(PartialEq)]
enum ReportType {
    Increasing,
    Decreasing,
}

fn is_report_valid(report: &[u32]) -> bool {
    let mut report_type = None;
    let mut report_is_valid = true;
    for i in 0..report.len() - 1 {
        let level_type = if report[i] > report[i + 1] {
            &ReportType::Decreasing
        } else {
            &ReportType::Increasing
        };
        let diff = report[i].abs_diff(report[i + 1]);
        if report_type.is_none() {
            report_type = Some(level_type);
        }

        if report_type != Some(level_type) || !(1..=3).contains(&diff) {
            report_is_valid = false;
            break;
        }
    }
    report_is_valid
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    let mut total_valid_report = 0;
    for r in reports {
        if is_report_valid(&r) {
            total_valid_report += 1;
        }
    }
    Some(total_valid_report)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    let mut total_valid_report = 0;
    for r in reports {
        let mut variations = Vec::new();
        for i in 0..r.len() {
            let mut variation = r.clone();
            variation.remove(i);
            variations.push(variation);
        }

        for v in variations {
            if is_report_valid(&v) {
                total_valid_report += 1;
                break;
            }
        }
    }
    Some(total_valid_report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
