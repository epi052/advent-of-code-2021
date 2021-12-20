use advent_of_code_2021::{get_input, parse_args};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    risk: usize,
    position: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .risk
            .cmp(&self.risk)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug)]
struct Edge {
    node: (usize, usize),
    cost: usize,
}

impl Edge {
    // fn get_cost(&self, map_len: usize, solver: Solver) -> usize {
    //     match solver {
    //         Solver::PartOne => self.cost,
    //         Solver::PartTwo => {}
    //     }
    // }

    fn neighbors(&self, map: &[Vec<Edge>]) -> Vec<Edge> {
        let row_max = map.len() - 1;
        let col_max = map[0].len() - 1;

        let row_start: i32 = if self.node.0 > 0 { -1 } else { 0 };
        let row_end: i32 = if self.node.0 < row_max { 1 } else { 0 };

        let col_start: i32 = if self.node.1 > 0 { -1 } else { 0 };
        let col_end: i32 = if self.node.1 < col_max { 1 } else { 0 };

        let mut neighbors = Vec::new();

        for x in row_start..=row_end {
            for y in col_start..=col_end {
                let center = x == 0 && y == 0;
                let up_left = x == -1 && y == -1;
                let up_right = x == -1 && y == 1;
                let down_left = x == 1 && y == -1;
                let down_right = x == 1 && y == 1;
                if center || up_left || up_right || down_left || down_right {
                    // skip current node and skip diagonals
                    continue;
                }
                neighbors.push(
                    map[(self.node.0 as i32 + x) as usize][(self.node.1 as i32 + y) as usize]
                        .clone(),
                );
            }
        }

        neighbors
    }
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(
    adj_list: &[Vec<Edge>],
    start: (usize, usize),
    goal: (usize, usize),
) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len())
        .map(|_| {
            (0..adj_list[0].len())
                .map(|_| usize::MAX)
                .collect::<Vec<usize>>()
        })
        .collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start.0][start.1] = 0;
    heap.push(State {
        risk: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { risk, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(risk);
        }

        // Important as we may have already found a better way
        if risk > dist[position.0][position.1] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position.0][position.1].neighbors(adj_list) {
            let next = State {
                risk: risk + edge.cost,
                position: edge.node,
            };

            // If so, add it to the frontier and continue
            if next.risk < dist[next.position.0][next.position.1] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position.0][next.position.1] = next.risk;
            }
        }
    }

    // Goal not reachable
    None
}

fn parse(input: &str) -> Vec<Vec<Edge>> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(ii, ch)| Edge {
                    node: (i, ii),
                    cost: ch.to_string().parse::<usize>().unwrap(),
                })
                .collect::<Vec<Edge>>()
        })
        .collect::<Vec<_>>()
}

fn parse_two(input: &str) -> String {
    let mut temp = String::new();

    for i in 0..5 {
        for line in input.lines() {
            for ii in 0..5 {
                for char in line.chars() {
                    let next = char as u8 + i + ii;
                    let value = if (0x31..=0x39).contains(&next) {
                        // '1' => '9'
                        next as char
                    } else {
                        (next - 9) as char
                    };
                    temp.push_str(&value.to_string());
                }
            }
            temp.push('\n');
        }
    }
    temp
}

fn part_one(input: &str) -> i32 {
    let map = parse(input);

    shortest_path(&map, (0, 0), (map.len() - 1, map[0].len() - 1)).unwrap() as i32
}

fn part_two(input: &str) -> i32 {
    let second = parse_two(input);
    part_one(&second)
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
    static INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test_one() {
        assert_eq!(40, part_one(INPUT));
    }
}
