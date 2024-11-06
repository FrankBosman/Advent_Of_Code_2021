use Advent_Of_Code_2021::read_in;
const TEST_MODE: bool = false;

fn main() {
    println!("AOC Day 1");
    let lines = read_in::parse_lines(if TEST_MODE { "src/day1/testInput.txt" } else { "src/day1/input.txt" });

    part1(&lines);
    part2(lines);
}

fn part1(lines: &Vec<i32>) {
    let mut increasing = 0;
    let mut previous = 100000;
    for line in lines {
        if line > &previous {
            increasing += 1;
        }
        previous = *line;
    }

    println!("Part 1");
    println!("- {increasing} measurements are larger than the previous");
}

fn part2(lines: Vec<i32>) {
    let mut increasing = 0;

    let mut sliding_window= [0; 3];
    let mut sliding_index = 0;
    let mut previous = 100000;

    for line in lines {
        sliding_window[sliding_index] = line;
        sliding_index = if sliding_index < sliding_window.len()-1 {sliding_index + 1} else {0};

        // Do not process the windows unless it has been filled;
        if !sliding_window.contains(&0) {
            let sum = sliding_window.iter().fold(0, |acc, x| acc + x);
            if sum > previous {
                increasing += 1;
            }
            previous = sum;
        }
    }

    println!("Part 2");
    println!("- {increasing} measurements are larger than the previous");
}