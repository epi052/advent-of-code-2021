use advent_of_code_2021::{get_input, parse_args};
use regex::Regex;
use std::cmp;
use std::ops::RangeInclusive;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Solver {
    PartOne,
    PartTwo,
}

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    covered_by: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug)]
struct Line {
    slope: i32,
    points: (Point, Point),
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
}

impl Line {
    fn new(p1: Point, p2: Point) -> Self {
        let left_x = cmp::min(p1.x, p2.x);
        let right_x = cmp::max(p1.x, p2.x);
        let left_y = cmp::min(p1.y, p2.y);
        let right_y = cmp::max(p1.y, p2.y);
        let slope = (p1.y - p2.y).checked_div(p1.x - p2.x).unwrap_or(0);

        Self {
            points: (p1, p2),
            x_range: (left_x..=right_x),
            y_range: (left_y..=right_y),
            slope,
        }
    }
}

impl Line {
    fn contains(&self, point: Point, solver: Solver) -> bool {
        if self.is_vertical() && self.points.0.x == point.x {
            // need to check y values
            self.y_range.contains(&point.y)
        } else if self.is_horizontal() && self.points.0.y == point.y {
            self.x_range.contains(&point.x)
        } else if self.is_diagonal() && solver == Solver::PartTwo {
            self.slope * (point.x - self.points.0.x) == point.y - self.points.0.y
                && self.x_range.contains(&point.x)
                && self.y_range.contains(&point.y)
        } else {
            false
        }
    }

    fn is_vertical(&self) -> bool {
        self.points.0.x == self.points.1.x
    }

    fn is_horizontal(&self) -> bool {
        self.points.0.y == self.points.1.y
    }

    fn is_diagonal(&self) -> bool {
        let lhs =
            cmp::max(self.points.0.x, self.points.1.x) - cmp::min(self.points.1.x, self.points.0.x);
        let rhs =
            cmp::max(self.points.0.y, self.points.1.y) - cmp::min(self.points.1.y, self.points.0.y);
        lhs == rhs
    }
}

#[derive(Clone, Debug, Default)]
struct Graph {
    lines: Vec<Line>,
    x_boundary: i32,
    y_boundary: i32,
    covered_points: Vec<Point>,
}

impl Graph {
    fn new(input: &str) -> Self {
        let mut line_segments = vec![];
        let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();

        // track edges of the diagram, i.e. largest x and y we see while parsing
        let mut largest_x = 0;
        let mut largest_y = 0;

        for line in input.lines() {
            let parsed = re.captures(line).unwrap();

            let x1 = parsed[1].parse::<i32>().unwrap();
            let y1 = parsed[2].parse::<i32>().unwrap();
            let x2 = parsed[3].parse::<i32>().unwrap();
            let y2 = parsed[4].parse::<i32>().unwrap();

            // grab the largest of the three current numbers for both x and y
            largest_x = cmp::max(largest_x, cmp::max(x1, x2));
            largest_y = cmp::max(largest_y, cmp::max(y1, y2));

            let p1 = Point::new(x1, y1);
            let p2 = Point::new(x2, y2);

            line_segments.push(Line::new(p1, p2))
        }
        #[cfg(test)]
        {
            assert_eq!(10, line_segments.len());
            assert_eq!(9, largest_y);
            assert_eq!(9, largest_x);
        }

        Self {
            lines: line_segments,
            x_boundary: largest_x,
            y_boundary: largest_y,
            covered_points: Vec::<Point>::new(),
        }
    }
}

fn part_one(input: &str, solver: Solver) -> i32 {
    let mut graph = Graph::new(input);

    // iterating up to and including the largest known values of x and y, since we saw them while
    // parsing, meaning they're valid points
    for row in 0..=graph.x_boundary {
        for column in 0..=graph.y_boundary {
            // get the current point on the graph
            let mut current = Point::new(row, column);

            for line in &graph.lines {
                // each line is a line segment from the given input, check if any of the lines
                // cover the current point
                if line.contains(current, solver) {
                    current.covered_by += 1;
                }
            }

            if current.covered_by > 1 && !graph.covered_points.contains(&current) {
                // for any point that's covered by at least 2 lines, add it to the vec of ones we
                // care about
                graph.covered_points.push(current);
            }
        }
    }

    graph.covered_points.len() as i32
}

fn part_two(input: &str, solver: Solver) -> i32 {
    part_one(input, solver)
}

fn main() {
    let args = parse_args();
    let input = get_input(file!());

    match args.part {
        1 => println!("{}", part_one(&input, Solver::PartOne)),
        2 => println!("{}", part_two(&input, Solver::PartTwo)),
        _ => println!(
            "got unexpected value for --part: {} (try 1 or 2)",
            args.part
        ),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_contains_part_one() {
        let line = Line::new(Point::new(7, 0), Point::new(7, 4));

        for i in 0..=4 {
            assert!(line.contains(Point::new(7, i), Solver::PartOne));
        }

        let line = Line::new(Point::new(7, 0), Point::new(3, 0));
        for i in 3..=7 {
            assert!(line.contains(Point::new(i, 0), Solver::PartOne));
        }
    }

    #[test]
    fn test_contains_part_two() {
        let line = Line::new(Point::new(1, 1), Point::new(3, 3));
        assert!(line.contains(Point::new(1, 1), Solver::PartTwo));
        assert!(line.contains(Point::new(2, 2), Solver::PartTwo));
        assert!(line.contains(Point::new(3, 3), Solver::PartTwo));

        let line = Line::new(Point::new(9, 7), Point::new(7, 9));
        assert!(line.contains(Point::new(9, 7), Solver::PartTwo));
        assert!(line.contains(Point::new(8, 8), Solver::PartTwo));
        assert!(line.contains(Point::new(7, 9), Solver::PartTwo));

        assert!(!Line::new(Point::new(8, 0), Point::new(0, 8))
            .contains(Point::new(0, 0), Solver::PartTwo));
    }

    #[test]
    fn test_one() {
        assert_eq!(5, part_one(INPUT, Solver::PartOne));
    }

    #[test]
    fn test_two() {
        assert_eq!(12, part_two(INPUT, Solver::PartTwo));
    }
}
