advent_of_code::solution!(4);
use itertools::Itertools;
use std::collections::HashMap;
use std::ops::Add;
use Direction::*;

type Index = (usize, usize);

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    DownRight,
    Down,
    DownLeft,
}

struct Puzzle {
    cells: Vec<Vec<char>>,
    rows_len: usize,
    cols_len: usize,
    word: String,
    word_len: usize,
    affixes: [char; 2],
    directions: Vec<Direction>,
}

impl Puzzle {
    fn new(input: &str, word: &str, directions: Vec<Direction>) -> Self {
        let cells: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let rows_len = cells.len();
        let cols_len = cells.get(0).unwrap().len();
        let affixes = [word.chars().next().unwrap(), word.chars().last().unwrap()];

        Self {
            cells,
            rows_len,
            cols_len,
            word: word.to_string(),
            word_len: word.len(),
            affixes,
            directions,
        }
    }

    fn is_affix(&self, char: &char) -> bool {
        self.affixes.contains(char)
    }

    fn is_match(&self, other_word: String) -> bool {
        self.word == other_word || self.word == other_word.chars().rev().join("")
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let puzzle = Puzzle::new(&input, "XMAS", vec![Right, DownRight, Down, DownLeft]);
    let mut matches: u32 = 0;
    for row_idx in 0..puzzle.rows_len {
        let row = puzzle.cells.get(row_idx).unwrap();
        for col_idx in 0..puzzle.cols_len {
            if puzzle.is_affix(&row[col_idx]) {
                for direction in directions(&puzzle, (row_idx, col_idx)) {
                    matches += is_match(&puzzle, &direction, (row_idx, col_idx)) as u32;
                }
            }
        }
    }
    Some(matches)
}

pub fn part_two(input: &str) -> Option<u32> {
    let puzzle = Puzzle::new(&input, "SAM", vec![DownLeft, DownRight]);
    // 1st pass: Build hash with (row, dir) keys holding cols with matches for that row/direction
    let mut sams: HashMap<(usize, Direction), Vec<usize>> = HashMap::new();
    for row_idx in 0..puzzle.rows_len {
        for col_idx in 0..puzzle.cols_len {
            if puzzle.is_affix(&puzzle.cells[row_idx][col_idx]) {
                for direction in directions(&puzzle, (row_idx, col_idx)) {
                    if is_match(&puzzle, &direction, (row_idx, col_idx)) {
                        sams.entry((row_idx, direction)).or_default().push(col_idx);
                    }
                }
            }
        }
    }
    // 2nd pass: Iterate DownRight matches and count instances of DownLeft two columns over
    let mut sam_xs = 0;
    for ((row_idx, _direction), cols) in sams
        .iter()
        .filter(|((_row_idx, direction), _cols)| *direction == DownRight)
        .sorted_by(|(a, _), (b, _)| Ord::cmp(&a.0, &b.0))
    {
        for col_idx in cols {
            if let Some(cols) = sams.get(&(*row_idx, DownLeft)) {
                if cols.contains(&col_idx.add(2)) {
                    sam_xs += 1;
                }
            }
        }
    }
    Some(sam_xs)
}

fn directions(puzzle: &Puzzle, (row_idx, col_idx): Index) -> Vec<Direction> {
    let at_least_4_chars_right = col_idx <= puzzle.cols_len - puzzle.word_len;
    let at_least_4_chars_down = row_idx <= puzzle.rows_len - puzzle.word_len;
    let at_least_4_chars_left = col_idx >= puzzle.word_len - 1;
    let mut directions: Vec<Direction> = vec![];
    if at_least_4_chars_right {
        directions.push(Right);
    }
    if at_least_4_chars_down {
        directions.push(Down);
        if at_least_4_chars_right {
            directions.push(DownRight);
        }
        if at_least_4_chars_left {
            directions.push(DownLeft);
        }
    }
    directions.retain(|direction| puzzle.directions.contains(direction));
    directions
}

fn is_match(puzzle: &Puzzle, direction: &Direction, (row_idx, col_idx): Index) -> bool {
    let cells = puzzle.cells.clone();
    let offset = puzzle.word_len - 1;
    let chars = match direction {
        Right => cells[row_idx][col_idx..=col_idx + offset].into(),
        Down => cells[row_idx..=row_idx + offset]
            .iter()
            .map(|row| row[col_idx])
            .collect::<Vec<char>>(),
        DownRight => (col_idx..=col_idx + offset)
            .zip(&cells[row_idx..=row_idx + offset])
            .map(|(col_idx, row)| row[col_idx])
            .collect::<Vec<char>>(),
        DownLeft => (col_idx - offset..=col_idx)
            .rev()
            .zip(&cells[row_idx..=row_idx + offset])
            .map(|(col_idx, row)| row[col_idx])
            .collect::<Vec<char>>(),
    };
    let word: String = chars.into_iter().collect();
    puzzle.is_match(word)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
