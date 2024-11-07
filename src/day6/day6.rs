use Advent_Of_Code_2021::read_in;
use colored::Colorize;
const TEST_MODE: bool = false;

fn main() {
    println!("\n{}", "AOC Day 6".bright_green().bold());
    let input = read_in::split_parse(if TEST_MODE { "src/day6/testInput.txt" } else { "src/day6/input.txt" }, ",");

    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<i32>) {
    let mut population = input.clone();
    for _ in 0..80 {
        let mut new_count = 0;
        for i in 0..population.len() {
            if population[i] == 0 {
                population[i] = 6;
                new_count += 1;
            } else {
                population[i] -= 1;
            }
        }

        for _i in 0..new_count {
            population.push(8);
        }
    }

    let answer = population.len();
    println!("{}","Part 1".green());
    println!("- {}", answer.to_string().yellow());
}

fn part2(input: &Vec<i32>) {
    // from index 0 till 8 are the inner clocks
    let mut population = [0;9];
    // Transpose the Vector into the new population array
    for fish in input {
        if *fish < 0 || *fish > 8 {
            panic!("Fish inner clock should be between 0 and 8");
        }
        population[*fish as usize] += 1;
    }

    for _ in 0..256 {
        let new_count = population[0];
        for i in 1..=6 {
            population[i - 1] = population[i];
        }
        population[6] = population[7] + new_count;
        population[7] = population[8];
        population[8] = new_count;
    }

    let answer: i64 = population.iter().sum();
    println!("{}","Part 2".green());
    println!("- {}", answer.to_string().yellow());
}