use advent_of_code_2021::{get_input, parse_args, Solver};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
struct Dot {
    x: usize,
    y: usize,
}

impl Dot {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Default)]
struct Paper {
    dots: HashSet<Dot>,
}

impl Display for Paper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        // get largest x/y values
        let max_x = self.dots.iter().max_by_key(|dot| dot.x).unwrap();
        let max_y = self.dots.iter().max_by_key(|dot| dot.y).unwrap();

        for row in 0..=max_y.y {
            for col in 0..=max_x.x {
                match self.dots.get(&Dot::new(col, row)) {
                    Some(_) => result.push('#'),
                    None => result.push('.'),
                }
            }
            result.push('\n');
        }

        write!(f, "{}", result)
    }
}

impl Paper {
    fn new(dots: HashSet<Dot>) -> Self {
        Self { dots }
    }

    fn fold(&mut self, instructions: &[String], solver: Solver) {
        for instr in instructions {
            let mut split = instr.split('=');
            let (axis, fold_line) = (
                split.next().unwrap(),
                split.next().unwrap().parse::<usize>().unwrap(),
            );

            #[cfg(test)]
            println!("[INSTRUCTION] fold along {}", instr);

            // can't iter_mut a hashset, and can't do an immutable borrow while we alter the set
            // from within the loop, so clone
            for dot in self.dots.clone().iter() {
                match axis {
                    "x" => {
                        if dot.x > fold_line {
                            // somewhere to the right the fold line
                            let distance_from_fold = dot.x - fold_line;

                            self.dots.remove(dot);
                            self.dots
                                .insert(Dot::new(fold_line - distance_from_fold, dot.y));
                        }
                    }
                    "y" => {
                        if dot.y > fold_line {
                            // somewhere below the fold line
                            let distance_from_fold = dot.y - fold_line;

                            self.dots.remove(dot);
                            self.dots
                                .insert(Dot::new(dot.x, fold_line - distance_from_fold));
                        }
                    }
                    _ => unreachable!(),
                }
            }
            if matches!(solver, Solver::PartOne) {
                break; // break out after first instruction
            }
        }
    }
}

fn parse(input: &str) -> (HashSet<Dot>, Vec<String>) {
    let dots = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut split = line.split(',');
            Dot::new(
                split.next().unwrap().parse::<usize>().unwrap(),
                split.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<HashSet<Dot>>();

    let instructions = input
        .lines()
        .skip_while(|line| !line.starts_with("fold"))
        .map(|line| {
            let mut split = line.split_whitespace();
            split.next().unwrap(); // fold
            split.next().unwrap(); // along
            split.next().unwrap().to_string() // x=5 or w/e
        })
        .collect::<Vec<String>>();

    (dots, instructions)
}

fn part_one(input: &str) -> i32 {
    let (dots, instructions) = parse(input);

    let mut paper = Paper::new(dots);

    #[cfg(test)]
    println!("{:?}", paper.dots);

    paper.fold(&instructions, Solver::PartOne);

    #[cfg(test)]
    println!("{:#?}", paper.dots);

    paper.dots.len() as i32
}

fn part_two(input: &str) -> i32 {
    let (dots, instructions) = parse(input);

    let mut paper = Paper::new(dots);
    paper.fold(&instructions, Solver::PartTwo);

    println!("{}", paper);

    0
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
    static INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_one() {
        assert_eq!(17, part_one(INPUT));
    }

    #[test]
    fn test_two() {
        assert_eq!(0, part_two(INPUT));
    }
}
