use advent_of_code_2021::{get_input, parse_args};

fn part_one(input: &str) {}

fn part_two(input: &str) {}

fn main() {
    let args = parse_args();
    let input = get_input(file!());

    match args.part {
        1 => part_one(&input),
        2 => part_two(&input),
        _ => println!(
            "got unexpected value for --part: {} (try 1 or 2)",
            args.part
        ),
    }
}
