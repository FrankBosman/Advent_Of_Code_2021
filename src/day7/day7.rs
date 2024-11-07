use Advent_Of_Code_2021::read_in;
use colored::Colorize;
const TEST_MODE: bool = false;

fn main() {
    println!("\n{}", "AOC Day 7".bright_green().bold());
    let crabs = read_in::split_parse(if TEST_MODE { "src/day7/testInput.txt" } else { "src/day7/input.txt" }, ",");

    part1(&crabs);
    part2(&crabs);
}

fn part1(crabs: &Vec<i32>) {
    let answer = brute_force(crabs, false);
    println!("{}", "Part 1".green());
    println!("- {}", answer.to_string().yellow());
}


fn part2(crabs: &Vec<i32>) {
    let answer = brute_force(crabs, true);
    println!("{}", "Part 2".green());
    println!("- {}", answer.to_string().yellow());
}

fn brute_force(crabs: &Vec<i32>, part2: bool) -> i32 {
    // Find optimal path using brute force
    let lowest = crabs.iter().min().expect("empty crab list");
    let highest = crabs.iter().max().expect("empty crab list");
    let difference = highest - lowest;

    let mut scores = Vec::with_capacity(difference as usize);
    for i in 0..difference {
        if !part2 {
            scores.push(calculate_fuel(crabs, lowest + i));
        } else {
            scores.push(calculate_fuel2(crabs, lowest + i));
        }
    }

    let best = scores.iter().min().expect("empty score list").to_owned();
    best
}

fn calculate_fuel(crabs: &Vec<i32>, pos: i32) -> i32 {
    let mut fuel = 0;
    for crab in crabs {
        fuel += (*crab).abs_diff(pos) as i32;
    }
    fuel
}

fn calculate_fuel2(crabs: &Vec<i32>, pos: i32) -> i32 {
    let mut fuel = 0;
    for crab in crabs {
        // The fuel is scored with the triangular number
        let difference = (*crab - pos).abs();
        fuel += (difference * (difference + 1)) / 2;
    }

    fuel
}