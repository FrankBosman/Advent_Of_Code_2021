use colored::Colorize;
use std::fmt;
use std::fmt::Formatter;
use Advent_Of_Code_2021::read_in;

const TEST_MODE: bool = false;
const BOARD_SIZE: usize = 5;

struct Point {
    num: i32,
    checked: bool,
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num
    }
}
impl Clone for Point {
    fn clone(&self) -> Self {
        Point { num: self.num, checked: self.checked }
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", if self.checked { self.num.to_string().yellow() } else { self.num.to_string().white() })
    }
}

fn main() {
    println!("\n{}", "AOC Day 4".bright_green().bold());
    let lines = read_in::lines(if TEST_MODE { "src/day4/testInput.txt" } else { "src/day4/input.txt" });

    let (numbers, boards) = process_data(lines);
    part1(&numbers, &boards);
    part2(&numbers, &boards);
}

fn part1(numbers: &Vec<i32>, boards: &Vec<Vec<Point>>) {
    println!("{}", "Part 1".green());

    let mut boards = boards.clone();

    let mut winning_board = boards[0].clone();
    let mut winning_number = 0;

    'number_loop: for number in numbers {
        boards = add_number(&boards, number);
        for board in &boards {
            if has_won(board) {
                winning_number = *number;
                winning_board = board.clone();
                break 'number_loop;
            }
        }
    }

    print_board(&winning_board);

    let answer = calculate_score(&winning_board, winning_number);
    println!("- Score: {}", answer.to_string().yellow());
}

fn part2(numbers: &Vec<i32>, boards: &Vec<Vec<Point>>) {
    println!("{}", "Part 2".green());
    let mut boards = boards.clone();

    let mut losing_board = boards[0].clone();
    let mut losing_number = 0;
    let mut number_index = 0;

    while boards.len() > 0 {
        boards = add_number(&boards, &numbers[number_index]);
        losing_board = boards[0].clone();
        boards = boards.iter().filter(|board| !has_won(board)).cloned().collect();
        number_index += 1;
    }
    losing_number = numbers[(number_index - 1)];

    print_board(&losing_board);
    let answer = calculate_score(&losing_board, losing_number);
    println!("- Score: {}", answer.to_string().yellow());
}

fn process_data(lines: Vec<String>) -> (Vec<i32>, Vec<Vec<Point>>) {
    let numbers = lines[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let lines = lines[2..].to_vec();

    let mut boards: Vec<Vec<Point>> = Vec::new();

    let mut board = Vec::new();
    for line in lines {
        if line.is_empty() {
            boards.push(board.to_vec());
            board = Vec::new();
        } else {
            line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).for_each(|x| {
                board.push(Point { num: x, checked: false });
            });
        }
    }
    boards.push(board.clone());

    // for board in boards {
    //     print_board(&board);
    //     println!()
    // }

    (numbers, boards)
}

fn add_number(boards: &Vec<Vec<Point>>, number: &i32) -> Vec<Vec<Point>> {
    let mut boards = boards.clone();
    for i in 0..boards.len() {
        let mut board = boards[i].clone();
        for point in board.iter_mut() {
            if point.num.eq(number) {
                point.checked = true;
            }
        }
        boards[i] = board;
    }
    boards
}

fn has_won(board: &Vec<Point>) -> bool {
    // Check Rows
    'y_loop: for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            if !board[x + y * BOARD_SIZE].checked {
                continue 'y_loop;
            }
        }
        return true;
    }

    // Check Columns
    'x_loop: for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            if !board[x + y * BOARD_SIZE].checked {
                continue 'x_loop;
            }
        }
        return true;
    }

    false
}

fn calculate_score(board: &Vec<Point>, winning_number: i32) -> i32 {
    let summed = board.iter().filter(|x| !x.checked).fold(0, |acc, x| acc + x.num);
    summed * winning_number
}

fn print_board(board: &Vec<Point>) {
    let mut i = 0;
    for point in board {
        if i >= BOARD_SIZE {
            i = 0;
            println!();
        }
        print!("{}{} ", if point.num < 10 { " " } else { "" }, point);
        i += 1;
    }
    println!();
}