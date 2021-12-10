use advent_of_code_2021::{get_input, parse_args};
use std::collections::HashMap;

fn part_one(input: &str) -> i32 {
    let mut stack = Vec::new();
    let lookup = HashMap::from([(')', '('), (']', '['), ('>', '<'), ('}', '{')]);

    let mut score = 0;
    let scores = HashMap::from([(')', 3), (']', 57), ('>', 25137), ('}', 1197)]);

    for line in input.lines() {
        for ch in line.chars() {
            match ch {
                '(' | '[' | '{' | '<' => {
                    stack.push(ch);
                }
                ')' | ']' | '}' | '>' => {
                    let opener = stack.pop().unwrap();
                    let expected = lookup.get(&ch).unwrap();
                    if opener != *expected {
                        score += scores.get(&ch).unwrap();
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    score
}

fn score_part_two(closers: &[char]) -> usize {
    let scores = HashMap::from([('(', 1), ('[', 2), ('<', 4), ('{', 3)]);

    closers.iter().fold(0, |mut acc, ch| {
        acc = acc * 5 + scores.get(ch).unwrap();
        acc
    })
}

fn part_two(input: &str) -> usize {
    let mut scores = Vec::new();
    let lookup = HashMap::from([
        (')', '('),
        (']', '['),
        ('>', '<'),
        ('}', '{'),
        ('(', ')'),
        ('[', ']'),
        ('<', '>'),
        ('{', '}'),
    ]);

    'outer: for line in input.lines() {
        // fresh stack per line this time, as the remainder will be used to determine the answer
        let mut stack = Vec::new();

        for ch in line.chars() {
            match ch {
                '(' | '[' | '{' | '<' => {
                    stack.push(ch);
                }
                ')' | ']' | '}' | '>' => {
                    let opener = stack.pop().unwrap();
                    let expected = lookup.get(&ch).unwrap();
                    if opener != *expected {
                        // malformed lines aren't considered for part two, just go to next line
                        continue 'outer;
                    }
                }
                _ => unreachable!(),
            }
        }

        // in reaching the end of the character iterator, we have an incomplete line (malformed are
        // discarded), so we can score it for later

        stack.reverse(); // score needs to be computed in the reverse order of the stack
        scores.push(score_part_two(&stack));
    }

    scores.sort_unstable();
    scores[scores.len() / 2]
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
    static INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_one() {
        assert_eq!(26397, part_one(INPUT));
    }

    #[test]
    fn test_two() {
        assert_eq!(288957, part_two(INPUT));
    }
}
