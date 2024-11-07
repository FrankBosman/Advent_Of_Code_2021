use std::collections::HashMap;
use Advent_Of_Code_2021::{read_in, Point};
use colored::Colorize;
const TEST_MODE: bool = false;

fn main() {
    println!("\n{}", "AOC Day 5".bright_green().bold());
    let lines = read_in::lines(if TEST_MODE { "src/day5/testInput.txt" } else { "src/day5/input.txt" });

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let paths = process_data(&lines, false);

    let answer = count_overlapping(paths);
    println!("{}","Part 1".green());
    println!("- {}", answer.to_string().yellow());
}

fn part2(lines: &Vec<String>) {
    let paths = process_data(&lines, true);

    let answer = count_overlapping(paths);
    println!("{}","Part 2".green());
    println!("- {}", answer.to_string().yellow());
}

fn process_data(lines: &Vec<String>, part2: bool) -> Vec<Vec<Point>> {
    let mut paths: Vec<Vec<Point>> = Vec::new();

    for line in lines.iter() {
        let points = line.split("->").collect::<Vec<&str>>();
        let start_point = Point::from(&points[0]);
        let end_point = Point::from(&points[1]);

        if part2 || (start_point.x == end_point.x) || (start_point.y == end_point.y) {
            paths.push(get_path(&start_point, &end_point));
        }
    }

    paths
}


fn get_path(start: &Point, end: &Point) -> Vec<Point> {
    let mut path: Vec<Point> = Vec::new();
    // Vertical
    if start.x == end.x {
        if start.y < end.y {
            for y in start.y..=end.y {
                path.push(Point::new(start.x, y));
            }
        } else {
            for y in end.y..=start.y {
                path.push(Point::new(start.x, y));
            }
        }
    }
    // Horizontal
    else if start.y == end.y {
        if start.x < end.x{
            for x in start.x..=end.x {
                path.push(Point::new(x, start.y));
            }
        } else {
            for x in end.x..=start.x {
                path.push(Point::new(x, start.y));
            }
        }
    }
    // Diagonal
    else {
        let x_increment = if start.x < end.x {1} else {-1};
        let y_increment = if start.y < end.y {1} else {-1};
        let steps = x_increment * (end.x - start.x);
        for i in 0..=steps {
            path.push(Point::new(start.x + i * x_increment, start.y + i * y_increment));
        }        
    }

    path
}

fn count_overlapping(paths: Vec<Vec<Point>>) -> usize {
    let mut map: HashMap<Point, i16> = HashMap::new();
    for path in paths {
        for point in path {
            if map.contains_key(&point) {
                if let Some(x) = map.get_mut(&point) {
                    *x += 1;
                }
            } else {
                map.insert(point, 1);
            }
        }
    }

    map.iter().filter(|&(_, &v)| v >= 2).count()
}