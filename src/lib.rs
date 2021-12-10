use clap::Parser;
use std::fs::read_to_string;
use std::path::Path;

/// cli for aoc binaries
#[derive(Parser)]
pub struct AoCArgParser {
    /// which part of the day
    #[clap(short, long)]
    pub part: i32,
}

/// parse cli arguments
pub fn parse_args() -> AoCArgParser {
    AoCArgParser::parse()
}

/// read input file from src/inputs based on current file's filename
pub fn get_input(filename: &str) -> String {
    let this_file = Path::new(filename);
    let mut stem = this_file.file_stem().unwrap().to_str().unwrap().split('-');

    stem.next().unwrap(); // discard 'day'
    let current_day = stem.next().unwrap();

    let input_path = format!("src/inputs/input-{}", current_day);

    read_to_string(input_path).expect("couldn't read input file")
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Solver {
    PartOne,
    PartTwo,
}
