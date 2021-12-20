use advent_of_code_2021::{get_input, parse_args};
use regex::Regex;

#[derive(Debug, Default)]
struct Velocity {
    horizontal: i32,
    vertical: i32,
}

impl Velocity {
    fn new(horizontal: i32, vertical: i32) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }
    fn highest_vertical(&self) -> i32 {
        self.vertical * (self.vertical + 1) / 2
    }
}

#[derive(Debug, Default)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Default)]
struct Target {
    lower_x: i32,
    upper_x: i32,
    lower_y: i32,
    upper_y: i32,
}

#[derive(Debug, Default)]
struct Probe {
    position: Position,
    velocity: Velocity,
    target: Target,
}

impl Probe {
    fn new(lower_x: i32, upper_x: i32, lower_y: i32, upper_y: i32) -> Self {
        Self {
            target: Target {
                lower_x,
                upper_x,
                lower_y,
                upper_y,
            },
            ..Default::default()
        }
    }

    fn reached_target(&self) -> bool {
        let inside_x =
            self.target.lower_x <= self.position.x && self.position.x <= self.target.upper_x;

        let inside_y =
            self.target.lower_y <= self.position.y && self.position.y <= self.target.upper_y;

        inside_x && inside_y
    }

    fn reaches_target(&mut self) -> bool {
        self.position = Position::default(); // 0,0
        loop {
            self.position.x += self.velocity.horizontal;
            self.position.y += self.velocity.vertical;
            self.velocity.horizontal = std::cmp::max(0, self.velocity.horizontal - 1);
            self.velocity.vertical -= 1;

            if self.position.x > self.target.upper_x || self.position.y < self.target.lower_y {
                return false;
            }

            if self.reached_target() {
                return true;
            }
        }
    }

    fn min_velocity_to_target(&self) -> Velocity {
        let horizontal = (self.target.lower_x as f32 * 2.0).sqrt() as i32;
        let vertical = self.target.lower_y;
        Velocity {
            horizontal,
            vertical,
        }
    }

    fn max_velocity_to_target(&self) -> Velocity {
        let horizontal = self.target.upper_x;
        let vertical = (self.target.lower_y + 1).abs();
        Velocity {
            horizontal,
            vertical,
        }
    }
}

fn parse(input: &str) -> (i32, i32, i32, i32) {
    let re = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
    let captures = re.captures(input).unwrap();
    (
        captures[1].parse::<i32>().unwrap(),
        captures[2].parse::<i32>().unwrap(),
        captures[3].parse::<i32>().unwrap(),
        captures[4].parse::<i32>().unwrap(),
    )
}

fn part_one(input: &str) -> i32 {
    let bounds = parse(input);
    let probe = Probe::new(bounds.0, bounds.1, bounds.2, bounds.3);
    let max_velocity = probe.max_velocity_to_target();
    max_velocity.highest_vertical()
}

fn part_two(input: &str) -> i32 {
    let bounds = parse(input);
    let mut probe = Probe::new(bounds.0, bounds.1, bounds.2, bounds.3);

    let min_velocity = probe.min_velocity_to_target();
    let max_velocity = probe.max_velocity_to_target();
    let mut valid_velocities = 0;

    (min_velocity.horizontal..=max_velocity.horizontal).for_each(|h| {
        (min_velocity.vertical..=max_velocity.vertical).for_each(|v| {
            let velocity = Velocity::new(h, v);
            probe.velocity = velocity;
            if probe.reaches_target() {
                valid_velocities += 1;
            }
        })
    });

    valid_velocities
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
    static INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_one() {
        assert_eq!(45, part_one(INPUT));
    }

    #[test]
    fn test_two() {
        assert_eq!(112, part_two(INPUT));
    }
}
