use std::cmp::PartialEq;
use Advent_Of_Code_2021::read_in;
use colored::Colorize;
const TEST_MODE: bool = false;

#[derive(PartialEq, Debug)]
enum Bracket {
    Normal,
    Block,
    Curly,
    Arrow,
}

impl Bracket {
    fn new(char: char) -> Bracket {
        match char {
            '(' => Bracket::Normal,
            '[' => Bracket::Block,
            '{' => Bracket::Curly,
            '<' => Bracket::Arrow,
            _ => panic!("Unknown opening bracket: {}", char),
        }
    }
    fn from_closing(char: char) -> Bracket {
        match char {
            ')' => Bracket::Normal,
            ']' => Bracket::Block,
            '}' => Bracket::Curly,
            '>' => Bracket::Arrow,
            _ => panic!("Unknown closing bracket: {}", char),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Bracket::Normal => 3,
            Bracket::Block => 57,
            Bracket::Curly => 1197,
            Bracket::Arrow => 25137,
        }
    }

    fn closing_score(&self) -> u64 {
        match self {
            Bracket::Normal => 1,
            Bracket::Block => 2,
            Bracket::Curly => 3,
            Bracket::Arrow => 4,
        }
    }

    fn is_opening(char: char) -> bool {
        "([{<".contains(char)
    }

}

fn main() {
    println!("\n{}", "AOC Day 10".bright_green().bold());
    let lines = read_in::lines(if TEST_MODE { "src/day10/testInput.txt" } else { "src/day10/input.txt" });

    part1(&lines);
}

fn part1(lines: &Vec<String>) {
    // Process the corrupted lines
    let mut illegal_chars: Vec<Bracket> = Vec::new();
    let mut remaining_stacks: Vec<Vec<Bracket>> = Vec::new();

    'lines: for line in lines {
        let mut stack: Vec<Bracket> = Vec::new();
        for char in line.chars() {
            if Bracket::is_opening(char) {
                stack.push(Bracket::new(char));
            } else {
                let last = stack.pop().unwrap();
                let closing = Bracket::from_closing(char);
                if !last.eq(&closing) {
                    illegal_chars.push(closing);
                    continue 'lines;
                }
            }
        }
        remaining_stacks.push(stack);
    }

    let score = illegal_chars.iter().fold(0, |score, bracket| score + bracket.score());

    println!("{}","Part 1".green());
    println!("- {}", score.to_string().yellow());

    part2(remaining_stacks);
}

fn part2(remaining_stacks: Vec<Vec<Bracket>>) {
    println!("{}","Part 2".green());

    let mut scores = remaining_stacks.iter().map(|stack| calc_score(stack)).collect::<Vec<u64>>();
    scores.sort_unstable();

    if TEST_MODE {
        println!("{:#?}", scores);
    }

    let answer = scores[scores.len() / 2];
    println!("- {}", answer.to_string().yellow());
}

fn calc_score(stack: &Vec<Bracket>) -> u64 {
    let mut score = 0;
    for bracket in stack.iter().rev() {
        score *= 5u64;
        score += bracket.closing_score();
    }
    score
}