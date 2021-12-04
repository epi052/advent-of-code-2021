use advent_of_code_2021::{get_input, parse_args};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Default)]
struct Number {
    value: i32,
    called: bool,
}

#[derive(Debug, Default, Clone)]
struct Board {
    rows: Vec<Vec<Number>>,
    index: usize,
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for row in &self.rows {
            for num in row {
                let repr = if num.called {
                    String::from("XX ")
                } else {
                    format!("{:02} ", num.value)
                };
                result.push_str(&repr);
            }
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}

impl Board {
    fn check_vertical(&self) -> bool {
        for i in 0..5 {
            if self.rows[0][i].called
                && self.rows[1][i].called
                && self.rows[2][i].called
                && self.rows[3][i].called
                && self.rows[4][i].called
            {
                return true;
            }
        }

        false
    }

    fn check_horizontal(&self) -> bool {
        self.rows.iter().any(|row| row.iter().all(|num| num.called))
    }

    fn check(&self) -> bool {
        self.check_vertical() || self.check_horizontal()
    }

    fn mark(&mut self, called_number: i32) {
        for row in self.rows.iter_mut() {
            for mut num in row {
                if num.value == called_number {
                    num.called = true;
                }
            }
        }
    }

    fn score(&self) -> i32 {
        self.rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|num| if !num.called { num.value } else { 0 })
                    .sum::<i32>()
            })
            .sum()
    }
}

fn build_boards(input: &str) -> (Vec<i32>, Vec<Board>) {
    let mut lines = input.lines();
    let called_nums = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut boards = vec![];

    lines.next().unwrap(); // discard first blank line

    let mut board = Board::default();

    for line in lines {
        if line.is_empty() {
            // start of new board
            boards.push(board.clone());
            board = Board::default();
            continue;
        }

        let row = line
            .split_whitespace()
            .map(|n| Number {
                value: n.parse::<i32>().unwrap(),
                called: false,
            })
            .collect::<Vec<Number>>();

        board.index = boards.len();
        board.rows.push(row);
    }

    boards.push(board.clone()); // push last board

    (called_nums, boards)
}

fn part_one(input: &str) -> i32 {
    let (called_numbers, mut boards) = build_boards(input);

    for called_number in called_numbers {
        for board in boards.iter_mut() {
            board.mark(called_number);
            if board.check() {
                println!("{}", board);
                println!(
                    "{} x {} = {}",
                    board.score(),
                    called_number,
                    board.score() * called_number
                );
                return board.score() * called_number;
            }
        }
    }
    0
}

fn part_two(input: &str) -> i32 {
    let (called_numbers, mut boards) = build_boards(input);

    let mut last_winner = Board::default();
    let mut last_called = 0;
    let mut done: Vec<usize> = vec![];

    for called_number in called_numbers {
        for board in boards.iter_mut() {
            board.mark(called_number);
            if board.check() {
                if !done.contains(&board.index) {
                    last_winner = board.clone();
                    last_called = called_number;
                    done.push(board.index);
                }
                continue;
            }
        }
    }

    println!("{}", last_winner);
    println!(
        "{} x {} = {}",
        last_winner.score(),
        last_called,
        last_winner.score() * last_called
    );

    last_winner.score() * last_called
}

fn main() {
    let args = parse_args();
    let input = get_input(file!());

    match args.part {
        1 => part_one(&input),
        2 => part_two(&input),
        _ => {
            println!(
                "got unexpected value for --part: {} (try 1 or 2)",
                args.part
            );
            0
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_one() {
        assert_eq!(4512, part_one(INPUT));
    }

    #[test]
    fn test_two() {
        assert_eq!(1924, part_two(INPUT));
    }
}
