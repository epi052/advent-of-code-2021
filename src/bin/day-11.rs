use advent_of_code_2021::{get_input, parse_args};
use std::fmt::{Display, Formatter};

#[derive(Debug, Default)]
struct Cave {
    octopuses: Vec<Vec<Octopus>>,
    flashes: i32,
    synced: bool,
}

impl Cave {
    fn new(input: &str) -> Self {
        let mut rows = Vec::new();

        input.lines().for_each(|line| {
            let mut row = Vec::new();
            line.chars().for_each(|ch| {
                let parsed = ch.to_digit(10).unwrap() as usize;
                row.push(Octopus::new(parsed));
            });
            rows.push(row);
        });

        Self {
            octopuses: rows,
            ..Default::default()
        }
    }

    fn flash_neighbors(&mut self, row: i32, col: i32) {
        let row_max = self.octopuses.len() as i32 - 1;
        let col_max = self.octopuses[0].len() as i32 - 1;

        let row_start = if row > 0 { -1 } else { 0 };
        let row_end = if row < row_max { 1 } else { 0 };

        let col_start = if col > 0 { -1 } else { 0 };
        let col_end = if col < col_max { 1 } else { 0 };

        // for each neighbor, increase their energy by one
        for x in row_start..=row_end {
            for y in col_start..=col_end {
                if x != 0 || y != 0 {
                    let x_idx = (row + x) as usize;
                    let y_idx = (col + y) as usize;

                    let neighbor = &mut self.octopuses[x_idx][y_idx];

                    neighbor.energy += 1;

                    if neighbor.energy > 9 && !neighbor.flashed {
                        // neighbor exceeded 9 and hasn't flashed during this step
                        neighbor.flashed = true;
                        self.flashes += 1;
                        self.flash_neighbors(row + x, col + y);
                    }
                }
            }
        }
    }

    fn step(&mut self, count: usize) {
        (0..count).for_each(|_| {
            // First, the energy level of each octopus increases by 1.
            for row in self.octopuses.iter_mut() {
                for octopus in row.iter_mut() {
                    octopus.energy += 1;
                }
            }

            // Then, any octopus with an energy level greater than 9 flashes.
            for i in 0..self.octopuses.len() {
                for ii in 0..self.octopuses[0].len() {
                    if self.octopuses[i][ii].energy > 9 && !self.octopuses[i][ii].flashed {
                        // set current octopus status to 'has flashed'
                        self.octopuses[i][ii].flashed = true;
                        self.flashes += 1;
                        self.flash_neighbors(i as i32, ii as i32);
                    }
                }
            }

            let mut synced = true;

            // Finally, any octopus that flashed during this step has its energy level set to 0
            for row in self.octopuses.iter_mut() {
                for octopus in row.iter_mut() {
                    if !octopus.flashed {
                        synced = false;
                    }
                    if octopus.flashed {
                        octopus.energy = 0;
                        octopus.flashed = false;
                    }
                }
            }

            if synced {
                self.synced = true;
            }
        });
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for row in &self.octopuses {
            for octopus in row {
                result.push_str(&format!("{}", octopus.energy));
            }
            result.push('\n');
        }

        write!(f, "{}", result)
    }
}

#[derive(Debug, Default)]
struct Octopus {
    flashed: bool,
    energy: usize,
}

impl Octopus {
    fn new(energy: usize) -> Self {
        Self {
            energy,
            ..Default::default()
        }
    }
}

fn part_one(input: &str) -> i32 {
    let mut cave = Cave::new(input);

    cave.step(100);
    cave.flashes
}

fn part_two(input: &str) -> i32 {
    let mut cave = Cave::new(input);
    let mut index = 0;

    loop {
        cave.step(1);

        index += 1;

        if cave.synced {
            return index;
        }
    }
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
    static INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_one() {
        assert_eq!(1656, part_one(INPUT));
    }

    #[test]
    fn test_two() {
        assert_eq!(195, part_two(INPUT));
    }
}
