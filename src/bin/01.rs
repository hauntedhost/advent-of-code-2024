advent_of_code::solution!(1);
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let (mut lefts, mut rights): (Vec<u32>, Vec<u32>) = input.lines().map(parse_u32_pair).unzip();

    lefts.sort();
    rights.sort();

    let mut diff = 0;
    for (index, left) in lefts.iter().enumerate() {
        let right = rights.get(index).expect("Failed to get pair");
        diff += left.abs_diff(*right);
    }

    Some(diff)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lefts = vec![];
    let mut counts: HashMap<u32, u32> = HashMap::new();

    input.lines().for_each(|line| {
        let (left, right) = parse_u32_pair(line);
        lefts.push(left);
        *counts.entry(right).or_insert(0) += 1;
    });

    let mut score: u32 = 0;
    for left in lefts {
        let count = *counts.get(&left).unwrap_or(&0);
        score += left * count;
    }

    Some(score)
}

fn parse_u32_pair(line: &str) -> (u32, u32) {
    let mut parts = line
        .split_whitespace()
        .map(|s| s.parse::<u32>().expect("Failed to parse u32"));

    (
        parts.next().expect("Missing left value"),
        parts.next().expect("Missing right value"),
    )
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
