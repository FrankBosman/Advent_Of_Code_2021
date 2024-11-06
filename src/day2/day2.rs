use Advent_Of_Code_2021::read_in;
const TEST_MODE: bool = false;

fn main() {
    println!("AOC Day 2");
    let lines = read_in::lines(if TEST_MODE { "src/day2/testInput.txt" } else { "src/day2/input.txt" });

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut horizontal = 0;
    let mut depth = 0;

    for line_in in lines {
        let parts = line_in.split(" ").collect::<Vec<&str>>();
        let amount = parts[1].parse::<i32>().expect(&format!("Could not parse number {line_in}"));
        match parts[0] {
            "forward" => horizontal += amount,
            "up" => depth -= amount,
            "down" => depth += amount,
            _ => panic!("Invalid operator {line_in}")
        }
    }

    let answer = horizontal * depth;
    println!("Part 1");
    println!("- {answer}");
}

fn part2(lines: &Vec<String>) {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line_in in lines {
        let parts = line_in.split(" ").collect::<Vec<&str>>();
        let amount = parts[1].parse::<i32>().expect(&format!("Could not parse number {line_in}"));
        match parts[0] {
            "forward" => {
                horizontal += amount;
                depth += aim * amount;
            },
            "up" => aim -= amount,
            "down" => aim += amount,
            _ => panic!("Invalid operator {line_in}")
        }
    }

    let answer = horizontal * depth;
    println!("Part 2");
    println!("- {answer}");
}