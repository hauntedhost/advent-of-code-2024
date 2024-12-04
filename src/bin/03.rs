advent_of_code::solution!(3);
use advent_of_code::parse_u32;
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let sum = sum_mul_instructions(&strip_newlines(input));
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let filtered_input = remove_dont_blocks(&strip_newlines(input));
    let sum = sum_mul_instructions(&filtered_input);
    Some(sum)
}

fn sum_mul_instructions(input: &str) -> u32 {
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let sum: u32 = mul_re
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [x, y])| parse_u32(x) * parse_u32(y))
        .sum();
    sum
}

fn remove_dont_blocks(input: &str) -> String {
    let dont_re = Regex::new(r"(?x)(don't\(\).*?)(do\(\)|$)").unwrap();
    let filtered_input = dont_re
        .replace_all(&strip_newlines(input), |caps: &regex::Captures| {
            caps[0].replace(&caps[1], "")
        })
        .to_string();
    filtered_input
}

fn strip_newlines(input: &str) -> String {
    input.lines().collect::<String>()
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
