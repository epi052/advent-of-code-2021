use advent_of_code_2021::{get_input, parse_args, Solver};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum Cave {
    Small(String),
    Big(String),
    Start,
    End,
}

impl FromStr for Cave {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(Cave::Start),
            "end" => Ok(Cave::End),
            _ => {
                if s.chars().all(|ch| ch.is_lowercase()) {
                    Ok(Cave::Small(s.to_string()))
                } else {
                    Ok(Cave::Big(s.to_string()))
                }
            }
        }
    }
}

#[derive(Debug, Default)]
struct Caves {
    map: HashMap<Cave, Vec<Cave>>,
}

impl FromStr for Caves {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut caves = Caves::default();

        s.lines().for_each(|line| {
            let mut split = line.split('-');
            let (key, value) = (split.next().unwrap(), split.next().unwrap());

            // convert from str to cave
            let key = Cave::from_str(key).unwrap();
            let value = Cave::from_str(value).unwrap();

            // get the key's vec if it exists, otherwise, create a new vec and either way insert
            // the value
            caves
                .map
                .entry(key.clone())
                .or_insert_with(Vec::new)
                .push(value.clone());

            caves.map.entry(value).or_insert_with(Vec::new).push(key);
        });

        Ok(caves)
    }
}

impl Caves {
    fn count_paths(&mut self, solver: Solver) -> usize {
        let mut path = Vec::new();
        let mut count = 0;

        match solver {
            Solver::PartOne => self.explore_one(Cave::Start, &mut path, &mut count),
            Solver::PartTwo => self.explore_two(Cave::Start, &mut path, &mut count, false),
        };

        count
    }

    fn neighbors(&self, cave: &Cave) -> Vec<Cave> {
        self.map.get(cave).unwrap().clone()
    }

    fn explore_two(
        &mut self,
        cave: Cave,
        path: &mut Vec<Cave>,
        count: &mut usize,
        visited_twice: bool,
    ) {
        path.push(cave.clone()); // add current cave to path

        // println!("[{:?}] {:?} ? {}", cave, path, visited_twice);

        // for every neighbor, recursively explore paths available to that neighbor, incrementing
        // count for every fully explored path
        for neighbor in self.neighbors(&cave) {
            if matches!(neighbor, Cave::Start) {
                // don't revisit the start
                continue;
            }

            if matches!(neighbor, Cave::End) {
                // increment number of paths by 1 when we reach the end
                #[cfg(test)]
                println!("[END] - {:?}", path);
                *count += 1;
            }

            if matches!(neighbor, Cave::Big(_)) {
                // big caves can be re-visited, just yeet them when seen
                self.explore_two(neighbor.clone(), path, count, visited_twice);
            }

            if matches!(neighbor, Cave::Small(_)) && (!visited_twice || !path.contains(&neighbor)) {
                let two_visit_sentry = visited_twice || path.contains(&neighbor);
                self.explore_two(neighbor, path, count, two_visit_sentry);
            }
        }

        // remove current cave
        path.pop();
    }

    fn explore_one(&mut self, cave: Cave, path: &mut Vec<Cave>, count: &mut usize) {
        path.push(cave.clone()); // add current cave to path

        // for every neighbor, recursively explore paths available to that neighbor, incrementing
        // count for every fully explored path
        for neighbor in self.neighbors(&cave) {
            if matches!(neighbor, Cave::Start) {
                // don't revisit the start
                continue;
            }

            if matches!(neighbor, Cave::End) {
                // increment number of paths by 1 when we reach the end
                #[cfg(test)]
                println!("[END] - {:?}", path);
                *count += 1;
            }

            if matches!(neighbor, Cave::Big(_)) {
                // big caves can be re-visited, just yeet them when seen
                self.explore_one(neighbor.clone(), path, count);
            }

            if matches!(neighbor, Cave::Small(_)) && !path.contains(&neighbor) {
                // small caves need to not already be in the path to be yote
                self.explore_one(neighbor.clone(), path, count);
            }
        }

        // remove current cave
        path.pop();
    }
}

fn part_one(input: &str) -> i32 {
    let mut caves = Caves::from_str(input).unwrap();
    caves.count_paths(Solver::PartOne) as i32
}

fn part_two(input: &str) -> i32 {
    let mut caves = Caves::from_str(input).unwrap();
    caves.count_paths(Solver::PartTwo) as i32
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
    static INPUT: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    static INPUT1: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
    static INPUT2: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn test_one() {
        assert_eq!(10, part_one(INPUT));
        assert_eq!(19, part_one(INPUT1));
        assert_eq!(226, part_one(INPUT2));
    }

    #[test]
    fn test_two() {
        // assert_eq!(36, part_two(INPUT));
        // assert_eq!(103, part_two(INPUT));
        // assert_eq!(3509, part_two(INPUT));
    }
}
