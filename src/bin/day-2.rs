use advent_of_code_2021::{get_input, parse_args};

fn part_one(input: &str) {
    let mut h_pos = 0;
    let mut depth = 0;

    for line in input.lines() {
        let (direction, value) = line.split_once(" ").unwrap();
        let i_value = value.parse::<i32>().unwrap();

        match direction {
            "forward" => h_pos += i_value,
            "up" => depth -= i_value,
            "down" => depth += i_value,
            _ => println!("got an unknown direction: {}", direction),
        }
    }
    println!("horizontal position x depth = {}", h_pos * depth);
}

fn part_two(input: &str) {
    let mut h_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input.lines() {
        let (direction, value) = line.split_once(" ").unwrap();
        let i_value = value.parse::<i32>().unwrap();

        match direction {
            "forward" => {
                h_pos += i_value;
                depth += aim * i_value;
            }
            "up" => aim -= i_value,
            "down" => aim += i_value,
            _ => println!("got an unknown direction: {}", direction),
        }
    }
    println!("horizontal position x depth = {}", h_pos * depth);
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
