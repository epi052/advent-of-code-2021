use advent_of_code_2021::{get_input, parse_args};

#[derive(Copy, Clone, Debug)]
pub enum Rating {
    Oxygen,
    CO2,
}

fn count_bits(input: &str) -> (Vec<Vec<char>>, [i32; 12]) {
    let mut counter = [0; 12];

    // [
    //   ['1', '0', '1', '1', '1', '1', '1', '0', '1', '0', '0', '0'],
    //   ['1', '1', '0', '1', '1', '0', '0', '1', '1', '1', '0', '0'],
    //   ...
    // ]
    let two_d_vec = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // most common bit will be determined by positive/negative
    two_d_vec.iter().for_each(|row| {
        row.iter().enumerate().for_each(|(idx, ch)| match ch {
            '0' => counter[idx] -= 1,
            '1' => counter[idx] += 1,
            _ => {}
        })
    });

    (two_d_vec, counter)
}

fn part_one(input: &str) -> i32 {
    // gamma rate can be determined by finding the most common bit in the corresponding position
    // of all numbers in the diagnostic report
    let (_, gamma_ctr) = count_bits(input);

    // string builder to arrive at something like
    // gamma: 000100011100
    // epsilon: 111011100011
    let (gamma, epsilon) =
        gamma_ctr
            .iter()
            .fold((String::new(), String::new()), |(gam, eps), ele| {
                if *ele > 0 {
                    (format!("{}{}", gam, "1"), format!("{}{}", eps, "0"))
                } else {
                    (format!("{}{}", gam, "0"), format!("{}{}", eps, "1"))
                }
            });

    let gamma = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon, 2).unwrap();

    println!("gamma x epsilon = {}", gamma * epsilon);
    gamma * epsilon
}

pub fn popularity_contest(index: usize, numbers: &[Vec<char>], rating: Rating) -> char {
    let mut counter = 0;

    for number in numbers {
        match number[index] {
            '0' => counter -= 1,
            '1' => counter += 1,
            _ => {}
        }
    }

    match (counter, rating) {
        // most common is 0 when counter is negative, 1 when positive
        (i32::MIN..=-1, Rating::Oxygen) => '0',
        (i32::MIN..=-1, Rating::CO2) => '1',
        (0, Rating::Oxygen) => '1',
        (0, Rating::CO2) => '0',
        (1..=i32::MAX, Rating::Oxygen) => '1',
        (1..=i32::MAX, Rating::CO2) => '0',
    }
}

fn find_life_support_component_rating(diagnostics: &[Vec<char>], rating: Rating) -> i32 {
    let mut filterable = diagnostics.to_owned();

    for col_idx in 0..diagnostics[0].len() {
        // iterate for given number of columns, getting the most common bit for each index
        let polularest = popularity_contest(col_idx, &filterable, rating);
        let filterable_len = filterable.len(); // can't call .len from into_iter

        // filter out those that don't match, unless there's only one value in the vector
        filterable = filterable
            .into_iter()
            .filter(|value| {
                if filterable_len == 1 {
                    true
                } else {
                    value[col_idx] == polularest
                }
            })
            .collect();
    }

    // convert from Vec<char> -> binary String -> i32
    i32::from_str_radix(&filterable[0].iter().collect::<String>(), 2).unwrap()
}

fn part_two(input: &str) -> i32 {
    let (two_d_vec, _) = count_bits(input);

    let oxygen_rating = find_life_support_component_rating(&two_d_vec, Rating::Oxygen);
    let co2_rating = find_life_support_component_rating(&two_d_vec, Rating::CO2);

    println!("oxygen x co2 = {}", oxygen_rating * co2_rating);

    oxygen_rating * co2_rating
}

fn main() {
    let args = parse_args();
    let input = get_input(file!());

    match args.part {
        1 => part_one(&input),
        2 => part_two(&input),
        _ => {
            println!(
                "got unexpected value for --part: {} (try 1 or 2)",
                args.part
            );
            0
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_two() {
        assert_eq!(230, part_two(INPUT));
    }
}
