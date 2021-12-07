use advent_of_code_2021::{get_input, parse_args, Solver};

fn movement_cost(left: i32, right: i32, solver: Solver) -> i32 {
    match solver {
        Solver::PartOne => (left - right).abs(),
        Solver::PartTwo => {
            let distance = (left - right).abs();
            (0..=distance).fold(0, |mut acc, ele| {
                acc += ele;
                acc
            })
        }
    }
}

fn part_one(input: &str, solver: Solver) -> i32 {
    let mut crabs = input
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    crabs.sort_unstable(); // get upper bounds as last item in array
    let (mut position, mut least_cost) = (0, i32::MAX);

    (0..crabs[crabs.len() - 1]).for_each(|pos| {
        // 0 to the maximum crab's position, i.e. 16 in the testcase
        let fuel_cost = crabs.iter().fold(0, |mut acc, crab| {
            // add up the distance between the current position from outer loop and each crab
            // then store the result in the positions array
            acc += movement_cost(pos, *crab, solver);
            acc
        });

        if fuel_cost < least_cost {
            least_cost = fuel_cost;
            position = pos;
        }
    });

    least_cost
}

fn part_two(input: &str) -> i32 {
    part_one(input, Solver::PartTwo)
}

fn main() {
    let args = parse_args();
    let input = get_input(file!());

    match args.part {
        1 => println!("{}", part_one(&input, Solver::PartOne)),
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
    static INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_one() {
        assert_eq!(37, part_one(INPUT, Solver::PartOne));
    }

    #[test]
    fn test_two() {
        assert_eq!(168, part_two(INPUT));
    }
}
