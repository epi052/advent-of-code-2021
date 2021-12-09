use advent_of_code_2021::{get_input, parse_args};
use std::collections::HashSet;

fn get_heightmap(input: &str) -> Vec<Vec<i32>> {
    let mut heightmap = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        heightmap.push(Vec::new());
        for ch in line.chars() {
            heightmap[idx].push(ch.to_digit(10).unwrap() as i32)
        }
    }

    heightmap
}

fn is_lowpoint(row: usize, col: usize, map: &[Vec<i32>]) -> bool {
    // we'll assume access to row/col is valid
    let current = map[row][col];

    // access to adjacent points can fail, but we can reuse row and col; even so we can underflow
    // the usize for row/col, so that needs to be handled as well as the actual map access
    if let Some(right) = map[row].get(col + 1) {
        if *right < current {
            return false;
        }
    }

    if let Some(checked) = col.checked_sub(1) {
        if let Some(left) = map[row].get(checked) {
            if *left < current {
                return false;
            }
        }
    }

    if let Some(checked) = row.checked_sub(1) {
        if let Some(up) = map.get(checked) {
            if up[col] < current {
                return false;
            }
        }
    }

    if let Some(down) = map.get(row + 1) {
        if down[col] < current {
            return false;
        }
    }

    true
}

fn get_basin(
    map: &[Vec<i32>],
    visited: &mut HashSet<(usize, usize)>,
    to_visit: &mut Vec<(usize, usize)>,
) {
    // similar to lowpoint, need to handle OOB array access as well as underflows
    while !to_visit.is_empty() {
        // as long as we have places to go, keep looping

        // grab the next place to check, and mark it as visited
        let (row, col) = to_visit.pop().unwrap();
        visited.insert((row, col));

        if let Some(checked_row) = row.checked_sub(1) {
            // row isn't 0
            if let Some(up) = map.get(checked_row) {
                // can go up a row
                if up[col] != 9 && !visited.contains(&(checked_row, col)) {
                    // value above isn't 9, add it to to_visit list
                    to_visit.push((checked_row, col));
                }
            }
        }

        if let Some(down) = map.get(row + 1) {
            // can go down a row
            if down[col] != 9 && !visited.contains(&(row + 1, col)) {
                to_visit.push((row + 1, col));
            }
        }

        if let Some(checked_col) = col.checked_sub(1) {
            // col isn't zero
            if let Some(left) = map[row].get(checked_col) {
                // can go left a column
                if *left != 9 && !visited.contains(&(row, checked_col)) {
                    to_visit.push((row, checked_col));
                }
            }
        }

        if let Some(right) = map[row].get(col + 1) {
            // can go right a column
            if *right != 9 && !visited.contains(&(row, col + 1)) {
                to_visit.push((row, col + 1));
            }
        }
    }
}

fn part_one(input: &str) -> i32 {
    let heightmap = get_heightmap(input);
    let mut risk_levels = Vec::new();

    for i in 0..heightmap.len() {
        for j in 0..heightmap[0].len() {
            if is_lowpoint(i, j, &heightmap) {
                risk_levels.push(heightmap[i][j] + 1); // store height + 1 for risk level
            }
        }
    }

    risk_levels.iter().sum::<i32>()
}

fn part_two(input: &str) -> i32 {
    let heightmap = get_heightmap(input);
    let mut to_visit = Vec::new();
    let mut visited = HashSet::new();
    let mut basins = Vec::new();

    for i in 0..heightmap.len() {
        for j in 0..heightmap[0].len() {
            if is_lowpoint(i, j, &heightmap) {
                to_visit.push((i, j)); // add lowpoint as a place of interest

                // get_basin only operates on a single lowpoint at a time
                get_basin(&heightmap, &mut visited, &mut to_visit);

                basins.push(visited.len());

                // to_visit is cleared when get_basin finishes, no need to clean it up as well
                visited.clear();
            }
        }
    }

    basins.sort_unstable();
    basins.reverse(); // big ones up front

    basins[..3].iter().fold(1, |mut acc, basin| {
        acc *= *basin;
        acc
    }) as i32
}

fn main() {
    let args = parse_args();
    let input = get_input(file!());

    match args.part {
        1 => println!("{}", part_one(&input)),
        2 => println!("{}", part_two(&input)),
        _ => println!(
            "got unexpected value for --part: {} (try 1 or 2)",
            args.part
        ),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_one() {
        assert_eq!(15, part_one(INPUT));
    }

    #[test]
    fn test_two() {
        assert_eq!(1134, part_two(INPUT));
    }
}
