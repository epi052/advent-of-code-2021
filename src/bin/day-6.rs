use advent_of_code_2021::{get_input, parse_args};

#[derive(Clone, Debug, Default)]
struct Model {
    population: Vec<Fish>,
}

impl Model {
    fn new() -> Self {
        Model {
            population: Vec::new(),
        }
    }

    fn tick(&mut self) {
        let mut new_fish_counter = 0;

        for fish in self.population.iter_mut() {
            match fish.timer {
                0 => {
                    //create new fish & reset timer
                    fish.timer = 6;
                    new_fish_counter += 1;
                }
                1..=8 => {
                    fish.timer -= 1;
                }
                _ => {}
            }
        }
        (0..new_fish_counter).for_each(|_| self.population.push(Fish::new(8)));
    }
}

#[derive(Clone, Debug, Default, Copy)]
struct Fish {
    timer: i32,
}

impl Fish {
    fn new(timer: i32) -> Self {
        Fish { timer }
    }
}

fn progress_model(model: &mut Model, days: i32) {
    (0..days).for_each(|d| {
        println!("[+] simulating day {}", d);
        model.tick();
    });
}

fn part_one(input: &str) -> i32 {
    let mut model = Model::new();
    input
        .split(',')
        .for_each(|n| model.population.push(Fish::new(n.parse::<i32>().unwrap())));

    progress_model(&mut model, 80);
    model.population.len() as i32
}

fn part_two(input: &str) -> usize {
    // model days instead of fish
    let mut model = [0; 9];

    input.split(',').for_each(|n| {
        let fish_on_day = n.parse::<usize>().unwrap();
        model[fish_on_day] += 1;
    });

    (0..256).for_each(|_| {
        // rotate left by 1 is equivalent to decrementing each fish's timer by 1
        model.rotate_left(1);

        // need to update the 'old' fish vs. 'new' fish
        // old fish will go to day 6 and new fish go to day 8
        // the day 8 move happens naturally with the rotate left, but we'll need to increment
        // day 6 by the same value as index 8
        model[6] += model[8];
    });

    model.iter().sum()
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
    static INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_one() {
        assert_eq!(5934, part_one(INPUT));
    }

    #[test]
    fn test_two() {
        assert_eq!(26984457539, part_two(INPUT));
    }
}
