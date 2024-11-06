use colored::Colorize;
use Advent_Of_Code_2021::read_in;
const TEST_MODE: bool = false;

fn main() {
    println!("\n{}", "AOC Day 3".bright_green().bold());
    let lines = read_in::lines(if TEST_MODE { "src/day3/testInput.txt" } else { "src/day3/input.txt" });

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let columns = gen_columns(lines);

    let mut gamma = String::new();
    let mut epsilon = String::new();
    for column in columns {
        // let zeros = column.iter().filter(|num| num.eq(&0)).count();
        let ones = column.iter().sum::<usize>();
        if ones > column.len() / 2 {
            gamma.push_str("1");
            epsilon.push_str("0");
        } else {
            gamma.push_str("0");
            epsilon.push_str("1");
        }
    }

    let gamma_num =  i32::from_str_radix(&gamma, 2).expect("Failed to parse gamma number");
    let epsilon_num = i32::from_str_radix(&epsilon, 2).expect("Failed to parse epsilon number");


    let answer = gamma_num * epsilon_num;
    println!("{}", "Part 1".green());
    println!("- Gamma: {}, Epsilon: {}", gamma.to_string().yellow().italic(), epsilon.to_string().yellow().italic());
    println!("- {}", answer.to_string().yellow());
}

fn part2(lines: &Vec<String>) {
    let mut rows = lines.clone();

    let mut num = 0;
    loop {
        let most_common_char = most_common(&rows, num, false);
        rows = rows.iter().filter(|row| row.chars().nth(num) == Some(most_common_char)).cloned().collect();
        if rows.len() == 1 {
            break;
        }
        num += 1;
    }
    let oxygen = i32::from_str_radix(&rows[0], 2).expect("Failed to parse oxygen number");

    let mut rows = lines.clone();
    num = 0;
    loop {
        let most_common_char = most_common(&rows, num, true);
        rows = rows.iter().filter(|row| row.chars().nth(num) == Some(most_common_char)).cloned().collect();
        if rows.len() == 1 {
            break;
        }
        num += 1;
    }
    let co2 = i32::from_str_radix(&rows[0], 2).expect("Failed to parse co2 number");


    let answer = oxygen * co2;
    println!("{}", "Part 2".green());
    println!("- Oxygen: {}, CO2 Scrubber {}", oxygen.to_string().yellow().italic(), co2.to_string().yellow().italic());
    println!("- {}", answer.to_string().yellow());
}

fn most_common(rows: &Vec<String>, num: usize, least_common: bool) -> char {
    let mut zeros = 0;
    for line in rows {
        if line.chars().nth(num) == Some('0') {
            zeros += 1;
        }
    }
    if least_common {
        if zeros <= rows.len() / 2 {'0'} else {'1'}
    } else {
        if zeros > rows.len() / 2 {'0'} else {'1'}
    }
}

fn gen_columns(lines: &Vec<String>) -> Vec<Vec<usize>> {
    let mut columns: Vec<Vec<usize>> = Vec::with_capacity(lines[0].chars().count());
    for _i in 0..lines[0].chars().count() {
        columns.push(Vec::with_capacity(lines.len()));
    }

    for line in lines {
        for i in 0..line.chars().count() {
            columns[i].push(match line.chars().nth(i) {
                Some('1') => 1,
                Some('0') => 0,
                _ => panic!("Character should be 1 or 0")
            });
        }
    }
    columns
}