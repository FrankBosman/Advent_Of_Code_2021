use std::collections::HashMap;
use Advent_Of_Code_2021::read_in;
use colored::Colorize;
use itertools::Itertools;

const TEST_MODE: bool = false;

fn main() {
    println!("\n{}", "AOC Day 14".bright_green().bold());
    let lines = read_in::lines(if TEST_MODE { "src/day14/testInput.txt" } else { "src/day14/input.txt" });

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut polymer = lines[0].clone();
    let mut map = HashMap::with_capacity(lines.len() - 2);
    for insertion in lines[2..].iter().clone() {
        let (key, value) = insertion.split_once(" -> ").unwrap();
        map.insert(key.trim(), value.trim().chars().nth(0).unwrap());
    }

    for _i in 0..10 {
        polymer = step(&polymer, &map);
    }

    let (max, min) = count_chars(&polymer);

    let answer = max - min;
    println!("{}","Part 1".green());
    println!("- {}", answer.to_string().yellow());
}

fn step(template: &String, map: &HashMap<&str, char>) -> String {
    let mut output = String::new();
    let array = template.chars().collect::<Vec<_>>();
    for window in array.windows(2) {
        output.push(window[0]);
        let key = format!("{}{}", window[0], window[1]);
        let result = map.get(key.as_str());
        if let Some(result) = result {
            output.push(*result);
        }
    }
    output.push(*array.last().unwrap());

    output
}

fn count_chars(template: &String) -> (usize, usize) {
    let mut counts = HashMap::<char, usize>::new();
    for char in template.chars() {
        match counts.get_mut(&char) {
            None => { counts.insert(char, 1); }
            Some(result) => { *result += 1; }
        }
    }

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();
    (*max, *min)
}

fn part2(lines: &Vec<String>) {
    println!("{}","Part 2".green());

    // Parse the polymer string
    let polymer_str = lines[0].clone();
    // - The polymer is a map containing all the pairs in the polymer and how often they occur
    let mut polymer = HashMap::with_capacity(polymer_str.len()-1);
    for (char1, char2) in polymer_str.chars().tuple_windows::<(_,_)>() {
        let pair = format!("{char1}{char2}");
        *polymer.entry(pair).or_insert(0) += 1;
    }

    // Set up the pairs map
    let mut map = HashMap::with_capacity(lines.len() - 2);
    for insertion in lines[2..].iter().clone() {
        let (key, value) = insertion.split_once(" -> ").unwrap();
        let value = value.trim();
        let pair1 = format!("{}{value}", key.chars().nth(0).unwrap());
        let pair2 = format!("{value}{}", key.chars().nth(1).unwrap());

        let pairs = vec![pair1, pair2];

        map.insert(key.trim().to_string(), pairs);
    }

    // Walk through the 40 steps
    for _i in 0..40 {
        polymer = step2(&polymer, &map);
    }

    // Get the most and least occurring chars
    let (max, min) = count_chars2(&polymer, &polymer_str);
    let answer = max - min;
    println!("- {}", answer.to_string().yellow());
}

fn step2(polymer: &HashMap<String, usize>, map: &HashMap<String, Vec<String>>) -> HashMap<String, usize> {
    let mut output: HashMap<String, usize> = HashMap::new();
    for (pair, count) in polymer {
        let pairs = map.get(pair).unwrap();
        for pair in pairs {
            *output.entry(pair.to_owned()).or_insert(0) += count;
        }
    }
    output
}

fn count_chars2(polymer: &HashMap<String, usize>, polymer_str: &str) -> (usize, usize) {
    let mut counts = HashMap::<char, usize>::new();

    // The last and first characters are only in the list once
    let first_char = polymer_str.chars().nth(0).unwrap();
    let last_char = polymer_str.chars().last().unwrap();
    *counts.entry(first_char).or_insert(0) += 1;
    *counts.entry(last_char).or_insert(0) += 1;

    for (pair, count) in polymer {
        for char in pair.chars() {
            *counts.entry(char).or_insert(0) += count;
        }
    }

    // All the other characters are in the list twice, so divide by 2
    let max = counts.values().max().unwrap() / 2;
    let min = counts.values().min().unwrap() / 2;
    (max, min)
}