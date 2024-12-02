advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lefts = vec![];
    let mut rights = vec![];

    input.lines().for_each(|line| {
        let [left, right]: [u32; 2] = line
            .split_whitespace()
            .map(|s| s.parse::<u32>().expect("Failed to parse u32"))
            .collect::<Vec<u32>>()
            .try_into()
            .expect("Failed to split line");

        lefts.push(left);
        rights.push(right);
    });

    lefts.sort();
    rights.sort();

    let mut diff = 0;
    for (index, left) in lefts.iter().enumerate() {
        let right = rights.get(index).expect("Failed to get pair");
        diff += left.abs_diff(*right);
    }

    Some(diff)
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
