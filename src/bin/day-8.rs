use advent_of_code_2021::{get_input, parse_args, Solver};
use std::collections::HashMap;

fn sort_signals(signal: &str) -> String {
    let mut chars = signal.chars().collect::<Vec<char>>();
    chars.sort_unstable();
    String::from_iter(chars)
}

fn contains_all(left: &str, right: &str) -> bool {
    left.chars().all(|c| right.contains(c))
}

fn filter_all(left: &str, right: &str) -> String {
    left.chars()
        .filter(|c| !right.contains(*c))
        .collect::<String>()
}

fn add_signal_to_map(
    signal: &str,
    name: &str,
    capacity: usize,
    known: &mut HashMap<String, String>,
) {
    let sorted = sort_signals(signal);
    known.insert(sorted.clone(), String::with_capacity(capacity));
    known.insert(String::from(name), sorted);
}

/// returns i32 for part one, while part two relies on manipulating the `known` hashmap
fn determine_digit(signal: &str, known: &mut HashMap<String, String>, solver: Solver) -> i32 {
    match (signal.len(), solver) {
        // easy, used by both parts one and two
        (2, _) => {
            add_signal_to_map(signal, "one", 1, known);
            1
        }
        (3, _) => {
            add_signal_to_map(signal, "seven", 7, known);
            7
        }
        (4, _) => {
            add_signal_to_map(signal, "four", 4, known);
            4
        }
        (7, _) => {
            add_signal_to_map(signal, "eight", 8, known);
            8
        }
        // less easy, but due to sorting the initial signals array, we know that one and four are
        // always set before processing anything below
        (5, Solver::PartTwo) => {
            // possible: 2, 3, 5
            // sort strings for proper membership comparison

            let five_identifier = filter_all(known.get("four").unwrap(), known.get("one").unwrap());

            if contains_all(known.get("one").unwrap(), signal) {
                add_signal_to_map(signal, "three", 3, known);
                3
            } else if contains_all(&five_identifier, signal) {
                add_signal_to_map(signal, "five", 5, known);
                5
            } else {
                add_signal_to_map(signal, "two", 2, known);
                2
            }
        }
        (6, Solver::PartTwo) => {
            // 0, 6, 9
            let two_identifier = filter_all(known.get("two").unwrap(), known.get("three").unwrap());

            if contains_all(known.get("one").unwrap(), signal)
                && contains_all(known.get("five").unwrap(), signal)
            {
                add_signal_to_map(signal, "nine", 9, known);
                9
            } else if contains_all(known.get("five").unwrap(), signal)
                && contains_all(&two_identifier, signal)
            {
                add_signal_to_map(signal, "six", 6, known);
                6
            } else {
                add_signal_to_map(signal, "zero", 0, known);
                2
            }
        }
        _ => -1,
    }
}

fn part_one(input: &str) -> i32 {
    let mut known = HashMap::new(); // unused for part 1

    input.lines().fold(0, |mut acc, line| {
        let mut line = line.split('|');

        line.next().unwrap(); // discard signal patterns
        let output_value = line.next().unwrap();

        acc += output_value
            .split_whitespace()
            .filter(|ov| determine_digit(ov, &mut known, Solver::PartOne) > 0)
            .count();
        acc
    }) as i32
}

fn part_two(input: &str) -> i32 {
    let mut result = 0;

    for line in input.lines() {
        let mut known = HashMap::new(); // reset known for each new input

        let mut line = line.split('|');
        let mut signal_patterns = line
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();

        // sort the signals so we process 1 first (len 2), 7 second (len 3), etc...
        signal_patterns.sort_by_key(|left| left.len());
        let output_value = line.next().unwrap();

        for sp in signal_patterns {
            determine_digit(sp, &mut known, Solver::PartTwo);
        }

        // go from "fgae fg ..." -> 41..
        let current = output_value
            .split_whitespace()
            .fold(String::new(), |mut acc, ov| {
                let sorted = sort_signals(ov);
                acc.push_str(&format!("{}", known.get(&sorted).unwrap().capacity()));
                acc
            })
            .parse::<i32>()
            .unwrap();

        result += current;
    }

    result
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
    static INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_one() {
        assert_eq!(26, part_one(INPUT));
    }

    #[test]
    fn test_two() {
        assert_eq!(61229, part_two(INPUT));
    }
}
