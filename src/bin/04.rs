advent_of_code::solution!(4);

type Grid = Vec<Vec<char>>;

#[derive(Debug, Clone)]
enum Direction {
    Right,
    DownRight,
    Down,
    DownLeft,
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();
    let num_rows = grid.len();
    let num_cols = grid.get(0).unwrap().len();
    let is_xmas = build_is_xmas_tester(grid.clone());
    let directions = build_directions_lister(num_rows, num_cols);
    let mut matches: u32 = 0;
    for row_idx in 0..num_rows {
        let row = grid.get(row_idx).unwrap();
        for col_idx in 0..num_cols {
            if ['X', 'S'].contains(&row[col_idx]) {
                for dir in directions(row_idx, col_idx) {
                    matches += is_xmas(dir, row_idx, col_idx) as u32;
                }
            }
        }
    }
    Some(matches)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn build_is_xmas_tester(grid: Grid) -> impl Fn(Direction, usize, usize) -> bool {
    move |dir, row_idx, col_idx| is_xmas_test(&grid, dir, row_idx, col_idx)
}

fn is_xmas_test(grid: &Grid, dir: Direction, row_idx: usize, col_idx: usize) -> bool {
    let chars = match dir {
        Direction::Right => grid[row_idx][col_idx..=col_idx + 3].into(),
        Direction::Down => grid[row_idx..=row_idx + 3]
            .iter()
            .map(|row| row[col_idx])
            .collect::<Vec<char>>(),
        Direction::DownRight => (col_idx..=col_idx + 3)
            .zip(&grid[row_idx..=row_idx + 3])
            .map(|(col_idx, row)| row[col_idx])
            .collect::<Vec<char>>(),
        Direction::DownLeft => (col_idx - 3..=col_idx)
            .rev()
            .zip(&grid[row_idx..=row_idx + 3])
            .map(|(col_idx, row)| row[col_idx])
            .collect::<Vec<char>>(),
    };
    chars == ['X', 'M', 'A', 'S'] || chars == ['S', 'A', 'M', 'X']
}

fn build_directions_lister(
    num_rows: usize,
    num_cols: usize,
) -> impl Fn(usize, usize) -> Vec<Direction> {
    move |row_idx, col_idx| list_directions((row_idx, num_rows), (col_idx, num_cols))
}

fn list_directions(
    (row_idx, rows_len): (usize, usize),
    (col_idx, cols_len): (usize, usize),
) -> Vec<Direction> {
    let at_least_4_chars_right = col_idx <= cols_len - 4;
    let at_least_4_chars_down = row_idx <= rows_len - 4;
    let at_least_4_chars_left = col_idx >= 3;
    let mut directions: Vec<Direction> = vec![];
    if at_least_4_chars_right {
        directions.push(Direction::Right);
    }
    if at_least_4_chars_down {
        directions.push(Direction::Down);
        if at_least_4_chars_right {
            directions.push(Direction::DownRight);
        }
        if at_least_4_chars_left {
            directions.push(Direction::DownLeft);
        }
    }
    directions
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
        assert_eq!(result, None);
    }
}
