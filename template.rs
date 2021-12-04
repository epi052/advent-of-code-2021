use advent_of_code_2021::{get_input, parse_args};

fn part_one(input: &str) -> i32 {
    0
}

fn part_two(input: &str) -> i32 {
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
    static INPUT: &str = "";

    #[test]
    fn test_one() {
        assert_eq!(0, part_one(INPUT));
    }

    #[test]
    fn test_two() {
        assert_eq!(0, part_two(INPUT));
    }
}
