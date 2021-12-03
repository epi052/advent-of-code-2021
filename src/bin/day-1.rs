#![feature(array_windows)]
use advent_of_code_2021::{get_input, parse_args};

fn part_one(input: &str) {
    let mut counter = -1;
    let mut last_value = 0;

    for line in input.lines() {
        let value = line.parse::<i32>().unwrap();
        if value > last_value {
            counter += 1;
        }
        last_value = value;
    }

    println!("counter: {}", counter);
}

fn part_two(input: &str) {
    let numbers: Vec<i32> = input.lines().map(|s| s.parse::<i32>().unwrap()).collect();

    let mut last = 0;
    let mut counter = -1;

    for window in numbers.array_windows::<3>() {
        let current = window.iter().sum::<i32>();

        if current > last {
            counter += 1;
        }

        last = current;
    }
    println!("counter: {}", counter);
}

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
