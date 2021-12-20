use advent_of_code_2021::{get_input, parse_args};
use regex::Regex;
use std::collections::HashMap;

fn parse(input: &str) -> HashMap<String, String> {
    let mut rules = HashMap::new();
    let re = Regex::new(r"^([A-Z]{2}) -> ([A-Z])$").unwrap();

    input
        .lines()
        .filter(|line| line.contains("->"))
        .for_each(|line| {
            let parsed = re.captures(line).unwrap();

            let key = parsed[1].to_string();
            let value = parsed[2].to_string();

            rules.insert(key, value);
        });

    rules
}

fn part_one(input: &str) -> usize {
    let mut polymer = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|ch| ch.to_string())
        .collect::<Vec<String>>();

    let rules = parse(input);

    let mut inserts = vec![];

    // 10 steps
    for _step in 0..10 {
        // for each pair of elements, determine what rule applies and track the point in the
        // polymer it needs to be inserted
        for (index, pair) in polymer.windows(2).enumerate() {
            // go from NN to C or similar
            let rule_key = format!("{}{}", pair[0], pair[1]);
            let element = rules.get(&rule_key).unwrap();

            // current offset + 1 + number of insertions that have already occurred gives us the
            // offset at which we place the element into the new polymer
            let insertion_point = index + inserts.len() + 1;
            inserts.push((insertion_point, element));
        }

        // after building the insertions, perform them on the polymer and clear the inserts vec
        // to be ready for the next step
        for (idx, ele) in inserts.iter() {
            polymer.insert(*idx, ele.to_string());
        }
        inserts.clear();

        #[cfg(test)]
        println!("[{}] polymer len: {}", _step, polymer.len());
    }

    let mut counts = HashMap::new();

    for element in polymer {
        *counts.entry(element).or_insert(0) += 1;
    }

    let mut to_sort: Vec<_> = counts.iter().collect();
    to_sort.sort_by(|a, b| a.1.cmp(b.1));

    let max = to_sort[to_sort.len() - 1];
    let min = to_sort[0];

    println!("counts: {:?} {:?} {:?}", to_sort, min, max);

    max.1 - min.1
}

fn part_two(input: &str) -> usize {
    let polymer_input = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|ch| ch.to_string())
        .collect::<Vec<String>>();

    // array to store # of times we've seen an element
    // each element such as 'A' will be stored mapped to the array by subtracting 65 from the ascii
    // value, i.e. 'A' - 65 == 0
    let mut counts = [0_usize; 26];

    let mut polymer = HashMap::new();

    polymer_input.windows(2).for_each(|pair| {
        // pairs from rules, window slides by 2, making it unsuitable for counting
        let rule_key = format!("{}{}", pair[0], pair[1]);
        *polymer.entry(rule_key).or_insert(0) += 1;
    });

    polymer_input.iter().for_each(|element| {
        // initial count values
        counts[element.chars().next().unwrap() as usize - 65] += 1;
    });

    let rules = parse(input);

    for _step in 0..40 {
        // for each pair of elements (polymer's keys), track how many new pairs each step produces
        // by incrementing the value
        let mut temp = polymer.clone();

        for (key, value) in polymer.iter() {
            // go from NN to C or similar
            if let Some(insert) = rules.get(key) {
                // when we see a key of BB and an insert of N (BB -> N)
                // we need to insert BN and NB into the map. Additionally, we need to
                // increment each one by BB's value (if we had 10 BB's we'll now have 10 BN's
                // and 10 NB's)

                let mut chars = key.chars();
                let left_pair = format!("{}{}", chars.next().unwrap(), insert);
                let right_pair = format!("{}{}", insert, chars.next().unwrap());

                *temp.get_mut(key).unwrap() -= value;
                *temp.entry(left_pair).or_insert(0) += value;
                *temp.entry(right_pair).or_insert(0) += value;

                counts[insert.chars().next().unwrap() as usize - 65] += value;
            }
        }

        polymer = temp;
        #[cfg(test)]
        {
            println!("[{}] polymer len: {:#?}", _step, polymer);
            println!("counts: {:?}", counts);
        }
    }

    let mut non_zeroes = counts
        .into_iter()
        .filter(|n| *n > 0)
        .collect::<Vec<usize>>();

    non_zeroes.sort_unstable();
    non_zeroes[non_zeroes.len() - 1] - non_zeroes[0]
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
    static INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_one() {
        assert_eq!(1588, part_one(INPUT));
    }

    #[test]
    fn test_two() {
        assert_eq!(2188189693529, part_two(INPUT));
    }
}
