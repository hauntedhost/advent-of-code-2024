advent_of_code::solution!(2);
use advent_of_code::*;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
enum Trend {
    Unknown,
    Increasing,
    Decreasing,
}

enum Diff {
    Unsafe,
    SafeIncrease,
    SafeDecrease,
}

fn diff(part: u32, prev_part: u32) -> Diff {
    match (part.cmp(&prev_part), part.abs_diff(prev_part)) {
        (Ordering::Greater, diff) if diff <= 3 => Diff::SafeIncrease,
        (Ordering::Less, diff) if diff <= 3 => Diff::SafeDecrease,
        (_, _) => Diff::Unsafe,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_count = 0;
    for line in input.lines() {
        let mut safe = 1;
        let mut prev_part: Option<u32> = None;
        let mut prev_trend = Trend::Unknown;
        'parts: for part in parse_u32_parts(line) {
            if let Some(prev_part) = prev_part {
                match (diff(part, prev_part), prev_trend) {
                    (Diff::SafeIncrease, Trend::Increasing | Trend::Unknown) => {
                        prev_trend = Trend::Increasing;
                    }
                    (Diff::SafeDecrease, Trend::Decreasing | Trend::Unknown) => {
                        prev_trend = Trend::Decreasing;
                    }
                    (_, _) => {
                        safe = 0;
                        break 'parts;
                    }
                };
            }
            prev_part = Some(part);
        }
        safe_count += safe;
    }
    Some(safe_count)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
