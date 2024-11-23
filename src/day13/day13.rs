use colored::Colorize;
use std::collections::HashSet;
use Advent_Of_Code_2021::{read_in, Point};
const TEST_MODE: bool = false;

fn main() {
    println!("\n{}", "AOC Day 13".bright_green().bold());
    let data = read_in::from(if TEST_MODE { "src/day13/testInput.txt" } else { "src/day13/input.txt" });
    let (points, instructions) = data.split_once("\n\r\n").unwrap();

    part1(points, instructions);
    part2(points, instructions);
}

fn part1(points: &str, instructions: &str) {
    let mut field: HashSet<Point> = HashSet::new();
    for point in points.lines() {
        field.insert(Point::from(point));
    }

    let mut instructions_iter = instructions.lines();
    let field = step(&field, instructions_iter.next().unwrap());


    let answer = field.len();
    println!("{}", "Part 1".green());
    if TEST_MODE { print_field(&field); }
    println!("- {}", answer.to_string().yellow());
}

fn part2(points: &str, instructions: &str) {
    let mut field: HashSet<Point> = HashSet::new();
    for point in points.lines() {
        field.insert(Point::from(point));
    }

    let mut instructions_iter = instructions.lines();
    loop {
        let instruction = instructions_iter.next();
        if let Some(instruct) = instruction {
            field = step(&field, instruct);
        } else {
            break;
        }
    }

    println!("{}", "Part 2".green());
    print_field(&field);
}

fn step(field: &HashSet<Point>, instruction: &str) -> HashSet<Point> {
    let operation = instruction.split(" ").last().unwrap();
    let mut new_field = HashSet::with_capacity(field.len());
    let (axis, pos) = operation.split_once("=").unwrap();
    let pos = pos.parse::<i16>().unwrap();

    for point_ref in field.iter() {
        let point = mirror_point(point_ref, axis, pos);
        new_field.insert(point);
    }

    new_field
}

fn mirror_point(point: &Point, axis: &str, pos: i16) -> Point {
    match axis {
        "y" => {
            if point.y < pos { return point.clone(); }
            Point::new(point.x, (pos * 2) - point.y)
        }
        "x" => {
            if point.x < pos { return point.clone(); }
            Point::new((pos * 2) - point.x, point.y)
        }
        &_ => panic!("Unexpected axis {}", axis)
    }
}

fn print_field(field: &HashSet<Point>) {
    let max_x = field.iter().map(|p| p.x).max().unwrap().abs() as usize;
    let max_y = field.iter().map(|p| p.y).max().unwrap().abs() as usize;
    for y in 0..=max_y {
        for x in 0..=max_x {
            if field.contains(&Point::new(x as i16, y as i16)) {
                print!("{}", "█".yellow());
            } else {
                print!("{}", ".".white());
            }
        }
        println!();
    }
}