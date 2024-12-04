advent_of_code::solution!(2);
use advent_of_code::*;
use std::cmp::Ordering;
use std::iter::once;

enum Diff {
    Safe(Trend),
    Unsafe,
}

#[derive(PartialEq, Eq)]
enum Trend {
    Upward,
    Downward,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_count = 0;
    for line in input.lines() {
        let mut safe = 1;
        let mut prev_part: Option<u32> = None;
        let mut trend: Option<Trend> = None;
        'parts: for part in line_into_u32_iter(line) {
            if let Some(prev_part) = prev_part {
                match (diff(part, prev_part), &trend) {
                    (Diff::Safe(diff_trend), Some(curr_trend)) if diff_trend == *curr_trend => (),
                    (Diff::Safe(diff_trend), None) => trend = Some(diff_trend),
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

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe_count = 0;
    for line in input.lines() {
        let parts = line_into_u32_iter(line).collect::<Vec<u32>>();
        if is_safe_with_dampener(parts) {
            safe_count += 1;
        }
    }
    Some(safe_count)
}

// Brute-force baby! 🤠💨🔫
fn is_safe_with_dampener(parts: Vec<u32>) -> bool {
    for sans_index in once(None).chain((0..parts.len()).map(Some)) {
        if is_safe(parts.clone(), sans_index) {
            return true;
        }
    }
    false
}

fn is_safe(mut parts: Vec<u32>, remove_index: Option<usize>) -> bool {
    if let Some(i) = remove_index {
        parts.remove(i);
    };

    let mut prev_part: Option<u32> = None;
    let mut trend = None;
    for i in 0..parts.len() {
        let current_part_u32 = parts[i];
        if let Some(prev_part_u32) = prev_part {
            match (diff(current_part_u32, prev_part_u32), &trend) {
                (Diff::Safe(diff_trend), Some(curr_trend)) if diff_trend == *curr_trend => (),
                (Diff::Safe(diff_trend), None) => trend = Some(diff_trend),
                (_, _) => return false,
            }
        }
        prev_part = Some(current_part_u32);
    }
    true
}

fn diff(part: u32, prev_part: u32) -> Diff {
    match (part.cmp(&prev_part), part.abs_diff(prev_part)) {
        (Ordering::Greater, 1..=3) => Diff::Safe(Trend::Upward),
        (Ordering::Less, 1..=3) => Diff::Safe(Trend::Downward),
        _ => Diff::Unsafe,
    }
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
